use std::collections::{ HashMap };
use legion::*;
use legion::systems::*;
use zodiac_entities::components::*;

pub struct RelationshipMap {
    map: HashMap<Entity, Relationship>
}

impl RelationshipMap {
    pub fn new() -> Self {
        Self {
            map: HashMap::<Entity, Relationship>::new()
        }
    }

    pub fn get_relationship_for_entity(&self, entity: &Entity) -> Option<&Relationship> {
        self.map.get(entity)
    }

    pub fn add_relationship_for_entity(&mut self, entity: Entity, relationship: Relationship) {
        self.map.insert(entity, relationship);
    }
}

#[system(for_each)]
#[filter(!component::<RelationshipMapped>())]
pub fn build_relationship_map(
    #[resource] relationship_map: &mut RelationshipMap,
    command_buffer: &mut CommandBuffer,
    entity: &Entity,
    relationship: &Relationship) {
        relationship_map.add_relationship_for_entity(*entity, *relationship);
        command_buffer.add_component(*entity, RelationshipMapped {});
    }