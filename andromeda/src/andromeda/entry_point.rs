use super::{
    window::Window,
    HazelApp,
};
use winit::{
    event::*,
    event_loop::ControlFlow,
};
use super::Application;

pub fn start_app<T: 'static>(app: T, title: String, width: u32, height: u32) where T: Application {
    let mut window = Window::new(title, width, height).unwrap();

    let mut app = HazelApp::new(app);

    app.init(&mut window);

    let handler = window.event_loop().take().unwrap();
    handler.run(move |event, _, control_flow| {
        let _ = (
            &app,
            &window,
        );
        *control_flow = ControlFlow::Poll;
        query_event(&event, control_flow, &mut window, &app);
        app.update();
    });
}
fn query_event<T>(event: &Event<()>, control_flow: &mut ControlFlow, window: &mut Window, app: &HazelApp<T>) where T: Application {
    match window.handle_event(event) {
        EventReturn::Terminate => *control_flow = ControlFlow::Exit,
        EventReturn::Nothing => {
            match app.handle_event(event, window) {
                EventReturn::Terminate => *control_flow = ControlFlow::Exit,
                _ => {}
            }
        },
        EventReturn::Handled => {}
    }
}

/*fn query_event<T>(event: &Event<()>, control_flow: &mut ControlFlow, window: &Window, app: &T, input_handler: &mut InputHandler) where T: Application {
    let mut event_status = EventReturn::Nothing;
    match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if *window_id == window.window_handle().id() => {
            event_status = handle_outside_events(event, window, app, input_handler);
            if let EventReturn::Terminate = event_status {
                *control_flow = ControlFlow::Exit;
            }
        }
        Event::MainEventsCleared if event_status == EventReturn::Nothing => {
            window.window_handle().request_redraw();
        },
        Event::RedrawRequested(_) => {
            app.update();
            app.render();
        },
        _ => {},
    }
}

fn handle_outside_events<T>(event: &WindowEvent, window: &Window, app: &T, input_handler: &mut InputHandler) -> EventReturn where T: Application {
    let window_status = window.handle_event(event);
    if window_status == EventReturn::Nothing {
        let input_event = input_handler.wrap_window_input(event);
        if let Some(event) = input_event {
            return app.input(&event)
        }
    }
    window_status
}*/

#[derive(PartialEq)]
pub enum EventReturn {
    Handled,
    Terminate,
    Nothing,
}