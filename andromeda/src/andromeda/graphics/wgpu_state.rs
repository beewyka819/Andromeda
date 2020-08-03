use winit::window::Window;

pub struct WgpuState {
    pub surface: wgpu::Surface,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub sc_desc: wgpu::SwapChainDescriptor,
    pub swap_chain: wgpu::SwapChain,
    vsync: bool,

    pub size: winit::dpi::PhysicalSize<u32>,
}

impl WgpuState {
    pub async fn new(window_handle: &Window) -> Self {
        let size = window_handle.inner_size();

        let (surface, adapter) = WgpuState::request_adapter(window_handle).await;

        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
            extensions: wgpu::Extensions {
                anisotropic_filtering: false,
            },
            limits: Default::default(),
        }).await;

        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        let swap_chain = device.create_swap_chain(&surface, &sc_desc);

        Self {
            surface,
            adapter,
            device,
            queue,
            sc_desc,
            swap_chain,

            size,

            vsync: true,
        }
    }

    async fn request_adapter(window_handle: &Window) -> (wgpu::Surface, wgpu::Adapter) {
        let surface = wgpu::Surface::create(window_handle);

        let adapter = wgpu::Adapter::request(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::Default,
                compatible_surface: Some(&surface),
            },
            wgpu::BackendBit::PRIMARY,
        ).await.expect("Failed to find graphics capable device!");

        (surface, adapter)
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        self.sc_desc.width = new_size.width;
        self.sc_desc.height = new_size.height;
        self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
    }

    pub fn set_vsync(&mut self, vsync: bool) {
        if self.vsync != vsync {
            self.vsync = vsync;
            if vsync {
                self.sc_desc.present_mode = wgpu::PresentMode::Fifo;
            } else {
                self.sc_desc.present_mode = wgpu::PresentMode::Immediate;
            }
            self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
        }
    }
}