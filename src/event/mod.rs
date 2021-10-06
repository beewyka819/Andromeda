pub use window_events::*;
pub use keyboard_events::*;
pub use mouse_events::*;

mod window_events;
mod keyboard_events;
mod mouse_events;

pub mod converters;

pub enum Event {
    WindowClose,
    WindowResize(WindowResizeEvent),
    WindowFocused(WindowFocusedEvent),
    WindowScaleFactorChanged(WindowScaleFactorChangedEvent),
    WindowMoved(WindowMovedEvent),
    KeyboardInput(KeyboardInputEvent),
    KeyTyped(KeyTypedEvent),
    MouseInput(MouseInputEvent),
    MouseMoved(MouseMovedEvent),
    MouseScrolled(MouseScrolledEvent),
    MouseEntered,
    MouseLeft,
    Handled,
}

impl std::fmt::Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Event::WindowClose => write!(f, "WindowClose"),
            Event::WindowResize(e) => write!(f, "{}", e),
            Event::WindowFocused(e) => write!(f, "{}", e),
            Event::WindowScaleFactorChanged(e) => write!(f, "{}", e),
            Event::WindowMoved(e) => write!(f, "{}", e),
            Event::KeyboardInput(e) => write!(f, "{}", e),
            Event::KeyTyped(e) => write!(f, "{}", e),
            Event::MouseInput(e) => write!(f, "{}", e),
            Event::MouseMoved(e) => write!(f, "{}", e),
            Event::MouseScrolled(e) => write!(f, "{}", e),
            Event::MouseEntered => write!(f, "MouseEntered"),
            Event::MouseLeft => write!(f, "MouseLeft"),
            Event::Handled => write!(f, "Handled"),
        }
    }
}

impl Event {
    pub fn handled(&self) -> bool {
        if let Event::Handled = self {
            return true;
        }
        false
    }

    pub fn set_handled(&mut self) {
        *self = Event::Handled;
    }
}

pub enum ElementState {
    Pressed { repeat_count: usize },
    Released,
}