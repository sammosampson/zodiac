
use legion::*;
use log::{info};
use legion::systems::*;
use legion::world::*;
use shrev::EventChannel;
use crate::layout::*;
use crate::components::*;
use crate::events::*;

pub fn request_root_layout(
    world: &mut SubWorld,
    command_buffer: &mut CommandBuffer,
    dimensions: &Dimensions) {
    
    info!("resize_screen: request_root_layout");

    for root in <Entity>::query()
        .filter(component::<Root>())
        .iter(world) {
            command_buffer.add_component(*root, LayoutRequest::from(dimensions));
    }
}

#[system(simple)]
#[read_component(Root)]
#[write_component(LayoutRequest)]
pub fn resize_screen(
    world: &mut SubWorld,
    command_buffer: &mut CommandBuffer,
    #[resource] event_channel: &mut EventChannel::<SystemEvent>,
    #[resource] event_readers: &mut LayoutEventReaderRegistry
) {
    for event in event_channel.read(&mut event_readers.resize_screen) {
        match event {
            SystemEvent::Window(SystemWindowEventType::RootWindowResize(dimensions)) => {
                request_root_layout(world, command_buffer, &dimensions);
            },
            _ => {}
        }
    }
}

#[system(for_each)]
#[filter(component::<Rebuild>())]
#[filter(!component::<LayoutRequest>())]
pub fn resize_after_rebuild(
    command_buffer: &mut CommandBuffer,
    entity: &Entity,
    current_layout_constraints: &CurrentLayoutConstraints
) {
    info!("source change {:?}", current_layout_constraints);
    command_buffer.add_component(*entity, LayoutRequest::from(current_layout_constraints));
}