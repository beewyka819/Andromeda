#[allow(dead_code)]
#[derive(Debug)]
pub struct WgpuContext {
    instance: wgpu::Instance,
    surface: wgpu::Surface,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
    
    sc_desc: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,

    current_frame: Option<wgpu::SwapChainTexture>,
    command_pool: Vec<wgpu::CommandBuffer>,

    size: winit::dpi::PhysicalSize<u32>,
}

impl WgpuContext {
    pub async fn new(window_handle: &winit::window::Window) -> WgpuContext {
        let size = window_handle.inner_size();

        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);

        let (surface, adapter) = WgpuContext::request_adapter(&instance, &window_handle).await;
        
        let (device, queue) = WgpuContext::request_device(&adapter).await;

        let (sc_desc, mut swap_chain) = WgpuContext::create_swap_chain(&size, &device, &surface);

        let current_frame = swap_chain
            .get_current_frame()
            .expect("timeout getting frame")
            .output;

        WgpuContext {
            instance,
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

    async fn request_adapter(instance: &wgpu::Instance, window_handle: &winit::window::Window) -> (wgpu::Surface, wgpu::Adapter) {
        let surface = unsafe { instance.create_surface(window_handle) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::Default,
                compatible_surface: Some(&surface),
            })
            .await
            .expect("failed to find an appropriate adapter");
        
        (surface, adapter)
    }

    async fn request_device(adapter: &wgpu::Adapter) -> (wgpu::Device, wgpu::Queue) {
        adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                    shader_validation: true,
                },
                None,
            )
            .await
            .expect("failed to create device")
    }

    fn create_swap_chain(size: &winit::dpi::PhysicalSize<u32>, device: &wgpu::Device, surface: &wgpu::Surface) -> (wgpu::SwapChainDescriptor, wgpu::SwapChain) {
        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Mailbox,
        };

        let swap_chain = device.create_swap_chain(surface, &sc_desc);

        (sc_desc, swap_chain)
    }

    pub fn swap_buffers(&mut self) {
        let mut submittal_pool = Vec::new();
        std::mem::swap(&mut submittal_pool, &mut self.command_pool);
        
        self.queue.submit(submittal_pool);
        
        self.current_frame = None;
        self.current_frame = Some(self.swap_chain.get_current_frame().expect("timeout getting frame").output);
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        self.sc_desc.width = new_size.width;
        self.sc_desc.height = new_size.height;

        self.current_frame = None;
        self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
        self.current_frame = Some(self.swap_chain.get_current_frame().expect("timeout getting frame").output);
    }

    pub fn current_frame(&self) -> &wgpu::SwapChainTexture {
        self.current_frame.as_ref().unwrap()
    }

    pub fn device(&self) -> &wgpu::Device {
        &self.device
    }

    pub fn queue(&self) -> &wgpu::Queue {
        &self.queue
    }

    pub fn queue_mut(&mut self) -> &mut wgpu::Queue {
        &mut self.queue
    }

    pub fn sc_desc(&self) -> &wgpu::SwapChainDescriptor {
        &self.sc_desc
    }

    pub fn submit_command(&mut self, command_buffer: wgpu::CommandBuffer) {
        self.command_pool.push(command_buffer);
    }
}
