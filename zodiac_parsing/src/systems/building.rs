use legion::*;
use legion::systems::*;
use zodiac_entities::*;
use crate::source::*;
use crate::building::*;

#[system(for_each)]
#[filter(component::<Rebuild>())]
pub fn world_build<T:SourceReader + 'static> (
    entity: &Entity,
    source_implementation: &SourceImplementation,
    command_buffer: &mut CommandBuffer,
    #[resource] source_entity_lookup: &mut SourceEntityLookup,
    #[resource] source_location_lookup: &mut SourceLocationLookup,
    #[resource] source_tokens_lookup: &mut SourceTokensLookup,
    #[resource] source_reader: &mut T
) {

    let mut world_builder = create_world_builder(command_buffer, *entity);
    
    let build_resources_mut = &mut MutableBuildResources::new(&mut world_builder);

    let build_resources = &mut BuildResources {
        source_entity_lookup,
        source_location_lookup,
        source_tokens_lookup,
        source_reader
    };
    
    if let Err(build_error) = build(entity, source_implementation, build_resources, build_resources_mut) {
        command_buffer.add_component(build_error.entity, BuildErrorOccurrence::from(build_error))
    }
}
