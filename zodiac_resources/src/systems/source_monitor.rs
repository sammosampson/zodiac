

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
    #[resource] file_entity_lookup: &mut SourceFileEntityLookup,
    #[resource] source_location_lookup: &mut SourceLocationLookup) {
    
    match monitor.try_get_file_changed() {
        Ok(event) => match event {
            FileMonitorFileChange::Modify(path) => {
                if let Some(entity) = file_entity_lookup.lookup_entity(&path) {
                    source_location_lookup.insert(*entity, SourceLocation::from(&path));
                    command_buffer.remove_component::<SourceFileParsed>(*entity);
                }
            },
            FileMonitorFileChange::Delete(path) => {
                if let Some(entity) = file_entity_lookup.remove_entity(&path) {
                    source_location_lookup.remove(&entity);
                    command_buffer.add_component(entity, SourceFileRemoval {});
                }
            },
            FileMonitorFileChange::Create(_) => {},
        },
        Err(err) => match err {
            _ => { } // TODO: handle filemonitor errors
        }
    }
}