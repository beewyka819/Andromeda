use andromeda::{
    Layer,
    EventHandleStatus,
    Event,
    Window,
    ImGuiLayer,
};

struct TestLayer;

impl Layer for TestLayer {
    fn on_attach(&mut self, _window: &mut Window) {
        
    }

    fn on_detach(&mut self) {
        
    }

    fn on_update(&mut self, window: &mut Window, renderer: &mut andromeda::graphics::Renderer) {
        renderer.render(window.context_mut());
    }

    fn on_event(&mut self, event: &Event<()>, window: &mut Window) -> EventHandleStatus {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if *window_id == window.window_handle().id() => {
                if let Some(event) = window.input_handler_mut().wrap_window_input(event) {
                    match event {
                        andromeda::input::InputEvent::KeyInput {
                            state,
                            virtual_keycode: Some(keycode),
                            ..
                        } => {
                            if let andromeda::input::ElementState::Pressed { .. } = state {
                                match keycode {
                                    andromeda::input::VirtualKeyCode::Escape => {
                                        return andromeda::EventHandleStatus::Terminate
                                    }
                                    _ => return andromeda::EventHandleStatus::Unhandled
                                }
                            }
                        }
                        _ => return andromeda::EventHandleStatus::Unhandled
                    }
                }
                return andromeda::EventHandleStatus::Unhandled
            },
            _ => EventHandleStatus::Unhandled,
        }
    }
}

fn main() {
    std::env::set_var("RUST_LOG", "andromeda=debug,gfx_memory=error");
    pretty_env_logger::init_timed();

    let test_layer = TestLayer {};
    let imgui_layer = ImGuiLayer::new();

    let layer_stack_descriptor = andromeda::LayerStackDescriptor {
        layers: Some(vec![Box::new(test_layer)]),
        overlays: Some(vec![Box::new(imgui_layer)]),
    };

    andromeda::start(layer_stack_descriptor, "Hello WGPU", 1280, 720);
}