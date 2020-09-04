use super::{
    LayerStack,
    Layer,
    Event,
    EventHandleStatus,
    graphics::Renderer,
};

pub struct WgpuApp {
    layer_stack: LayerStack,
}

impl WgpuApp {
    pub fn new() -> Self {
        Self {
            layer_stack: LayerStack::new(),
        }
    }

    pub fn handle_event(&mut self, event: &Event<()>, window: &mut super::Window) -> EventHandleStatus {
        self.layer_stack.handle_event(event, window)
    }

    pub fn update(&mut self, window: &mut super::Window, renderer: &mut Renderer) {
        self.layer_stack.update(window, renderer);
    }

    pub fn push_layer(&mut self, layer: Box<dyn Layer>, window: &mut super::Window) -> usize {
        let mut layer = layer;
        layer.on_attach(window);
        self.layer_stack.push_layer(layer)
    }

    pub fn push_overlay(&mut self, overlay: Box<dyn Layer>, window: &mut super::Window) -> usize {
        let mut overlay = overlay;
        overlay.on_attach(window);
        self.layer_stack.push_overlay(overlay)
    }
}

pub struct LayerStackDescriptor {
    pub layers: Option<Vec<Box<dyn Layer>>>,
    pub overlays: Option<Vec<Box<dyn Layer>>>,
}