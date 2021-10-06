use anyhow::{Context, Result};
use winit::{
    event_loop::EventLoop,
    window,
};
use super::*;

pub struct WindowBuilder {
    title: String,
    width: u32,
    height: u32,
    vsync: bool,
}

impl WindowBuilder {
    pub fn new() -> WindowBuilder {
        WindowBuilder::default()
    }

    pub fn build(self) -> Result<(Window, EventLoop<()>)> {
        Window::new(
            self.title,
            self.width,
            self.height,
            self.vsync
        )
    }

    pub fn with_title(mut self, title: &str) -> WindowBuilder {
        self.title = title.to_string();
        self
    }

    pub fn with_width(mut self, width: u32) -> WindowBuilder {
        self.width = width;
        self
    }

    pub fn with_height(mut self, height: u32) -> WindowBuilder {
        self.height = height;
        self
    }

    pub fn with_vsync(mut self, vsync: bool) -> WindowBuilder {
        self.vsync = vsync;
        self
    }
}

impl Default for WindowBuilder {
    fn default() -> WindowBuilder {
        WindowBuilder {
            title: "Andromeda Engine".to_string(),
            width: 1280,
            height: 720,
            vsync: true,
        }
    }
}

pub struct Window {
    window_handle: winit::window::Window,
    title: String,
    width: u32,
    height: u32,
    vsync: bool,
}

impl Window {
    fn new(title: String, width: u32, height: u32, vsync: bool) -> Result<(Window, EventLoop<()>)> {
        ad_core_info!("Creating window {} ({}, {})", title, width, height);

        let event_loop = EventLoop::new();

        let window_handle = window::WindowBuilder::new()
            .with_title(title.clone())
            .with_inner_size(winit::dpi::PhysicalSize::new(width, height))
            .build(&event_loop).context("Unable to create winit window!")?;
        
        let monitor = window_handle.current_monitor().unwrap();

        let m_width = monitor.size().width;
        let m_height = monitor.size().height;

        let w_width = window_handle.outer_size().width;
        let w_height = window_handle.outer_size().height;

        window_handle
            .set_outer_position(winit::dpi::PhysicalPosition::new(
                (m_width - w_width) / 2,
                (m_height - w_height) / 2,
            ));

        let window = Window {
            window_handle,
            title,
            width,
            height,
            vsync,
        };

        Ok((window, event_loop))
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn vsync(&self) -> bool {
        self.vsync
    }

    pub fn window_handle(&self) -> &winit::window::Window {
        &self.window_handle
    }
}