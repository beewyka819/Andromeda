use winit::window::Window;

pub struct WgpuContext {
    surface: wgpu::Surface,
    adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub sc_desc: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,
    pub current_frame: Option<wgpu::SwapChainOutput>,
    pub command_pool: Vec<wgpu::CommandBuffer>,

    size: winit::dpi::PhysicalSize<u32>,
}

impl WgpuContext {
    pub async fn new(window_handle: &Window) -> Self {
        let size = window_handle.inner_size();

        let (surface, adapter) = WgpuContext::request_adapter(window_handle).await;

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
        let mut swap_chain = device.create_swap_chain(&surface, &sc_desc);

        let current_frame = swap_chain.get_next_texture().expect("Timeout getting frame");

        Self {
            surface,
            adapter,
            device,
            queue,
            sc_desc,
            swap_chain,
            current_frame: Some(current_frame),
            command_pool: vec![],

            size,
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

    pub fn swap_buffers(&mut self) {
        self.queue.submit(self.command_pool.as_slice());
        self.command_pool.clear();
        self.current_frame = None;
        self.current_frame = Some(self.swap_chain.get_next_texture().expect("Timeout getting frame"));
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        self.sc_desc.width = new_size.width;
        self.sc_desc.height = new_size.height;
        self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
    }

    pub fn current_frame(&self) -> &wgpu::SwapChainOutput {
        self.current_frame.as_ref().unwrap()
    }
}