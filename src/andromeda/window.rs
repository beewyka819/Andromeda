use winit::{
    event::*,
    event_loop::EventLoop,
    window::WindowBuilder,
};
use log::info;
use super::{
    EventHandleStatus,
    input::InputHandler,
    graphics::WgpuContext,
};

pub struct Window {
    title: String,
    window_handle: winit::window::Window,
    input_handler: InputHandler,
    context: WgpuContext,
}

impl Window {
    pub fn new(title: &str, width: u32, height: u32) -> Result<(Window, EventLoop<()>), failure::Error> {
        info!("Creating window: {} ({}, {})", title, width, height);
        let event_loop = EventLoop::new();
        
        let window_handle = WindowBuilder::new()
            .with_title(title)
            .with_inner_size(winit::dpi::PhysicalSize::new(width, height))
            .build(&event_loop)?;
        
        let monitor = window_handle.current_monitor();

        let m_width = monitor.size().width;
        let m_height = monitor.size().height;

        let w_width = window_handle.outer_size().width;
        let w_height = window_handle.outer_size().height;

        window_handle.set_outer_position(winit::dpi::PhysicalPosition::new((m_width - w_width) / 2, (m_height - w_height) / 2));
        
        let context = futures::executor::block_on(WgpuContext::new(&window_handle));
        
        let window = Window {
            title: String::from(title),
            window_handle,
            input_handler: InputHandler::new(),
            context,
        };

        Ok((window, event_loop))
    }

    pub fn handle_event(&mut self, event: &Event<()>) -> EventHandleStatus {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if *window_id == self.window_handle.id() => match event {
                WindowEvent::CloseRequested => EventHandleStatus::Terminate,
                WindowEvent::Resized(physical_size) => {
                    self.context.resize(*physical_size);
                    // Window resize should be non-blocking
                    EventHandleStatus::Unhandled
                },
                WindowEvent::ScaleFactorChanged { new_inner_size, ..} => {
                    self.context.resize(**new_inner_size);
                    // Window resize should be non-blocking
                    EventHandleStatus::Unhandled
                },
                _ => EventHandleStatus::Unhandled,
            }
            _ => EventHandleStatus::Unhandled,
        }
    }

    #[allow(dead_code)]
    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn window_handle(&self) -> &winit::window::Window {
        &self.window_handle
    }

    pub fn input_handler(&self) -> &InputHandler {
        &self.input_handler
    }

    pub fn input_handler_mut(&mut self) -> &mut InputHandler {
        &mut self.input_handler
    }

    pub fn context(&self) -> &WgpuContext {
        &self.context
    }

    pub fn context_mut(&mut self) -> &mut WgpuContext {
        &mut self.context
    }
}