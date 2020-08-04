struct TestLayer {}

impl andromeda::Layer for TestLayer {
    fn on_attach(&mut self, _window: &mut andromeda::Window) {

    }

    fn on_detach(&mut self) {

    }

    fn on_event(&mut self, event: &andromeda::Event<()>, window: &mut andromeda::Window) -> andromeda::EventReturn {
        match event {
            andromeda::Event::WindowEvent {
                ref event,
                window_id,
            } if *window_id == window.window_handle().id() => {
                if let Some(event) = window.input_handler_mut().wrap_window_input(event) {
                    match event {
                        andromeda::InputEvent::KeyInput {
                            state,
                            virtual_keycode: Some(keycode),
                            ..
                        } => {
                            if let andromeda::ElementState::Pressed { .. } = state {
                                match keycode {
                                    andromeda::VirtualKeyCode::Escape => {
                                        return andromeda::EventReturn::Terminate
                                    }
                                    _ => return andromeda::EventReturn::Nothing
                                }
                            }
                        }
                        _ => return andromeda::EventReturn::Nothing
                    }
                }
                return andromeda::EventReturn::Nothing
            }
            _ => return andromeda::EventReturn::Nothing
        }
    }

    fn on_update(&mut self, renderer: &mut andromeda::graphics::Renderer, window: &mut andromeda::Window) {
        renderer.render(window.wgpu_state_mut());
    }
}

fn main() {
    std::env::set_var("RUST_LOG", "sandbox=debug,andromeda=debug");
    env_logger::init();
    let test_layer = TestLayer {};
    let layer_stack_descriptor = andromeda::ApplicationLayerStackDescriptor {
        layers: Some(vec![Box::new(test_layer)]),
        overlays: Some(vec![Box::new(andromeda::ImGuiLayer::new())]),
    };
    andromeda::start_app(layer_stack_descriptor, String::from("SANDBOX"), 1280, 720);
}