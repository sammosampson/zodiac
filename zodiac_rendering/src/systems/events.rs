
use log::{debug};
use legion::*;
use zodiac_entities::*;
use shrev::*;

use crate::*;

#[system(for_each)]
#[filter(component::<SourceFileRoot>())]
#[filter(component::<SourceFileInitialRead>())]
pub fn initial_window_size_notification<TRenderer: Renderer + 'static> (
    _: &Entity,
    #[resource] event_channel: &mut EventChannel::<SystemEvent>,
    #[resource] renderer: &mut TRenderer) {
        debug!("sending initial root window resize");
        event_channel.single_write(SystemEvent::Window(SystemWindowEventType::RootWindowResize(renderer.get_window_dimensions())))
}