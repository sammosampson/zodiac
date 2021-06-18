
use log::{info};
use legion::*;
use legion::systems::*;
use crate::*;
use shrev::*;

#[system(for_each)]
#[filter(component::<Root>())]
#[filter(!component::<RootWindowResized>())]
pub fn initial_window_size_notification<TRenderer: Renderer + 'static> (
    entity: &Entity,
    command_buffer: &mut CommandBuffer,
    #[resource] event_channel: &mut EventChannel::<SystemEvent>,
    #[resource] renderer: &mut TRenderer) {
        info!("sending initial root window resize");
        event_channel.single_write(SystemEvent::Window(SystemWindowEventType::RootWindowResize(renderer.get_window_dimensions())));
        command_buffer.add_component(*entity, RootWindowResized::default());
}