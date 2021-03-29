
use std::collections::HashMap;
use legion::*;


#[derive(Clone, Eq, PartialEq)]
pub struct SourceLocation {
    location: Option<String>
}

impl From<&std::path::PathBuf> for SourceLocation {    
    fn from(path: &std::path::PathBuf) -> Self {
        Self {
            location: match path.to_str() {
                Some(path) => Some(path.to_owned()),
                None => None
            }
        }
    }
}

impl From<&str> for SourceLocation {    
    fn from(string: &str) -> Self {
        Self {
            location: Some(string.to_string())
        }
    }
}

impl From<&SourceLocation> for std::path::PathBuf {    
    fn from(location: &SourceLocation) -> Self {
        if let Some(location) = &location.location {
            return std::path::PathBuf::from(location)
        }
        std::path::PathBuf::new()
    }
}

#[derive(Debug)]
pub enum SourceReaderError {
    ErrorReadingSource,
    SourceNotFound
}

pub trait SourceReader {
    fn read_source_at_location(&self, location: &SourceLocation) -> Result<String, SourceReaderError>;
}

pub fn create_source_location_lookup() -> SourceLocationLookup {
    SourceLocationLookup::new()
}

pub type SourceLocationLookup = HashMap<Entity, SourceLocation>;