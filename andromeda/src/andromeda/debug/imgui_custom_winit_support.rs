use imgui::{self, BackendFlags, ConfigFlags, Context, ImString, Io, Key, Ui};
use std::cmp::Ordering;
use winit::dpi::{LogicalPosition, LogicalSize};
use winit::{
    error::ExternalError,
    event::{
        DeviceEvent, ElementState, Event, KeyboardInput, MouseButton, MouseScrollDelta, TouchPhase,
        VirtualKeyCode, WindowEvent,
    },
    window::{CursorIcon as MouseCursor, Window},
};
use super::super::EventReturn;

/// winit backend platform state
#[derive(Debug)]
pub struct WinitPlatform {
    hidpi_mode: ActiveHiDpiMode,
    hidpi_factor: f64,
    cursor_cache: Option<CursorSettings>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct CursorSettings {
    cursor: Option<imgui::MouseCursor>,
    draw_cursor: bool,
}

fn to_winit_cursor(cursor: imgui::MouseCursor) -> MouseCursor {
    match cursor {
        imgui::MouseCursor::Arrow => MouseCursor::Arrow,
        imgui::MouseCursor::TextInput => MouseCursor::Text,
        imgui::MouseCursor::ResizeAll => MouseCursor::Move,
        imgui::MouseCursor::ResizeNS => MouseCursor::NsResize,
        imgui::MouseCursor::ResizeEW => MouseCursor::EwResize,
        imgui::MouseCursor::ResizeNESW => MouseCursor::NeswResize,
        imgui::MouseCursor::ResizeNWSE => MouseCursor::NwseResize,
        imgui::MouseCursor::Hand => MouseCursor::Hand,
        imgui::MouseCursor::NotAllowed => MouseCursor::NotAllowed,
    }
}

impl CursorSettings {
    fn apply(&self, window: &Window) {
        match self.cursor {
            Some(mouse_cursor) if !self.draw_cursor => {
                window.set_cursor_visible(true);
                window.set_cursor_icon(to_winit_cursor(mouse_cursor));
            }
            _ => window.set_cursor_visible(false),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum ActiveHiDpiMode {
    Default,
    Rounded,
    Locked,
}

/// DPI factor handling mode.
///
/// Applications that use imgui-rs might want to customize the used DPI factor and not use
/// directly the value coming from winit.
///
/// **Note: if you use a mode other than default and the DPI factor is adjusted, winit and imgui-rs
/// will use different logical coordinates, so be careful if you pass around logical size or
/// position values.**
#[derive(Copy, Clone, Debug, PartialEq)]
#[allow(dead_code)]
pub enum HiDpiMode {
    /// The DPI factor from winit is used directly without adjustment
    Default,
    /// The DPI factor from winit is rounded to an integer value.
    ///
    /// This prevents the user interface from becoming blurry with non-integer scaling.
    Rounded,
    /// The DPI factor from winit is ignored, and the included value is used instead.
    ///
    /// This is useful if you want to force some DPI factor (e.g. 1.0) and not care about the value
    /// coming from winit.
    Locked(f64),
}

impl HiDpiMode {
    fn apply(&self, hidpi_factor: f64) -> (ActiveHiDpiMode, f64) {
        match *self {
            HiDpiMode::Default => (ActiveHiDpiMode::Default, hidpi_factor),
            HiDpiMode::Rounded => (ActiveHiDpiMode::Rounded, hidpi_factor.round()),
            HiDpiMode::Locked(value) => (ActiveHiDpiMode::Locked, value),
        }
    }
}

impl WinitPlatform {
    /// Initializes a winit platform instance and configures imgui.
    ///
    /// This function configures imgui-rs in the following ways:
    ///
    /// * backend flags are updated
    /// * keys are configured
    /// * platform name is set
    pub fn init(imgui: &mut Context) -> WinitPlatform {
        let io = imgui.io_mut();
        io.backend_flags.insert(BackendFlags::HAS_MOUSE_CURSORS);
        io.backend_flags.insert(BackendFlags::HAS_SET_MOUSE_POS);
        io[Key::Tab] = VirtualKeyCode::Tab as _;
        io[Key::LeftArrow] = VirtualKeyCode::Left as _;
        io[Key::RightArrow] = VirtualKeyCode::Right as _;
        io[Key::UpArrow] = VirtualKeyCode::Up as _;
        io[Key::DownArrow] = VirtualKeyCode::Down as _;
        io[Key::PageUp] = VirtualKeyCode::PageUp as _;
        io[Key::PageDown] = VirtualKeyCode::PageDown as _;
        io[Key::Home] = VirtualKeyCode::Home as _;
        io[Key::End] = VirtualKeyCode::End as _;
        io[Key::Insert] = VirtualKeyCode::Insert as _;
        io[Key::Delete] = VirtualKeyCode::Delete as _;
        io[Key::Backspace] = VirtualKeyCode::Back as _;
        io[Key::Space] = VirtualKeyCode::Space as _;
        io[Key::Enter] = VirtualKeyCode::Return as _;
        io[Key::Escape] = VirtualKeyCode::Escape as _;
        io[Key::KeyPadEnter] = VirtualKeyCode::NumpadEnter as _;
        io[Key::A] = VirtualKeyCode::A as _;
        io[Key::C] = VirtualKeyCode::C as _;
        io[Key::V] = VirtualKeyCode::V as _;
        io[Key::X] = VirtualKeyCode::X as _;
        io[Key::Y] = VirtualKeyCode::Y as _;
        io[Key::Z] = VirtualKeyCode::Z as _;
        imgui.set_platform_name(Some(ImString::from(format!(
            "imgui-winit-support {}",
            env!("CARGO_PKG_VERSION")
        ))));
        WinitPlatform {
            hidpi_mode: ActiveHiDpiMode::Default,
            hidpi_factor: 1.0,
            cursor_cache: None,
        }
    }
    /// Attaches the platform instance to a winit window.
    ///
    /// This function configures imgui-rs in the following ways:
    ///
    /// * framebuffer scale (= DPI factor) is set
    /// * display size is set
    pub fn attach_window(&mut self, io: &mut Io, window: &Window, hidpi_mode: HiDpiMode) {
        let (hidpi_mode, hidpi_factor) = hidpi_mode.apply(window.scale_factor());
        self.hidpi_mode = hidpi_mode;
        self.hidpi_factor = hidpi_factor;
        io.display_framebuffer_scale = [hidpi_factor as f32, hidpi_factor as f32];
        let logical_size = window.inner_size().to_logical(hidpi_factor);
        let logical_size = self.scale_size_from_winit(window, logical_size);
        io.display_size = [logical_size.width as f32, logical_size.height as f32];
    }
    /// Returns the current DPI factor.
    ///
    /// The value might not be the same as the winit DPI factor (depends on the used DPI mode)
    pub fn hidpi_factor(&self) -> f64 {
        self.hidpi_factor
    }
    /// Scales a logical size coming from winit using the current DPI mode.
    ///
    /// This utility function is useful if you are using a DPI mode other than default, and want
    /// your application to use the same logical coordinates as imgui-rs.
    pub fn scale_size_from_winit(
        &self,
        window: &Window,
        logical_size: LogicalSize<f64>,
    ) -> LogicalSize<f64> {
        match self.hidpi_mode {
            ActiveHiDpiMode::Default => logical_size,
            _ => logical_size
                .to_physical::<f64>(window.scale_factor())
                .to_logical(self.hidpi_factor),
        }
    }
    /// Scales a logical position coming from winit using the current DPI mode.
    ///
    /// This utility function is useful if you are using a DPI mode other than default, and want
    /// your application to use the same logical coordinates as imgui-rs.
    pub fn scale_pos_from_winit(
        &self,
        window: &Window,
        logical_pos: LogicalPosition<f64>,
    ) -> LogicalPosition<f64> {
        match self.hidpi_mode {
            ActiveHiDpiMode::Default => logical_pos,
            _ => logical_pos
                .to_physical::<f64>(window.scale_factor())
                .to_logical(self.hidpi_factor),
        }
    }
    /// Scales a logical position for winit using the current DPI mode.
    ///
    /// This utility function is useful if you are using a DPI mode other than default, and want
    /// your application to use the same logical coordinates as imgui-rs.
    pub fn scale_pos_for_winit(
        &self,
        window: &Window,
        logical_pos: LogicalPosition<f64>,
    ) -> LogicalPosition<f64> {
        match self.hidpi_mode {
            ActiveHiDpiMode::Default => logical_pos,
            _ => logical_pos
                .to_physical::<f64>(self.hidpi_factor)
                .to_logical(window.scale_factor()),
        }
    }
    /// Handles a winit event.
    ///
    /// This function performs the following actions (depends on the event):
    ///
    /// * window size / dpi factor changes are applied
    /// * keyboard state is updated
    /// * mouse state is updated
    pub fn handle_event<T>(&mut self, io: &mut Io, window: &Window, event: &Event<T>) -> EventReturn {
        match *event {
            Event::WindowEvent {
                window_id,
                ref event,
            } if window_id == window.id() => {
                // We need to track modifiers separately because some system like macOS, will
                // not reliably send modifier states during certain events like ScreenCapture.
                // Gotta let the people show off their pretty imgui widgets!
                if let WindowEvent::ModifiersChanged(modifiers) = event {
                    io.key_shift = modifiers.shift();
                    io.key_ctrl = modifiers.ctrl();
                    io.key_alt = modifiers.alt();
                    io.key_super = modifiers.logo();
                    return EventReturn::Handled;
                }

                self.handle_window_event(io, window, event)
            }
            // Track key release events outside our window. If we don't do this,
            // we might never see the release event if some other window gets focus.
            Event::DeviceEvent {
                event:
                    DeviceEvent::Key(KeyboardInput {
                        state: ElementState::Released,
                        virtual_keycode: Some(key),
                        ..
                    }),
                ..
            } => {
                io.keys_down[key as usize] = false;
                EventReturn::Handled
            }
            _ => EventReturn::Nothing,
        }
    }
    fn handle_window_event(&mut self, io: &mut Io, window: &Window, event: &WindowEvent) -> EventReturn {
        match *event {
            WindowEvent::Resized(physical_size) => {
                let logical_size = physical_size.to_logical(window.scale_factor());
                let logical_size = self.scale_size_from_winit(window, logical_size);
                io.display_size = [logical_size.width as f32, logical_size.height as f32];
                EventReturn::Nothing
            }
            WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                let hidpi_factor = match self.hidpi_mode {
                    ActiveHiDpiMode::Default => scale_factor,
                    ActiveHiDpiMode::Rounded => scale_factor.round(),
                    _ => return EventReturn::Nothing,
                };
                // Mouse position needs to be changed while we still have both the old and the new
                // values
                if io.mouse_pos[0].is_finite() && io.mouse_pos[1].is_finite() {
                    io.mouse_pos = [
                        io.mouse_pos[0] * (hidpi_factor / self.hidpi_factor) as f32,
                        io.mouse_pos[1] * (hidpi_factor / self.hidpi_factor) as f32,
                    ];
                }
                self.hidpi_factor = hidpi_factor;
                io.display_framebuffer_scale = [hidpi_factor as f32, hidpi_factor as f32];
                // Window size might change too if we are using DPI rounding
                let logical_size = window.inner_size().to_logical(scale_factor);
                let logical_size = self.scale_size_from_winit(window, logical_size);
                io.display_size = [logical_size.width as f32, logical_size.height as f32];
                EventReturn::Nothing
            }
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        virtual_keycode: Some(key),
                        state,
                        ..
                    },
                ..
            } => {
                let pressed = state == ElementState::Pressed;
                io.keys_down[key as usize] = pressed;

                // This is a bit redundant here, but we'll leave it in. The OS occasionally
                // fails to send modifiers keys, but it doesn't seem to send false-positives,
                // so double checking isn't terrible in case some system *doesn't* send
                // device events sometimes.
                match key {
                    VirtualKeyCode::LShift | VirtualKeyCode::RShift => {
                        io.key_shift = pressed;
                        return EventReturn::Handled;
                    },
                    VirtualKeyCode::LControl | VirtualKeyCode::RControl => {
                        io.key_ctrl = pressed;
                        return EventReturn::Handled;
                    },
                    VirtualKeyCode::LAlt | VirtualKeyCode::RAlt => {
                        io.key_alt = pressed;
                        return EventReturn::Handled;
                    },
                    VirtualKeyCode::LWin | VirtualKeyCode::RWin => {
                        io.key_super = pressed;
                        return EventReturn::Handled;
                    },
                    _ => EventReturn::Nothing,
                }
            }
            WindowEvent::ReceivedCharacter(ch) => {
                // Exclude the backspace key ('\u{7f}'). Otherwise we will insert this char and then
                // delete it.
                if ch != '\u{7f}' {
                    io.add_input_character(ch);
                    return EventReturn::Handled;
                }
                EventReturn::Nothing
            }
            WindowEvent::CursorMoved { position, .. } => {
                let position = position.to_logical(window.scale_factor());
                let position = self.scale_pos_from_winit(window, position);
                io.mouse_pos = [position.x as f32, position.y as f32];
                EventReturn::Handled
            }
            WindowEvent::MouseWheel {
                delta,
                phase: TouchPhase::Moved,
                ..
            } => match delta {
                MouseScrollDelta::LineDelta(h, v) => {
                    io.mouse_wheel_h = h;
                    io.mouse_wheel = v;
                    EventReturn::Handled
                }
                MouseScrollDelta::PixelDelta(pos) => {
                    let mut handled = EventReturn::Nothing;
                    match pos.x.partial_cmp(&0.0) {
                        Some(Ordering::Greater) => {
                            io.mouse_wheel_h += 1.0;
                            handled = EventReturn::Handled;
                        },
                        Some(Ordering::Less) => {
                            io.mouse_wheel_h -= 1.0;
                            handled = EventReturn::Handled;
                        },
                        _ => (),
                    }
                    match pos.y.partial_cmp(&0.0) {
                        Some(Ordering::Greater) => {
                            io.mouse_wheel += 1.0;
                            handled = EventReturn::Handled;
                        },
                        Some(Ordering::Less) => {
                            io.mouse_wheel -= 1.0;
                            handled = EventReturn::Handled;
                        },
                        _ => (),
                    }
                    handled
                }
            },
            WindowEvent::MouseInput { state, button, .. } => {
                let pressed = state == ElementState::Pressed;
                match button {
                    MouseButton::Left => {
                        io.mouse_down[0] = pressed;
                        return EventReturn::Handled;
                    },
                    MouseButton::Right => {
                        io.mouse_down[1] = pressed;
                        return EventReturn::Handled;
                    },
                    MouseButton::Middle => {
                        io.mouse_down[2] = pressed;
                        return EventReturn::Handled;
                    },
                    MouseButton::Other(idx @ 0..=4) => {
                        io.mouse_down[idx as usize] = pressed;
                        return EventReturn::Handled;
                    },
                    _ => EventReturn::Nothing,
                }
            }
            _ => EventReturn::Nothing,
        }
    }
    /// Frame preparation callback.
    ///
    /// Call this before calling the imgui-rs context `frame` function.
    /// This function performs the following actions:
    ///
    /// * mouse cursor is repositioned (if requested by imgui-rs)
    pub fn prepare_frame(&self, io: &mut Io, window: &Window) -> Result<(), ExternalError> {
        if io.want_set_mouse_pos {
            let logical_pos = self.scale_pos_for_winit(
                window,
                LogicalPosition::new(f64::from(io.mouse_pos[0]), f64::from(io.mouse_pos[1])),
            );
            window.set_cursor_position(logical_pos)
        } else {
            Ok(())
        }
    }
    /// Render preparation callback.
    ///
    /// Call this before calling the imgui-rs UI `render_with`/`render` function.
    /// This function performs the following actions:
    ///
    /// * mouse cursor is changed and/or hidden (if requested by imgui-rs)
    pub fn prepare_render(&mut self, ui: &Ui, window: &Window) {
        let io = ui.io();
        if !io
            .config_flags
            .contains(ConfigFlags::NO_MOUSE_CURSOR_CHANGE)
        {
            let cursor = CursorSettings {
                cursor: ui.mouse_cursor(),
                draw_cursor: io.mouse_draw_cursor,
            };
            if self.cursor_cache != Some(cursor) {
                cursor.apply(window);
                self.cursor_cache = Some(cursor);
            }
        }
    }
}