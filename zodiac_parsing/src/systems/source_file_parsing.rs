use legion::*;
use legion::systems::*;
use zodiac_entities::*;
use crate::tokenization::abstract_syntax::*;
use crate::tokenization::world_building::*;
use crate::tokenization::source::*;
use crate::source_reading::*;

#[system(for_each)]
#[filter(component::<SourceFile>())]
#[filter(!component::<SourceFileParsed>())]
pub fn source_file_parse<T:SourceReader + 'static> (
    entity: &Entity,
    relationship: &Relationship,
    command_buffer: &mut CommandBuffer,
    #[resource] source_file_location_lookup: &mut SourceLocationLookup,
    #[resource] source_file_reader: &mut T
) {
    // TODO : reduce nesting
    if let Some(location) = source_file_location_lookup.get(entity) {
        if let Ok(source_text) = source_file_reader.read_source_at_location(location) {
            if let Err(_) =
                AbstractSyntaxTokenizer
                    ::from_source(SourceTokenizer::from_string(&source_text))
                    .build_world(command_buffer, *entity, *relationship) {
                        // TODO: source parse error
                } 
            
            command_buffer.add_component(*entity, SourceFileParsed {});
        }
        else {
            // TODO: error source cannot be read
        }
    }
    else {
        // TODO: error source does not exist
    }
}