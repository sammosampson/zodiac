use std::path::PathBuf;
use legion::systems::*;
use zodiac_entities::*;
use zodiac_parsing::*;

pub trait ToCanonicalisedSourceLocationConversion {
    fn to_canonicalised_source_location(&self) -> Result<SourceLocation, SourceLocationError>;
}

pub trait ToPathBufConversion {
    fn to_path_buf(&self) -> PathBuf;
}

impl ToCanonicalisedSourceLocationConversion for PathBuf {    
    fn to_canonicalised_source_location(&self) -> Result<SourceLocation, SourceLocationError> {
        if let Ok(path) = self.canonicalize() {
            Ok(SourceLocation { 
                location: match path.to_str() {
                    Some(path) => Some(path.to_owned()),
                    None => None
                }
            })
        } else {
            Err(SourceLocationError::DoesNotExist)
        }
    }
}


impl ToCanonicalisedSourceLocationConversion for &str {    
    fn to_canonicalised_source_location(&self) -> Result<SourceLocation, SourceLocationError> {
        PathBuf::from(self).to_canonicalised_source_location()
    }
}

impl ToCanonicalisedSourceLocationConversion for String {    
    fn to_canonicalised_source_location(&self) -> Result<SourceLocation, SourceLocationError> {
        PathBuf::from(self).to_canonicalised_source_location()
    }
}

impl ToPathBufConversion for &SourceLocation {    
    fn to_path_buf(&self) -> std::path::PathBuf {
        if let Some(location) = &self.location {
            return std::path::PathBuf::from(location)
        }
        std::path::PathBuf::new()
    }
}

pub fn delete_source(
    location: SourceLocation,
    command_buffer: &mut CommandBuffer,
    source_entity_lookup: &mut SourceEntityLookup) {

    if let Some(entity) = source_entity_lookup.get(&location) {
        command_buffer.add_component(*entity, SourceFileRemoval::default());
        command_buffer.remove_component::<SourceFileParsed>(*entity);
    }
}

pub fn modify_source(
    location: SourceLocation,
    command_buffer: &mut CommandBuffer,
    source_entity_lookup: &mut SourceEntityLookup) {
    
    if let Some(entity) = source_entity_lookup.get(&location) {
        command_buffer.add_component(*entity, SourceFileChange::default());
        command_buffer.remove_component::<SourceFileParsed>(*entity);
    }
}

pub fn create_source(
    location: SourceLocation,
    command_buffer: &mut CommandBuffer,
    source_entity_lookup: &mut SourceEntityLookup) {

    if let Some(entity) = source_entity_lookup.get(&location) {
        command_buffer.add_component(*entity, SourceFileCreation::default());
        command_buffer.remove_component::<SourceFileParsed>(*entity);
    } else {
        todo!() // error here the entity should exist in the lookup
    }
}

pub fn read_source(
    location: SourceLocation,
    command_buffer: &mut CommandBuffer,
    source_entity_lookup: &mut SourceEntityLookup,
    source_location_lookup: &mut SourceLocationLookup) {

    let entity = command_buffer.push((SourceFile::default(), SourceFileInitialRead::default()));
    source_location_lookup.insert(entity, location.to_owned());
    source_entity_lookup.insert(location, entity);
}