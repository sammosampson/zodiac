use shrev::{EventChannel};
use crate::*;

pub enum SystemWindowEventType {
    RootWindowResize(Dimensions),
    CloseRequested
}

pub enum SystemEvent {
    Unused,
    Window(SystemWindowEventType),
    Device()
}



pub fn create_system_event_channel() -> EventChannel::<SystemEvent> {
    EventChannel::<SystemEvent>::new()
}

pub fn create_system_event_producer() -> SystemEventProducer{
    SystemEventProducer::new()
}

pub struct SystemEventProducer {
    events: Vec<SystemEvent>,
}

impl SystemEventProducer {
    pub fn new() -> Self {
        Self {
            events: Vec::with_capacity(128),
        }
    }

    pub fn push(&mut self, to_push: SystemEvent) {
        self.events.push(to_push.into());
    }

    pub fn drain_to(&mut self, channel: &mut EventChannel::<SystemEvent>) {
        channel.drain_vec_write(&mut self.events);
    }
}