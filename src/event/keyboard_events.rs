use super::{
    ElementState,
    super::key_codes::KeyCode,
};

pub struct KeyboardInputEvent {
    pub scan_code: u32,
    pub key_code: Option<KeyCode>,
    pub state: ElementState,
}

impl std::fmt::Display for KeyboardInputEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.state {
            ElementState::Pressed { repeat_count } => {
                if let Some(key_code) = self.key_code {
                    write!(f, "KeyboardPressed: {} ({:?}) ({} repeats)", self.scan_code, key_code, repeat_count)
                } else {
                    write!(f, "KeyboardPressed: {} ({} repeats)", self.scan_code, repeat_count)
                }
            },
            ElementState::Released => {
                if let Some(key_code) = self.key_code {
                    write!(f, "KeyboardReleased: {} ({:?})", self.scan_code, key_code)
                } else {
                    write!(f, "KeyboardReleased: {}", self.scan_code)
                }
            },
        }
    }
}

pub struct KeyTypedEvent {
    pub char: char,
}

impl std::fmt::Display for KeyTypedEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "KeyTyped: {}", self.char)
    }
}