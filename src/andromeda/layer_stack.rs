use super::{
    layer::Layer,
    Event,
    EventHandleStatus,
    graphics::Renderer,
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
    pub fn new() -> LayerStack {
        LayerStack {
            layers: Vec::new(),
            layer_insert: 0,
            id_counter: 0,
        }
    }

    pub fn push_layer(&mut self, layer: Box<dyn Layer>) -> usize {
        let id = self.id_counter;
        let layer_id = LayerID {
            layer,
            id,
        };
        self.id_counter += 1;
        debug!("pushed layer {}", layer_id.id);
        self.layers.insert(self.layer_insert, layer_id);
        self.layer_insert += 1;

        id
    }

    pub fn push_overlay(&mut self, overlay: Box<dyn Layer>) -> usize {
        let id = self.id_counter;
        let overlay_id = LayerID {
            layer: overlay,
            id,
        };
        self.id_counter += 1;
        debug!("pushed overlay {}", overlay_id.id);
        self.layers.push(overlay_id);

        id
    }

    #[allow(dead_code)]
    pub fn pop_layer(&mut self, layer_id: usize) -> Option<Box<dyn Layer>> {
        let pos = self.layers.iter().take(self.layer_insert).position(|x| x.id == layer_id);
        if let Some(index) = pos {
            let mut elem = self.layers.remove(index);
            self.layer_insert -= 1;
            elem.layer.on_detach();
            debug!("popped overlay {} at index: {}", elem, index);
            return Some(elem.layer)
        }
        None
    }

    #[allow(dead_code)]
    pub fn pop_overlay(&mut self, overlay_id: usize) -> Option<Box<dyn Layer>> {
        let pos = self.layers.iter().skip(self.layer_insert).position(|x| x.id == overlay_id);
        if let Some(index) = pos {
            let mut elem = self.layers.remove(index);
            elem.layer.on_detach();
            debug!("popped overlay {} at index: {}", elem, index);
            return Some(elem.layer)
        }
        None
    }

    pub fn handle_event(&mut self, event: &Event<()>, window: &mut super::Window) -> EventHandleStatus {
        let mut handled = EventHandleStatus::Unhandled;
        for layer_id in self.layers.iter_mut().rev() {
            handled = layer_id.layer.on_event(event, window);
            if handled != EventHandleStatus::Unhandled {
                break;
            }
        }
        handled
    }

    pub fn update(&mut self, window: &mut super::Window, renderer: &mut Renderer) {
        self.layers
            .iter_mut()
            .for_each(|layer_id| {
                layer_id.layer.on_update(window, renderer);
        });
    }
}

impl Drop for LayerStack {
    fn drop(&mut self) {
        self.layers
            .iter_mut()
            .take(self.layer_insert)
            .for_each(|layer_id| {
                layer_id.layer.on_detach();
                debug!("popped layer {}", layer_id);
            });
        
        self.layers
            .iter_mut()
            .skip(self.layer_insert)
            .for_each(|overlay_id| {
                overlay_id.layer.on_detach();
                debug!("popped overlay {}", overlay_id);
            });
    }
}

#[allow(dead_code)]
pub struct LayerStackDescriptor {
    pub layers: Option<Vec<Box<dyn Layer>>>,
    pub overlays: Option<Vec<Box<dyn Layer>>>,
}
