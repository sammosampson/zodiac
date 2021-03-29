use std::collections::*;
use std::path::PathBuf;
use legion::*;

pub fn create_source_file_entity_lookup() -> SourceFileEntityLookup {
    SourceFileEntityLookup {
        inner: HashMap::<PathBuf, Entity>::default()
    }   
}

pub struct SourceFileEntityLookup {
    inner: HashMap<PathBuf, Entity>,
}

impl SourceFileEntityLookup {
    pub fn stash_entity(&mut self, path: PathBuf, entity: Entity) {
        self.inner.insert(path, entity);
    }

    pub fn lookup_entity(&mut self, path: &PathBuf) -> Option<&Entity> {
        self.inner.get(path)
    }
    
    pub fn remove_entity(&mut self, path: &PathBuf) -> Option<Entity> {
        self.inner.remove(path)
    }
}
