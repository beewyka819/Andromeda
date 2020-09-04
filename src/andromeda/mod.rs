mod window;
mod layer;
mod layer_stack;
pub mod input;
pub mod application;
pub mod graphics;
pub mod debug;

pub use application::*;
pub use window::*;
pub use winit::event::Event;
pub use layer::Layer;
pub use layer_stack::LayerStack;

use winit::{
    event_loop::ControlFlow,
};
use graphics::Renderer;

pub fn start(layer_stack_descriptor: LayerStackDescriptor, title: &str, width: u32, height: u32) {
    let (mut window, event_loop) = Window::new(title, width, height).expect("unable to build window");

    let mut app = WgpuApp::new();

    let layers = layer_stack_descriptor.layers;
    if let Some(layers) = layers {
        for layer in layers {
            app.push_layer(layer, &mut window);
        }
    }
    let overlays = layer_stack_descriptor.overlays;
    if let Some(overlays) = overlays {
        for overlay in overlays {
            app.push_overlay(overlay, &mut window);
        }
    }

    event_loop.run(move |event, _, control_flow| {
        let _ = (
            &app,
            &window,
        );
        *control_flow = ControlFlow::Poll;

        handle_event(&event, control_flow, &mut window, &mut app);
    });
}

fn handle_event(event: &Event<()>, control_flow: &mut ControlFlow, window: &mut Window, app: &mut WgpuApp) {
    match event {
        Event::MainEventsCleared => {
            window.window_handle().request_redraw();
        },
        Event::RedrawRequested(_) => {
            let mut renderer = Renderer {};

            app.update(window, &mut renderer);

            //renderer.render(window.context_mut());
            window.context_mut().swap_buffers();
        },
        _ => match window.handle_event(event) {
            EventHandleStatus::Terminate => *control_flow = ControlFlow::Exit,
            EventHandleStatus::Unhandled => match app.handle_event(event, window) {
                EventHandleStatus::Terminate => *control_flow = ControlFlow::Exit,
                _ => {}
            },
            EventHandleStatus::Handled => {},
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventHandleStatus {
    Handled,
    Unhandled,
    Terminate,
}
