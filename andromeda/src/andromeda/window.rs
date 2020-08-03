use winit::{
    event::*,
    event_loop::{EventLoop},
    window::{self, WindowBuilder},
};
use super::{EventReturn, input::InputHandler, graphics::WgpuState};
use log::info;

pub struct Window {
    title: String,
    event_loop: Option<EventLoop<()>>,
    window_handle: window::Window,
    vsync: bool,
    wgpu_state: WgpuState,
    input_handler: InputHandler,
}

impl Window {
    pub fn new(title: String, width: u32, height: u32) -> Result<Self, failure::Error> {
        info!("Creating window {}, ({}, {})", &title, width, height);
        let (window_handle, event_loop) = Self::init(&title, width, height)?;

        use futures::executor::block_on;
        let wgpu_state = block_on(WgpuState::new(&window_handle));

        Ok(Window {
            title,
            event_loop: Some(event_loop),
            window_handle,
            vsync: true,
            wgpu_state,
            input_handler: InputHandler::new(),
        })
    }

    fn init(title: &String, width: u32, height: u32) -> Result<(window::Window, EventLoop<()>), failure::Error> {
        let event_loop = EventLoop::new();
        let window_handle = WindowBuilder::new()
            .with_title(title)
            .with_inner_size(winit::dpi::LogicalSize::new(width, height))
            .build(&event_loop)?;
        let m_width = window_handle.current_monitor().size().width;
        let m_height = window_handle.current_monitor().size().height;
        let w_width = window_handle.outer_size().width;
        let w_height = window_handle.outer_size().height;
        window_handle.set_outer_position(winit::dpi::LogicalPosition::new((m_width - w_width) / 2, (m_height - w_height) / 2));
        Ok((window_handle, event_loop))
    }

    pub fn handle_event(&self, event: &Event<()>) -> EventReturn {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if *window_id == self.window_handle.id() => match event {
                WindowEvent::CloseRequested => EventReturn::Terminate,
                _ => EventReturn::Nothing,
            }
            _ => EventReturn::Nothing,
        }
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn event_loop_mut(&mut self) -> Option<EventLoop<()>> {
        self.event_loop.take()
    }

    pub fn window_handle(&self) -> &window::Window {
        &self.window_handle
    }

    pub fn input_handler_mut(&mut self) -> &mut InputHandler {
        &mut self.input_handler
    }

    pub fn vsync(&self) -> bool {
        self.vsync
    }

    pub fn set_vsync(&mut self, vsync: bool) {
        self.vsync = vsync;
        self.wgpu_state.set_vsync(vsync);
    }

    pub fn wgpu_state_mut(&mut self) -> &mut WgpuState {
        &mut self.wgpu_state
    }
}

