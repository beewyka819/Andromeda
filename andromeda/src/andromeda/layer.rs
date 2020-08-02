use winit::event::Event;
use super::EventReturn;

pub trait Layer {
    fn on_attach(&self);

    fn on_detach(&self);

    fn on_update(&self);

    fn on_event(&self, event: &Event<()>, window: &mut super::Window) -> EventReturn;
}