use super::{
    LayerStack,
    Layer,
    Event,
    EventReturn,
};

pub trait Application {
    fn new() -> Self;

    fn init(&mut self, window: &mut super::Window) -> LayerQueue;
}

pub struct HazelApp<T> where T: Application {
    app: T,
    layer_stack: LayerStack,
}

impl<T> HazelApp<T> where T: Application {
    pub fn new(app: T) -> Self {
        Self {
            app,
            layer_stack: LayerStack::new(),
        }
    }

    pub fn init(&mut self, window: &mut super::Window) {
        let queue = self.app.init(window);
        if let Some(layers) = queue.layers {
            for layer in layers {
                self.push_layer(layer);
            }
        }
        if let Some(overlays) = queue.overlays {
            for overlay in overlays {
                self.push_overlay(overlay);
            }
        }
    }

    pub fn handle_event(&self, event: &Event<()>, window: &mut super::Window) -> EventReturn {
        self.layer_stack.handle_event(event, window)
    }

    pub fn update(&self) {
        self.layer_stack.update();
    }

    pub fn push_layer(&mut self, layer: Box<dyn Layer>) -> usize {
        self.layer_stack.push_layer(layer)
    }

    pub fn push_overlay(&mut self, overlay: Box<dyn Layer>) -> usize {
        self.layer_stack.push_overlay(overlay)
    }
}

pub struct LayerQueue {
    pub layers: Option<Vec<Box<dyn Layer>>>,
    pub overlays: Option<Vec<Box<dyn Layer>>>,
}