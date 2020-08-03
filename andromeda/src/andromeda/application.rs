use super::{
    LayerStack,
    Layer,
    Event,
    EventReturn,
    Window,
    graphics::Renderer,
};

pub struct HazelApp {
    layer_stack: LayerStack,
}

impl HazelApp {
    pub fn new() -> Self {
        Self {
            layer_stack: LayerStack::new(),
        }
    }

    pub fn handle_event(&mut self, event: &Event<()>, window: &mut super::Window) -> EventReturn {
        self.layer_stack.handle_event(event, window)
    }

    pub fn update(&mut self, renderer: &mut Renderer, window: &mut Window) {
        self.layer_stack.update(renderer, window);
    }

    pub fn push_layer(&mut self, layer: Box<dyn Layer>, window: &mut Window) -> usize {
        let mut layer = layer;
        layer.on_attach(window);
        self.layer_stack.push_layer(layer)
    }

    pub fn push_overlay(&mut self, overlay: Box<dyn Layer>, window: &mut Window) -> usize {
        let mut overlay = overlay;
        overlay.on_attach(window);
        self.layer_stack.push_overlay(overlay)
    }
}

pub struct ApplicationLayerStackDescriptor {
    pub layers: Option<Vec<Box<dyn Layer>>>,
    pub overlays: Option<Vec<Box<dyn Layer>>>,
}