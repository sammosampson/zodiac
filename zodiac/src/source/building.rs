use std::collections::HashMap;

use legion::Entity;
use legion::systems::CommandBuffer;
use crate::*;

pub type EntityMap = HashMap<u64, Entity>;

pub fn create_entity_map() -> EntityMap {
    EntityMap::default()
}

pub trait EntityCreator {
    fn get_or_create<'a, T: Send + Sync + 'static>(&mut self, id: u64, creation_func: impl FnOnce() -> T, maps: &mut SourceBuildMaps<'a>) -> Entity;
    fn add_child<'a>(&mut self, parent: Entity, child_id: &u64, maps: &mut SourceBuildMaps<'a>);
    fn remove_child<'a>(&mut self, child_id: &u64, maps: &mut SourceBuildMaps<'a>);
}

impl EntityCreator for CommandBuffer {
    fn get_or_create<'a, T: Send + Sync + 'static>(&mut self, id: u64, creation_func: impl FnOnce() -> T, maps: &mut SourceBuildMaps<'a>) -> Entity {
        match maps.entity_map.get(&id) {
            Some(entity) => *entity,
            None => {
                let relationship = Relationship::default();
                let entity = self.push((relationship, ComponentId::from(id), creation_func()));
                maps.entity_map.insert(id, entity);
                maps.relationship_map.insert(entity, relationship);
                entity
            }
        }
    }

    fn add_child<'a>(&mut self, parent: Entity, child_id: &u64, maps: &mut SourceBuildMaps<'a>) {
        let child = *maps.entity_map.get(child_id).unwrap();
        maps.relationship_map.add_entity(parent, child, self);    
    }

    fn remove_child<'a>(&mut self, child_id: &u64, maps: &mut SourceBuildMaps<'a>) {
        let child = maps.entity_map.remove(child_id).unwrap();
        self.add_component(child, Removed::default())
    }
}