use legion::{World, Entity};
use zodiac_entities::components::*;

pub struct SiblingRelationshipIterator<'a> {
    world: &'a mut World,
    entity: Entity
}

impl<'a> SiblingRelationshipIterator<'a> {
    pub fn new(world: &'a mut World, starting_entity: Entity) -> Self {
        Self {
            world,
            entity: starting_entity
        }
    }
}

impl<'a> Iterator for SiblingRelationshipIterator<'a> {
    type Item = Entity;
    fn next(&mut self) -> Option<Entity> {
        if let Some(entry) = self.world.entry(self.entity) {
            match entry.get_component::<Relationship>() {
                Ok(relationship) => {
                    if let Some(next_sibling) = relationship.next_sibling {
                        self.entity = next_sibling;
                        return Some(next_sibling);
                    }
                    return None;
                },
                Err(_) => return None
            }
            
        }
        None
    }
}