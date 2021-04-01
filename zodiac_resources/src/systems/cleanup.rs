
use legion::*;
use legion::systems::*;
use zodiac_entities::*;

#[system(for_each)]
#[filter(component::<SourceFileChange>())]
pub fn remove_source_file_change(command_buffer: &mut CommandBuffer, entity: &Entity) {
    command_buffer.remove_component::<SourceFileChange>(*entity);
}