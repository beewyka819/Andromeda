use super::super::{
    Layer,
    Event,
    Window,
    EventReturn,
    graphics::{WgpuState, Renderer},
};
use imgui::*;
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use winit::event::VirtualKeyCode;
use std::time::Instant;

pub struct ImGuiLayer {
    imgui: Context,
    platform: WinitPlatform,
    renderer: Option<imgui_wgpu::Renderer>,
    m_time: Option<Instant>,
    last_cursor: Option<MouseCursor>,
}

impl ImGuiLayer {
    pub fn new() -> Self {
        let mut imgui = Context::create();

        let platform = WinitPlatform::init(&mut imgui);

        Self {
            imgui,
            platform,
            renderer: None,
            m_time: None,
            last_cursor: None,
        }
    }
}

impl Layer for ImGuiLayer {
    fn on_attach(&mut self, window: &Window, wgpu_state: &mut WgpuState) {
        self.imgui.style_mut().use_dark_colors();

        let io = self.imgui.io_mut();

        io.backend_flags |= imgui::BackendFlags::HAS_MOUSE_CURSORS;
        io.backend_flags |= imgui::BackendFlags::HAS_SET_MOUSE_POS;

        io.key_map[imgui::Key::Tab as usize] = VirtualKeyCode::Tab as u32;
        io.key_map[imgui::Key::LeftArrow as usize] = VirtualKeyCode::Left as u32;
        io.key_map[imgui::Key::RightArrow as usize] = VirtualKeyCode::Right as u32;
        io.key_map[imgui::Key::UpArrow as usize] = VirtualKeyCode::Up as u32;
        io.key_map[imgui::Key::DownArrow as usize] = VirtualKeyCode::Down as u32;
        io.key_map[imgui::Key::PageUp as usize] = VirtualKeyCode::PageUp as u32;
        io.key_map[imgui::Key::PageDown as usize] = VirtualKeyCode::PageDown as u32;
        io.key_map[imgui::Key::Home as usize] = VirtualKeyCode::Home as u32;
        io.key_map[imgui::Key::End as usize] = VirtualKeyCode::End as u32;
        io.key_map[imgui::Key::Insert as usize] = VirtualKeyCode::Insert as u32;
        io.key_map[imgui::Key::Delete as usize] = VirtualKeyCode::Delete as u32;
        io.key_map[imgui::Key::Backspace as usize] = VirtualKeyCode::Back as u32;
        io.key_map[imgui::Key::Space as usize] = VirtualKeyCode::Space as u32;
        io.key_map[imgui::Key::Enter as usize] = VirtualKeyCode::Return as u32;
        io.key_map[imgui::Key::Escape as usize] = VirtualKeyCode::Escape as u32;
        io.key_map[imgui::Key::A as usize] = VirtualKeyCode::A as u32;
        io.key_map[imgui::Key::C as usize] = VirtualKeyCode::C as u32;
        io.key_map[imgui::Key::V as usize] = VirtualKeyCode::V as u32;
        io.key_map[imgui::Key::X as usize] = VirtualKeyCode::X as u32;
        io.key_map[imgui::Key::Y as usize] = VirtualKeyCode::Y as u32;
        io.key_map[imgui::Key::Z as usize] = VirtualKeyCode::Z as u32;

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

        let clear_color = wgpu::Color {
            r: 0.1,
            g: 0.2,
            b: 0.3,
            a: 1.0,
        };

        #[cfg(not(feature = "glsl-to-spirv"))]
        let renderer = imgui_wgpu::Renderer::new(
            &mut self.imgui,
            &wgpu_state.device,
            &mut wgpu_state.queue,
            wgpu_state.sc_desc.format,
            Some(clear_color),
        );

        #[cfg(feature = "glsl-to-spirv")]
        let renderer = imgui_wgpu::Renderer::new_glsl(
            &mut self.imgui,
            &wgpu_state.device,
            &mut wgpu_state.queue,
            wgpu_state.sc_desc.format,
            Some(clear_color),
        );

        self.renderer = Some(renderer);

        self.m_time = Some(Instant::now())
    }

    fn on_detach(&mut self) {

    }

    fn on_update(&mut self, renderer: &mut Renderer, window: &Window, wgpu_state: &mut WgpuState) {
        if let Some(frame) = renderer.frame().as_ref() {
            let imgui = &mut self.imgui;
            let io = imgui.io_mut();

            let win_size = window.window_handle().inner_size();
            io.display_size = [win_size.width as f32, win_size.height as f32];

            self.m_time = Some(io.update_delta_time(self.m_time.unwrap()));

            // ---Render ImGui---
            self.platform.prepare_frame(io, window.window_handle()).expect("Failed to prepare frame");
            let ui = imgui.frame();

            let mut show = true;
            ui.show_demo_window(&mut show);

            let mut encoder = wgpu_state.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("ImGui Encoder"),
            });

            if self.last_cursor != ui.mouse_cursor() {
                self.last_cursor = ui.mouse_cursor();
                self.platform.prepare_render(&ui, window.window_handle());
            }

            self.platform.prepare_render(&ui, window.window_handle());
            self.renderer
                .as_mut()
                .unwrap()
                .render(ui.render(), &wgpu_state.device, &mut encoder, &frame.view)
                .expect("ImGui rendering failed");

            renderer.add_command_buffer(encoder.finish());
            // ---Finish Render---
        }
    }

    fn on_event(&mut self, event: &Event<()>, window: &mut Window) -> EventReturn {
        self.platform.handle_event(self.imgui.io_mut(), window.window_handle(), event);
        EventReturn::Nothing
    }
}