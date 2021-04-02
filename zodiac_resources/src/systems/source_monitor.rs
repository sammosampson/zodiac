

use legion::*;
use legion::systems::*;
use zodiac_entities::*;
use zodiac_parsing::*;

use crate::monitoring::*; 
use crate::source_files::*; 

#[system(simple)]
#[read_component(SourceFile)]
#[write_component(SourceFileRemoval)]
#[write_component(SourceFileParsed)]
pub fn source_file_monitoring(
    command_buffer: &mut CommandBuffer,
    #[resource] monitor: &FileMonitor,
    #[resource] source_entity_lookup: &mut SourceEntityLookup) {
    
    match monitor.try_get_file_changed() {
        Ok(event) => match event {
            FileMonitorFileChange::Modify(path) => {
                let location = path.to_canonicalised_source_location().unwrap();
                modify_source(location, command_buffer, source_entity_lookup);
                
            },
            FileMonitorFileChange::Delete(path) => {
                let location = path.to_canonicalised_source_location().unwrap();
                delete_source(location, command_buffer, source_entity_lookup);
            },
            FileMonitorFileChange::Create(path) => {
                let location = path.to_canonicalised_source_location().unwrap();
                create_source(location, command_buffer, source_entity_lookup); 
            }
        },
        Err(err) => match err {
            _ => { } // TODO: handle filemonitor errors
        }
    }
}
