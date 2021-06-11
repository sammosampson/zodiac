
use legion::*;
use legion::systems::*;
use crate::components::*;

#[system(for_each)]
#[filter(component::<Removed>())]
pub fn remove_entity(command_buffer: &mut CommandBuffer, entity: &Entity) {
    command_buffer.remove(*entity);
}
