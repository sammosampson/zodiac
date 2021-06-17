use glium::glutin::dpi::PhysicalSize;
use glium::glutin::*;
use zodiac::{Dimensions, SystemEvent};

pub fn create_system_event_loop() -> event_loop::EventLoop<()> {
    event_loop::EventLoop::<()>::new()
}
pub struct WrappedSystemEvent(SystemEvent);

impl WrappedSystemEvent {
    pub fn new(wrapped: SystemEvent) -> Self {
        Self(wrapped)
    }
}

impl Into<SystemEvent> for WrappedSystemEvent {
    fn into(self) -> SystemEvent {
        self.0
    }
}

pub struct WrappedDimensions(Dimensions);
impl From<PhysicalSize<u32>> for WrappedDimensions {
    fn from(size: PhysicalSize<u32>) -> Self {
        Self(Dimensions { width: size.width as u16, height: size.height as u16 })
    }
}

impl From<WrappedDimensions> for PhysicalSize<u32>  {
    fn from(dimensions: WrappedDimensions) -> Self {
        Self::new(dimensions.0.width as u32, dimensions.0.height as u32)
    }
}

impl From<Dimensions> for WrappedDimensions {
    fn from(size: Dimensions) -> Self {
        Self(size)
    }
}

impl From<&Dimensions> for WrappedDimensions {
    fn from(size: &Dimensions) -> Self {
        Self(*size)
    }
}

impl From<(u16, u16)> for WrappedDimensions {
    fn from(size: (u16, u16)) -> Self {
        Self(Dimensions::from(size))
    }
}

impl Into<Dimensions> for WrappedDimensions {
    fn into(self) -> Dimensions {
        self.0
    }
}
