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
    #[resource] file_entity_lookup: &mut SourceFileEntityLookup,
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
                    
                    let entity = command_buffer.push((SourceFile::default(),));

                    let location = SourceLocation::from(&path);
                    source_location_lookup.insert(entity, location);
                    file_entity_lookup.stash_entity(path, entity);                    
                }
            } else {
                //TODO: handle file read error
            }
        }
    } else {
            //TODO: handle file path not available
    }    
}