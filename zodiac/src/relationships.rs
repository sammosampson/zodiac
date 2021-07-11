use std::collections::HashMap;
use legion::*;
use legion::systems::*;
use crate::*;

#[derive(Default, Debug)]
pub struct RelationshipMap(HashMap<Entity, Relationship>);

pub fn create_relationship_map() -> RelationshipMap {
    RelationshipMap::default()
}

impl RelationshipMap {
    fn get(&self, entity: &Entity) -> Option<&Relationship> {
        self.0.get(entity)
    }

    fn remove(&mut self, entity: &Entity) -> Option<Relationship> {
        self.0.remove(entity)
    }

    pub fn insert(&mut self, entity: Entity, relationship: Relationship) {
        self.0.insert(entity, relationship);
    }

    pub fn get_parent(&self, entity: &Entity) -> Option<Entity> {
        if let Some(relationship) = self.get(entity) {
            return relationship.parent;
        }
        None
    }

    pub fn get_previous_sibling(&self, entity: &Entity) -> Option<Entity> {
        if let Some(relationship) = self.get(entity) {
            return relationship.previous_sibling;
        }
        None
    }

    pub fn get_children(&self, parent: &Entity) -> ChildrenRelationshipIterator {
        ChildrenRelationshipIterator::new(self, *parent)
    }

    pub fn add_entity(&mut self, parent: Entity, entity: Entity, command_buffer: &mut CommandBuffer) {
        let mut parent_relationship = *self.get(&parent).unwrap();
        let mut child_relationship = *self.get(&entity).unwrap();

        if parent_relationship.last_child == None {
            parent_relationship.first_child = Some(entity);
        } else {
            let previous_child = parent_relationship.last_child.unwrap();
            child_relationship.previous_sibling = Some(previous_child);
            let mut previous_child_relationship = *self.get(&previous_child).unwrap();
            previous_child_relationship.next_sibling = Some(entity);
            command_buffer.add_component(previous_child, previous_child_relationship);
            self.insert(previous_child, previous_child_relationship);        
        }

        parent_relationship.last_child = Some(entity);
        command_buffer.add_component(parent, parent_relationship);
        self.insert(parent, parent_relationship);

        child_relationship.parent = Some(parent);
        command_buffer.add_component(entity, child_relationship);
        self.insert(entity, child_relationship);
    }
    
    pub fn remove_entity(&mut self, entity: Entity, command_buffer: &mut CommandBuffer) {
        let relationship = self.remove(&entity).unwrap();
        let parent = relationship.parent;
        
        if let Some(previous_child) = relationship.previous_sibling {
            let mut previous_child_relationship = *self.get(&previous_child).unwrap();
            previous_child_relationship.next_sibling = relationship.next_sibling;
            command_buffer.add_component(previous_child, previous_child_relationship);
            self.insert(previous_child, previous_child_relationship);
        }

        if let Some(next_child) = relationship.next_sibling {
            let mut next_child_relationship = *self.get(&next_child).unwrap();
            next_child_relationship.previous_sibling = relationship.previous_sibling;
            command_buffer.add_component(next_child, next_child_relationship);
            self.insert(next_child, next_child_relationship);
        }   

        if let Some(parent) = parent {
            let mut parent_relationship = *self.get(&parent).unwrap();
    
            if parent_relationship.first_child.unwrap() == entity {
                parent_relationship.first_child = relationship.next_sibling;
                command_buffer.add_component(parent, parent_relationship);
                self.insert(parent, parent_relationship);
            }
            
            if parent_relationship.last_child.unwrap() == entity {
                parent_relationship.last_child = relationship.previous_sibling;
                command_buffer.add_component(parent, parent_relationship);
                self.insert(parent, parent_relationship);
            }
        }
    }
}

pub struct ChildrenRelationshipIterator<'a> {
    map: &'a RelationshipMap,
    parent: Entity,
    current_child: Option<Entity>
}

impl<'a> ChildrenRelationshipIterator<'a> {
    pub fn new(map: &'a RelationshipMap, parent: Entity) -> Self {
        Self { map, parent, current_child: None }
    }
}

impl <'a> Iterator for ChildrenRelationshipIterator<'a> {
    type Item = Entity;
    fn next(&mut self) -> Option<Entity> {
        if let Some(child) = self.current_child {
            if let Some(current_child_relationship) = self.map.get(&child) {
                self.current_child = current_child_relationship.next_sibling;
                return current_child_relationship.next_sibling
            } 
        } else {
            if let Some(parent_relationship) = self.map.get(&self.parent) {
                self.current_child = parent_relationship.first_child;
                return parent_relationship.first_child
            } 
        }
        None
    }
}