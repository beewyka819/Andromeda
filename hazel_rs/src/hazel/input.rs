use winit::event::WindowEvent;
pub use winit::{
    dpi::PhysicalPosition,
    event::{
        ScanCode,
        VirtualKeyCode,
        ModifiersState,
        ElementState,
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
        state: KeyState,
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

pub enum KeyState {
    Pressed { repeat: bool },
    Released,
}

pub struct InputHandler {
    key_repeats: HashMap<ScanCode, bool>,
}

impl InputHandler {
    pub fn new() -> Self {
        Self {
            key_repeats: HashMap::new(),
        }
    }

    pub fn wrap_window_input(&mut self, event: &WindowEvent) -> Option<InputEvent> {
        match event {
            WindowEvent::KeyboardInput {
                input,
                ..
            } => match input.state {
                ElementState::Pressed => {
                    let mut repeat = false;
                    if let Some(is_repeating) =  self.key_repeats.get(&input.scancode) {
                        repeat = *is_repeating;
                    }
                    self.key_repeats.insert(input.scancode, true);
                    Some(InputEvent::KeyInput {
                        scancode: input.scancode,
                        state: KeyState::Pressed { repeat },
                        virtual_keycode: input.virtual_keycode,
                    })
                },
                ElementState::Released => {
                    self.key_repeats.insert(input.scancode, false);
                    Some(InputEvent::KeyInput {
                        scancode: input.scancode,
                        state: KeyState::Released,
                        virtual_keycode: input.virtual_keycode,
                    })
                },
            },
            WindowEvent::ModifiersChanged(modifiers_state) => {
                Some(InputEvent::ModifiersChanged(*modifiers_state))
            },
            WindowEvent::MouseInput {
                state,
                button,
                ..
            } => {
                Some(InputEvent::MouseInput {
                    state: *state,
                    button: *button,
                })
            },
            WindowEvent::CursorMoved {
                position,
                ..
            } => {
                Some(InputEvent::CursorMoved {
                    position: *position,
                })
            },
            WindowEvent::MouseWheel {
                delta,
                phase,
                ..
            } => {
                Some(InputEvent::MouseWheel {
                    delta: *delta,
                    phase: *phase,
                })
            },
            WindowEvent::TouchpadPressure {
                pressure,
                stage,
                ..
            } => {
                Some(InputEvent::TouchpadPressure {
                    pressure: *pressure,
                    stage: *stage,
                })
            },
            WindowEvent::Touch(touch) => {
                Some(InputEvent::Touch(*touch))
            },
            _ => None,
        }
    }
}



