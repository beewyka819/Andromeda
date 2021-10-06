use glam::IVec2;
pub struct WindowResizeEvent {
    pub width: u32,
    pub height: u32,
}

impl std::fmt::Display for WindowResizeEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "WindowResize: {}, {}", self.width, self.height)
    }
}

pub struct WindowScaleFactorChangedEvent {
    pub scale_factor: f64,
    pub new_width: u32,
    pub new_height: u32,
}

impl std::fmt::Display for WindowScaleFactorChangedEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "WindowScaleFactorChanged: (factor: {}, width: {}, height: {})", self.scale_factor, self.new_width, self.new_height)
    }
}

pub struct WindowFocusedEvent {
    pub focused: bool,
}

impl std::fmt::Display for WindowFocusedEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "WindowFocused: {}", self.focused)
    }
}

pub struct WindowMovedEvent {
    pub position: IVec2,
}

impl std::fmt::Display for WindowMovedEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "WindowMoved: {}, {}", self.position.x, self.position.y)
    }
}