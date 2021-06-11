
use legion::*;
use legion::systems::*;
use crate::components::*;

#[system(for_each)]
#[filter(component::<Rebuild>())]
pub fn remove_rebuild(command_buffer: &mut CommandBuffer, entity: &Entity) {
    command_buffer.remove_component::<Rebuild>(*entity);
}