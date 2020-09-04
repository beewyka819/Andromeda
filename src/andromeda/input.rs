use winit::event::WindowEvent;
pub use winit::{
    dpi::PhysicalPosition,
    event::{
        self,
        ScanCode,
        VirtualKeyCode,
        ModifiersState,
        MouseButton,
        MouseScrollDelta,
        TouchPhase,
        Touch,
    },
};
use std::collections::HashMap;

pub enum InputEvent {
    KeyInput {
        scancode: ScanCode,
        state: ElementState,
        virtual_keycode: Option<VirtualKeyCode>,
    },
    ModifiersChanged(ModifiersState),
    MouseInput {
        state: ElementState,
        button: MouseButton,
    },
    CursorMoved {
        position: PhysicalPosition<f64>,
    },
    MouseWheel {
        delta: MouseScrollDelta,
        phase: TouchPhase,
    },
    TouchpadPressure {
        pressure: f32,
        stage: i64,
    },
    Touch(Touch),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElementState {
    Pressed { repeat: bool },
    Released,
}

pub struct InputHandler {
    key_repeats: HashMap<ScanCode, bool>,
    mouse_repeats: HashMap<MouseButton, bool>,
}

impl InputHandler {
    pub fn new() -> InputHandler {
        InputHandler {
            key_repeats: HashMap::new(),
            mouse_repeats: HashMap::new(),
        }
    }

    pub fn wrap_window_input(&mut self, event: &WindowEvent) -> Option<InputEvent> {
        match *event {
            WindowEvent::KeyboardInput {
                input,
                ..
            } => match input.state {
                event::ElementState::Pressed => {
                    let mut repeat = false;
                    if let Some(is_repeating) = self.key_repeats.get(&input.scancode) {
                        repeat = *is_repeating;
                    }
                    self.key_repeats.insert(input.scancode, true);
                    Some(InputEvent::KeyInput {
                        scancode: input.scancode,
                        state: ElementState::Pressed { repeat },
                        virtual_keycode: input.virtual_keycode,
                    })
                },
                event::ElementState::Released => {
                    self.key_repeats.insert(input.scancode, false);
                    Some(InputEvent::KeyInput {
                        scancode: input.scancode,
                        state: ElementState::Released,
                        virtual_keycode: input.virtual_keycode,
                    })
                }
            },
            WindowEvent::ModifiersChanged(modifiers_state) => {
                Some(InputEvent::ModifiersChanged(modifiers_state))
            },
            WindowEvent::MouseInput {
                state,
                button,
                ..
            } => match state {
                event::ElementState::Pressed => {
                    let mut repeat = false;
                    if let Some(is_repeating) = self.mouse_repeats.get(&button) {
                        repeat = *is_repeating;
                    }
                    self.mouse_repeats.insert(button, true);
                    Some(InputEvent::MouseInput {
                        state: ElementState::Pressed { repeat },
                        button,
                    })
                },
                event::ElementState::Released => {
                    self.mouse_repeats.insert(button, false);
                    Some(InputEvent::MouseInput {
                        state: ElementState::Released,
                        button,
                    })
                },
            },
            WindowEvent::CursorMoved {
                position,
                ..
            } => {
                Some(InputEvent::CursorMoved {
                    position
                })
            },
            WindowEvent::MouseWheel {
                delta,
                phase,
                ..
            } => {
                Some(InputEvent::MouseWheel {
                    delta,
                    phase,
                })
            },
            WindowEvent::TouchpadPressure {
                pressure,
                stage,
                ..
            } => {
                Some(InputEvent::TouchpadPressure {
                    pressure,
                    stage,
                })
            },
            WindowEvent::Touch(touch) => {
                Some(InputEvent::Touch(touch))
            },
            _ => None
        }
    }
}
