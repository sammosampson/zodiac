
use legion::*;
use legion::systems::*;
use crate::components::*;

#[system(for_each)]
#[filter(component::<Rebuild>())]
pub fn remove_rebuild(command_buffer: &mut CommandBuffer, entity: &Entity) {
    command_buffer.remove_component::<Rebuild>(*entity);
}

#[system(for_each)]
#[filter(component::<LayoutChange>())]
pub fn remove_layout_change(command_buffer: &mut CommandBuffer, entity: &Entity) {
    command_buffer.remove_component::<LayoutChange>(*entity);
}

#[system(for_each)]
#[filter(component::<Resized>())]
pub fn remove_resized(command_buffer: &mut CommandBuffer, entity: &Entity) {
    command_buffer.remove_component::<Resized>(*entity);
    command_buffer.remove_component::<Mapped>(*entity);
}