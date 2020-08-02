use hazel_rs::Application;

struct TestLayer {}

impl hazel_rs::Layer for TestLayer {
    fn on_attach(&self) {

    }

    fn on_detach(&self) {

    }

    fn on_event(&self, event: &hazel_rs::Event<()>, window: &mut hazel_rs::Window) -> hazel_rs::EventReturn {
        match event {
            hazel_rs::Event::WindowEvent {
                ref event,
                window_id,
            } if *window_id == window.window_handle().id() => {
                if let Some(event) = window.input_handler().wrap_window_input(event) {
                    match event {
                        hazel_rs::InputEvent::KeyInput {
                            state,
                            virtual_keycode: Some(keycode),
                            ..
                        } => {
                            if let hazel_rs::KeyState::Pressed { repeat } = state {
                                match keycode {
                                    hazel_rs::VirtualKeyCode::Escape => {
                                        return hazel_rs::EventReturn::Terminate
                                    }
                                    hazel_rs::VirtualKeyCode::E if !repeat => {
                                        println!("E");
                                        return hazel_rs::EventReturn::Handled
                                    }
                                    _ => return hazel_rs::EventReturn::Nothing
                                }
                            }
                        }
                        _ => return hazel_rs::EventReturn::Nothing
                    }
                }
                return hazel_rs::EventReturn::Nothing
            }
            _ => return hazel_rs::EventReturn::Nothing
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

    fn init(&mut self, _window: &mut hazel_rs::Window) -> hazel_rs::LayerQueue {
        let test_layer = TestLayer {};
        hazel_rs::LayerQueue {
            layers: Some(vec![Box::new(test_layer)]),
            overlays: None,
        }
    }
}

fn main() {
    std::env::set_var("RUST_LOG", "sandbox=debug,hazel_rs=debug");
    env_logger::init();

    let sandbox = Sandbox::new();
    hazel_rs::start_app(sandbox, String::from("SANDBOX"), 1280, 720);
}