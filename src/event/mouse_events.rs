use super::{
    ElementState,
    super::mouse_codes::MouseCode
};
use glam::Vec2;

pub struct MouseInputEvent {
    pub button: MouseCode,
    pub state: ElementState,
}

impl std::fmt::Display for MouseInputEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.state {
            ElementState::Pressed { .. } => write!(f, "MousePressed: {:?}", self.button),
            ElementState::Released => write!(f, "MouseReleased: {:?}", self.button),
        }
    }
}

pub struct MouseMovedEvent {
    pub delta: Vec2,
}

impl std::fmt::Display for MouseMovedEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MouseMoved: {}, {}", self.delta.x, self.delta.y)
    }
}

pub struct MouseScrolledEvent {
    pub delta: Vec2,
}

impl std::fmt::Display for MouseScrolledEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MouseScrolled: {}, {}", self.delta.x, self.delta.y)
    }
}