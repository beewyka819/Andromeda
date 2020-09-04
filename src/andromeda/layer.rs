use winit::event::Event;
use super::{
    EventHandleStatus,
    Window,
    graphics::Renderer,
};

pub trait Layer {
    fn on_attach(&mut self, window: &mut Window);

    fn on_detach(&mut self);

    fn on_update(&mut self, window: &mut Window, renderer: &mut Renderer);

    fn on_event(&mut self, event: &Event<()>, window: &mut Window) -> EventHandleStatus;
}