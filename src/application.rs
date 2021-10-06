use super::{
    *,
    event::{Event, converters::convert_winit_event},
    window::*,
    LayerStack,
};
use winit::event_loop::{EventLoop, ControlFlow};
use anyhow::Result;

pub struct Application {
    window: Window,
    running: bool,
    layer_stack: LayerStack,
}

impl Drop for Application {
    fn drop(&mut self) {
        destroy_log();
    }
}

impl Application {
    pub fn new() -> Result<(Application, EventLoop<()>)> {
        let (window, event_loop) = WindowBuilder::new()
            .build()?;

        let layer_stack = LayerStack::new();

        Ok((Application {
            window,
            running: true,
            layer_stack,
        }, event_loop))
    }

    pub fn push_layer(&mut self, layer: &LayerRef) {
        self.layer_stack.push_layer(layer);
    }

    pub fn push_overlay(&mut self, overlay: &LayerRef) {
        self.layer_stack.push_overlay(overlay);
    }

    pub fn pop_layer(&mut self, layer: &LayerRef) {
        self.layer_stack.pop_layer(layer);
    }

    pub fn on_event(&mut self, mut event: Event) {
        match event {
            Event::Handled => {},
            Event::WindowClose => self.on_window_close(),
            _ => {
                self.layer_stack
                    .iter()
                    .rev()
                    .for_each(|layer| if !event.handled() {
                        (**layer).borrow_mut().on_event(&mut event)
                    });
            },
        }
    }

    pub fn on_update(&mut self) {
        self.layer_stack
            .iter()
            .for_each(|layer| (**layer).borrow_mut().on_update());
    }

    fn on_window_close(&mut self) {
        self.running = false;
    }

    pub fn run(mut app: Application, event_loop: EventLoop<()>) {
        event_loop.run(move |event, _, control_flow| {
            let _ = (
                &app,
            );
            *control_flow = ControlFlow::Poll;

            match event {
                winit::event::Event::MainEventsCleared => {
                    app.window.window_handle().request_redraw();
                },
                winit::event::Event::RedrawRequested(_) => {
                    app.on_update();
                },
                _ => app.on_event(convert_winit_event(&event)),
            }

            if !app.running() {
                *control_flow = ControlFlow::Exit;
            }
        });
    }

    pub fn running(&self) -> bool {
        self.running
    }

    pub fn window(&self) -> &Window {
        &self.window
    }
}