use legion::*;
use legion::systems::*;
use zodiac_entities::*;
use crate::tokenization::abstract_syntax::*;
use crate::tokenization::world_building::*;
use crate::tokenization::source::*;
use crate::source_reading::*;

fn remove_children(
    command_buffer: &mut CommandBuffer,
    relationship_map: &RelationshipMap,
    entity: &Entity) {
    for child in relationship_map.get_children(entity) {
        println!("marking child as removed: {:?}", child);
        command_buffer.add_component(child, Removed::default());
        remove_children(command_buffer, relationship_map, &child);
    }
}

fn reset_relationship(command_buffer: &mut CommandBuffer, relationship_map: &mut RelationshipMap, entity: Entity) {
    let relationship = Relationship::default();
    command_buffer.add_component(entity, relationship);
    relationship_map.insert(entity, relationship);
}

fn remove_mapped(command_buffer: &mut CommandBuffer, entity: Entity) {
    command_buffer.remove_component::<Mapped>(entity);
}

#[system(for_each)]
#[filter(component::<SourceFile>())]
#[filter(!component::<SourceFileParsed>())]
pub fn remove_parsed_source_from_world (entity: &Entity, command_buffer: &mut CommandBuffer, #[resource] relationship_map: &mut RelationshipMap) {
    println!("Removing Source from world");    
    remove_children(command_buffer, relationship_map, entity);
    reset_relationship(command_buffer, relationship_map, *entity);
    remove_mapped(command_buffer, *entity);
}

#[system(for_each)]
#[filter(component::<SourceFile>())]
#[filter(!component::<SourceFileParsed>())]
pub fn source_file_parse<T:SourceReader + 'static> (
    entity: &Entity,
    command_buffer: &mut CommandBuffer,
    #[resource] source_file_location_lookup: &mut SourceLocationLookup,
    #[resource] source_file_reader: &mut T
) {
    
    // TODO : reduce nesting
    if let Some(location) = source_file_location_lookup.get(entity) {
        if let Ok(source_text) = source_file_reader.read_source_at_location(location) {
            println!("Source is now {:?} chars", source_text.len());
            if let Err(_) =
                AbstractSyntaxTokenizer
                    ::from_source(SourceTokenizer::from_string(&source_text))
                    .build_world(command_buffer, *entity) {
                        // TODO: source parse error
                } 
            command_buffer.add_component(*entity, SourceFileParsed::default());
        }
        else {
            // TODO: error source cannot be read
        }
    }
    else {
        // TODO: error source does not exist
    }
}