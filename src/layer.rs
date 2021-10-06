use std::ops::Index;
use std::{cell::RefCell, rc::Rc};

use super::event::Event;

pub trait Layer {
    fn get_name(&self) -> String;

    /// Called when layer is pushed to the layer stack
    fn on_attach(&mut self) {}

    /// Called when layer is popped from the layer stack
    fn on_detach(&mut self) {}

    /// Called once per frame
    fn on_update(&mut self) {}

    /// Called on Hazel events
    fn on_event(&mut self, _event: &mut Event) {}
}

pub type LayerRef = Rc<RefCell<Box<dyn Layer>>>;

pub fn create_layer_ref(layer: Box<dyn Layer>) -> LayerRef {
    Rc::new(RefCell::new(layer))
}

pub struct LayerStack {
    layers: Vec<LayerRef>,
    layer_insert_index: usize,
}

impl Drop for LayerStack {
    fn drop(&mut self) {
        self.layers
            .iter()
            .for_each(|layer| (**layer).borrow_mut().on_detach());
    }
}

impl LayerStack {
    pub(crate) fn new() -> LayerStack {
        LayerStack {
            layers: vec![],
            layer_insert_index: 0,
        }
    }

    pub fn push_layer(&mut self, layer: &LayerRef) {
        layer.borrow_mut().on_attach();
        self.layers.insert(self.layer_insert_index, Rc::clone(layer));
        self.layer_insert_index += 1;
    }

    pub fn push_overlay(&mut self, overlay: &LayerRef) {
        overlay.borrow_mut().on_attach();
        self.layers.push(Rc::clone(overlay));
    }

    pub fn pop_layer(&mut self, layer: &LayerRef) {
        let pos = self.layers
            .iter()
            .enumerate()
            .position(|(index, x)| {
                index < self.layer_insert_index && &(**x.borrow()) as *const dyn Layer == &(**layer.borrow()) as *const dyn Layer
            });

        if let Some(pos) = pos {
            let layer = self.layers.index(pos);
            (**layer).borrow_mut().on_detach();

            self.layers.remove(pos);
            self.layer_insert_index -= 1;
        }
    }

    pub fn pop_overlay(&mut self, overlay: &LayerRef) {
        let pos = self.layers
            .iter()
            .enumerate()
            .position(|(index, x)| {
                index >= self.layer_insert_index && &(**x.borrow()) as *const dyn Layer == &(**overlay.borrow()) as *const dyn Layer
            });

        if let Some(pos) = pos {
            let overlay = self.layers.index(pos);
            (**overlay).borrow_mut().on_detach();

            self.layers.remove(pos);
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, LayerRef> {
        self.layers.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, LayerRef> {
        self.layers.iter_mut()
    }
}