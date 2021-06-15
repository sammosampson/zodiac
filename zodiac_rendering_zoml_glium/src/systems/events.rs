
use legion::*;
use glium::glutin::{event_loop::*, platform::run_return::EventLoopExtRunReturn};
use glium::glutin::event::{Event, WindowEvent, DeviceEvent};
use zodiac::*;
use shrev::*;

use crate::*;

#[system(simple)]
pub fn event_loop(
    #[resource] event_loop: &mut EventLoop<()>,
    #[resource] event_producer: &mut SystemEventProducer,
    #[resource] event_channel: &mut EventChannel::<SystemEvent>) {
    event_loop.run_return(|event, _, flow| {
        match event {
            Event::WindowEvent { window_id: _, event} => {
                event_producer.push(WrappedSystemEvent::from(event).into());
            }
            Event::DeviceEvent { device_id: _, event } => {
                event_producer.push(WrappedSystemEvent::from(event).into());
            }
            _ => {}
        }
        *flow = ControlFlow::Exit;
    });

    event_producer.drain_to(event_channel);
}

impl From<WindowEvent<'_>> for WrappedSystemEvent {
    fn from(event: WindowEvent) -> Self {
        match event {
            WindowEvent::Resized(size) => WrappedSystemEvent::from(SystemWindowEventType::RootWindowResize(WrappedDimensions::from(size).into())),
            WindowEvent::CloseRequested => WrappedSystemEvent::from(SystemWindowEventType::CloseRequested),
            _ => WrappedSystemEvent::new(SystemEvent::Unused)
        }
    }
}

impl From<SystemWindowEventType> for WrappedSystemEvent {
    fn from(event_type: SystemWindowEventType) -> Self {
        WrappedSystemEvent::new(SystemEvent::Window(event_type))
    }
}
 
impl From<DeviceEvent> for WrappedSystemEvent {
    fn from(event: DeviceEvent) -> Self {
        match event {
            _ => WrappedSystemEvent::new(SystemEvent::Unused)
        }
    }
}