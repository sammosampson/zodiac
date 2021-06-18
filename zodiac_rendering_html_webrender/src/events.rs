use glium::glutin::*;
use zodiac::SystemEvent;

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