
use legion::*;
use legion::systems::*;
use crate::*;

#[system(for_each)]
#[filter(!component::<Mapped>())]
pub fn mark_as_mapped(command_buffer: &mut CommandBuffer, entity: &Entity) {
    command_buffer.add_component(*entity, Mapped {});
}
