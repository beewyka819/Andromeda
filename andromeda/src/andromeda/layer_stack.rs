use super::{
    layer::Layer,
    Event,
    EventReturn,
    Window,
    graphics::{WgpuState, Renderer},
};
use log::debug;

struct LayerID {
    layer: Box<dyn Layer>,
    id: usize,
}

impl std::fmt::Display for LayerID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

pub struct LayerStack {
    layers: Vec<LayerID>,
    layer_insert: usize,
    id_counter: usize,
}

impl LayerStack {
    pub fn new() -> Self {
        Self {
            layers: Vec::new(),
            layer_insert: 0,
            id_counter: 0,
        }
    }

    pub fn push_layer(&mut self, layer: Box<dyn Layer>) -> usize {
        let layer_id = LayerID {
            layer,
            id: self.id_counter,
        };
        self.id_counter += 1;
        debug!("pushed layer {}", layer_id.id);
        self.layers.insert(self.layer_insert, layer_id);
        self.layer_insert += 1;
        
        self.layers.get(self.layer_insert - 1).unwrap().id
    }

    pub fn push_overlay(&mut self, overlay: Box<dyn Layer>) -> usize {
        let overlay_id = LayerID {
            layer: overlay,
            id: self.id_counter,
        };
        self.id_counter += 1;
        debug!("pushed overlay {}", overlay_id.id);
        self.layers.push(overlay_id);
        self.layers.get(self.layers.len() - 1).unwrap().id
    }

    pub fn pop_layer(&mut self, layer_id: usize) -> Option<Box<dyn Layer>> {
        let pos = self.layers.iter().take(self.layer_insert).position(|x| x.id == layer_id);
        if let Some(index) = pos {
            let elem = self.layers.remove(index);
            self.layer_insert -= 1;
            debug!("popped layer {} at index: {}", elem, index);
            return Some(elem.layer)
        }
        None
    }

    pub fn pop_overlay(&mut self, overlay_id: usize) -> Option<Box<dyn Layer>> {
        let pos = self.layers.iter().position(|x| x.id == overlay_id);
        if let Some(index) = pos {
            let elem = self.layers.remove(index);
            debug!("popped overlay {} at index: {}", elem, index);
            return Some(elem.layer)
        }
        None
    }

    pub fn handle_event(&mut self, event: &Event<()>, window: &mut super::Window) -> EventReturn {
        let mut handled = EventReturn::Nothing;
        for layer_id in self.layers.iter_mut().rev() {
            handled = layer_id.layer.on_event(event, window);
            if handled != EventReturn::Nothing {
                break;
            }
        }
        handled
    }

    pub fn update(&mut self, renderer: &mut Renderer, window: &Window, wgpu_state: &mut WgpuState) {
        for layer_id in &mut self.layers {
            layer_id.layer.on_update(renderer, window, wgpu_state);
        }
    }
}