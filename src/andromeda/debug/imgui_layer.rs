use super::super::{
    Layer,
    Event,
    Window,
    EventHandleStatus,
    graphics::Renderer,
};
use imgui::*;
use super::imgui_custom_winit_support::{HiDpiMode, WinitPlatform};
//use std::time::Instant;

pub struct ImGuiLayer {
    imgui: Context,
    platform: WinitPlatform,
    renderer: Option<imgui_wgpu::Renderer>,
    //m_time: Option<Instant>,
    last_cursor: Option<MouseCursor>,
}

impl ImGuiLayer {
    pub fn new() -> ImGuiLayer {
        let mut imgui = Context::create();

        let platform = WinitPlatform::init(&mut imgui);

        ImGuiLayer {
            imgui,
            platform,
            renderer: None,
            //m_time: None,
            last_cursor: None,
        }
    }

    fn initialize(&mut self, window: &mut Window) {
        self.imgui.style_mut().use_classic_colors();

        let io = self.imgui.io_mut();

        io.config_flags |= imgui::ConfigFlags::NAV_ENABLE_KEYBOARD;
        //io.config_flags |= imgui::ConfigFlags::NAV_ENABLE_GAMEPAD;

        io.backend_flags |= imgui::BackendFlags::HAS_MOUSE_CURSORS;
        io.backend_flags |= imgui::BackendFlags::HAS_SET_MOUSE_POS;

        self.platform.attach_window(
            self.imgui.io_mut(),
            window.window_handle(),
            HiDpiMode::Default
        );
        self.imgui.set_ini_filename(None);

        let hidpi_factor = self.platform.hidpi_factor();
        let font_size = (13.0 * hidpi_factor) as f32;
        self.imgui.io_mut().font_global_scale = (1.0 / hidpi_factor) as f32;

        self.imgui.fonts().add_font(&[
            FontSource::DefaultFontData {
                config: Some(FontConfig {
                    oversample_h: 1,
                    pixel_snap_h: true,
                    size_pixels: font_size,
                    ..FontConfig::default()
                }),
            },
        ]);

        let context = window.context_mut();

        let renderer = imgui_wgpu::Renderer::new(
            &mut self.imgui,
            context.device(),
            context.queue(),
            context.sc_desc().format,
        );

        self.renderer = Some(renderer);

        //self.m_time = Some(Instant::now());
    }

    fn render(&mut self, window: &mut Window) {
        let imgui = &mut self.imgui;
        let io = imgui.io_mut();

        let win_size = window.window_handle().inner_size();
        io.display_size = [win_size.width as f32, win_size.height as f32];

        //self.m_time = Some(io.update_delta_time(self.m_time.unwrap()));

        self.platform.prepare_frame(io, window.window_handle()).expect("Failed to prepare frame");
        let ui = imgui.frame();

        let mut show = true;
        ui.show_demo_window(&mut show);

        if self.last_cursor != ui.mouse_cursor() {
            self.last_cursor = ui.mouse_cursor();
            self.platform.prepare_render(&ui, window.window_handle());
        }
        
        let context = window.context_mut();

        let mut encoder = context.device().create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("ImGui Encoder"),
        });

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[
                wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: &context.current_frame().view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: true,
                    },
                }
            ],
            depth_stencil_attachment: None,
        });

        self.renderer
            .as_mut()
            .unwrap()
            .render(ui.render(), context.queue(), context.device(), &mut render_pass)
            .expect("ImGui rendering failed");
        
        drop(render_pass);

        context.submit_command(encoder.finish());
    }
}

impl Layer for ImGuiLayer {
    fn on_attach(&mut self, window: &mut Window) {
        self.initialize(window);
    }

    fn on_detach(&mut self) {
        
    }

    fn on_update(&mut self, window: &mut Window, _renderer: &mut Renderer) {
        self.render(window);
    }

    fn on_event(&mut self, event: &Event<()>, window: &mut Window) -> EventHandleStatus {
        self.platform.handle_event(self.imgui.io_mut(), window.window_handle(), event)
    }
}
