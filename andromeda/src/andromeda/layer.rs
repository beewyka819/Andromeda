use winit::event::Event;
use super::{
    EventReturn,
    Window,
    graphics::{WgpuState, Renderer},
};

pub trait Layer {
    fn on_attach(&mut self, window: &Window, wgpu_state: &mut WgpuState);

    fn on_detach(&mut self);

    fn on_update(&mut self, renderer: &mut Renderer, window: &Window, wgpu_state: &mut WgpuState);

    fn on_event(&mut self, event: &Event<()>, window: &mut super::Window) -> EventReturn;
}