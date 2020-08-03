use winit::event::Event;
use super::{
    EventReturn,
    Window,
    graphics::Renderer,
};

pub trait Layer {
    fn on_attach(&mut self, window: &mut Window);

    fn on_detach(&mut self);

    fn on_update(&mut self, renderer: &mut Renderer, window: &mut Window);

    fn on_event(&mut self, event: &Event<()>, window: &mut super::Window) -> EventReturn;
}