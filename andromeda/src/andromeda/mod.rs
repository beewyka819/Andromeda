mod application;
mod window;
mod layer;
mod layer_stack;
pub mod debug;
pub mod input;
pub mod renderer;

pub use application::*;
pub use window::*;
pub use winit::event::Event;
pub use layer::Layer;
pub use layer_stack::LayerStack;

use winit::{
    event_loop::ControlFlow,
};

use HazelApp;

pub fn start_app(layer_stack_descriptor: ApplicationLayerStackDescriptor, title: String, width: u32, height: u32) {
    let mut window = Window::new(title, width, height).unwrap();

    let mut app = HazelApp::new();

    let mut renderer = renderer::Renderer::new();

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

    let handler = window.event_loop_mut().take().unwrap();
    handler.run(move |event, _, control_flow| {
        let _ = (
            &app,
            &window,
            &renderer,
        );
        *control_flow = ControlFlow::Poll;
        query_event(&event, control_flow, &mut window, &mut app, &mut renderer);
    });
}
fn query_event(event: &Event<()>, control_flow: &mut ControlFlow, window: &mut Window, app: &mut HazelApp, renderer: &mut renderer::Renderer) {
    match event {
        Event::MainEventsCleared => {
            window.window_handle().request_redraw();
        },
        Event::RedrawRequested(_) => {
            app.update(renderer, window);
            window.swap_buffers();
        },
        _ => match window.handle_event(event) {
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
    
}

#[derive(PartialEq)]
pub enum EventReturn {
    Handled,
    Terminate,
    Nothing,
}