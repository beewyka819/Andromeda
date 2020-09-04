use super::WgpuContext;

pub struct Renderer {
    //commands: Vec<>,
}

impl Renderer {
    pub fn render(&self, context: &mut WgpuContext) {
        Renderer::draw_frame(context);
    }

    fn draw_frame(context: &mut WgpuContext) {
        let mut encoder = context.device().create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });
    
        let clear_color = wgpu::Color {
            r: 1.0,
            g: 0.0,
            b: 1.0,
            a: 1.0,
        };
        
        let render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[
                wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: &context.current_frame().view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(clear_color),
                        store: true,
                    },
                }
            ],
            depth_stencil_attachment: None,
        });
    
        drop(render_pass);
    
        context.submit_command(encoder.finish());
    }
}
