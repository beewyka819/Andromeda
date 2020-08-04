use super::WgpuState;

pub struct Renderer {
    render_queue: Vec<wgpu::CommandBuffer>,
    frame: Option<wgpu::SwapChainOutput>,
}

impl Renderer {
    pub fn new() -> Self {
        Renderer {
            render_queue: vec![],
            frame: None,
        }
    }

    pub fn start_frame(&mut self, wgpu_state: &mut WgpuState) {
        self.frame = Some(wgpu_state.swap_chain.get_next_texture()
            .expect("Timeout getting texture"));
    }

    pub fn render(&mut self, wgpu_state: &mut WgpuState) {
        if let Some(frame) = self.frame.as_ref() {
            let mut encoder = wgpu_state.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });
        
            let render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[
                    wgpu::RenderPassColorAttachmentDescriptor {
                        attachment: &frame.view,
                        resolve_target: None,
                        load_op: wgpu::LoadOp::Clear,
                        store_op: wgpu::StoreOp::Store,
                        clear_color: wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        },
                    },
                ],
                depth_stencil_attachment: None,
            });
        
            drop(render_pass);
        
            self.render_queue.push(encoder.finish());
        }
    }

    pub fn push_command_buffer(&mut self, command_buffer: wgpu::CommandBuffer) {
        self.render_queue.push(command_buffer);
    }

    pub fn submit_renders(&mut self, wgpu_state: &mut WgpuState) {
        if !self.render_queue.is_empty() {
            wgpu_state.queue.submit(self.render_queue.as_slice());
            self.render_queue.clear();
            self.frame = None;
        }
    }

    pub fn frame(&self) -> &Option<wgpu::SwapChainOutput> {
        &self.frame
    }
}