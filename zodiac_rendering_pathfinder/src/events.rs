use log::info;
use glutin::{dpi::PhysicalSize, event_loop::*};
use pathfinder_canvas::*;
use shrev::*;
use zodiac_entities::*;

pub fn create_pathfinder_event_reader_registry(event_channel: &mut EventChannel::<SystemEvent>) -> PathFinderEventReaderRegistry {
    PathFinderEventReaderRegistry::register(event_channel)
}
pub struct PathFinderEventReaderRegistry{
    pub (crate) render_primitives: ReaderId<SystemEvent>
}

impl PathFinderEventReaderRegistry {
    fn register(event_channel: &mut EventChannel::<SystemEvent>) -> Self {
        info!("registering path finder event readers");
        Self {
            render_primitives: event_channel.register_reader() 
        }
    }
}

#[derive(Copy, Clone, Debug)]
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

impl From<WrappedDimensions> for Vector2F  {
    fn from(dimensions: WrappedDimensions) -> Self {
        vec2f(dimensions.0.width as f32, dimensions.0.height as f32)
    }
}

impl From<WrappedDimensions> for Vector2I  {
    fn from(dimensions: WrappedDimensions) -> Self {
        vec2i(dimensions.0.width as i32, dimensions.0.height as i32)
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

pub fn create_system_event_loop() -> EventLoop<()> {
    EventLoop::new()
}
