use andromeda::Application;

struct TestLayer {}

impl andromeda::Layer for TestLayer {
    fn on_attach(&self) {

    }

    fn on_detach(&self) {

    }

    fn on_event(&self, event: &andromeda::Event<()>, window: &mut andromeda::Window) -> andromeda::EventReturn {
        match event {
            andromeda::Event::WindowEvent {
                ref event,
                window_id,
            } if *window_id == window.window_handle().id() => {
                if let Some(event) = window.input_handler().wrap_window_input(event) {
                    match event {
                        andromeda::InputEvent::KeyInput {
                            state,
                            virtual_keycode: Some(keycode),
                            ..
                        } => {
                            if let andromeda::KeyState::Pressed { repeat } = state {
                                match keycode {
                                    andromeda::VirtualKeyCode::Escape => {
                                        return andromeda::EventReturn::Terminate
                                    }
                                    andromeda::VirtualKeyCode::E if !repeat => {
                                        println!("E");
                                        return andromeda::EventReturn::Handled
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

    fn on_update(&self) {
        
    }
}

struct Sandbox {}

impl Application for Sandbox {
    fn new() -> Self {
        Sandbox {}
    }

    fn init(&mut self, _window: &mut andromeda::Window) -> andromeda::LayerQueue {
        let test_layer = TestLayer {};
        andromeda::LayerQueue {
            layers: Some(vec![Box::new(test_layer)]),
            overlays: None,
        }
    }
}

fn main() {
    std::env::set_var("RUST_LOG", "sandbox=debug,andromeda=debug");
    env_logger::init();

    let sandbox = Sandbox::new();
    andromeda::start_app(sandbox, String::from("SANDBOX"), 1280, 720);
}