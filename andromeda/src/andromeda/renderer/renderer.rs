use super::WgpuContext;

pub struct Renderer {
    render_queue: Vec<wgpu::CommandBuffer>,
}

impl Renderer {
    pub fn new() -> Self {
        Renderer {
            render_queue: vec![],
        }
    }

    pub fn render(&mut self, wgpu_context: &mut WgpuContext) {
        let mut encoder = wgpu_context.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });
    
        let render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[
                wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: &wgpu_context.current_frame().view,
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

    pub fn draw_elements(&mut self, command_buffer: wgpu::CommandBuffer, wgpu_context: &mut WgpuContext) {
        wgpu_context.command_pool.push(command_buffer);
    }
}