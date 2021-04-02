use walkdir::WalkDir;
use std::path::*;
use legion::*;
use legion::systems::*;
use legion::world::*;
use zodiac_entities::*;
use zodiac_parsing::*;

use crate::file_system::*; 
use crate::source_files::*; 

#[system(simple)]
#[write_component(SourceFile)]
pub fn recurisve_source_location_build(
    command_buffer: &mut CommandBuffer,
    world: &mut SubWorld,
    #[resource] file_paths: &FilePaths,
    #[resource] source_entity_lookup: &mut SourceEntityLookup,
    #[resource] source_location_lookup: &mut SourceLocationLookup) {
    
    let source_files: Vec::<&SourceFile> = <&SourceFile>::query().iter(world).collect();

    if source_files.len() > 0 {
        return;
    }
    // TODO: Make this less nested
    if let Ok(root_path) = file_paths.get_absolute_folder_path() {
        for entry in WalkDir::new(root_path) {
            if let Ok(entry) = entry { 
                let path = PathBuf::from(entry.path());
                if let Some(extension) = path.extension() {
                    if extension != "zod" {
                        continue;
                    }
                    
                    let location = path.to_canonicalised_source_location().unwrap();
                    read_source(location, command_buffer, source_entity_lookup, source_location_lookup);                   
                }
            } else {
                //TODO: handle file read error
            }
        }
    } else {
            //TODO: handle file path not available
    }    
}