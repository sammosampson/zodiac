use std::collections::{ HashMap };
use legion::*;
use legion::systems::*;
use zodiac_entities::components::*;

pub type RelationshipMap = HashMap<Entity, Relationship>;

pub fn create_relationship_map() -> RelationshipMap {
    RelationshipMap::new()
}

pub trait ParentRetrieval {
    fn get_parent(&self, entity: &Entity) -> Option<Entity>;
}

pub trait ChildrenRetrieval {
    fn get_children(&self, entity: &Entity) -> ChildrenRelationshipIterator;
}

impl ParentRetrieval for RelationshipMap {
    fn get_parent(&self, entity: &Entity) -> Option<Entity> {
        if let Some(relationship) = self.get(entity) {
            return relationship.parent;
        }
        None
    }
}

impl ChildrenRetrieval for RelationshipMap {
    fn get_children(&self, parent: &Entity) -> ChildrenRelationshipIterator {

        ChildrenRelationshipIterator::new(self, *parent)
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

pub type LeftOffsetMap = HashMap<Entity, Left>;

pub fn create_left_offset_map() -> LeftOffsetMap {
    LeftOffsetMap::new()
}

pub type TopOffsetMap = HashMap<Entity, Top>;

pub fn create_top_offset_map() -> TopOffsetMap {
    TopOffsetMap::new()
}

pub type WidthMap = HashMap<Entity, Width>;

pub fn create_width_map() -> WidthMap {
    WidthMap::new()
}

pub type HeightMap = HashMap<Entity, Height>;

pub fn create_height_map() -> HeightMap {
    HeightMap::new()
}

#[system(for_each)]
#[filter(!component::<RelationshipMapped>())]
pub fn build_relationship_map(
    #[resource] relationship_map: &mut RelationshipMap,
    command_buffer: &mut CommandBuffer,
    entity: &Entity,
    relationship: &Relationship) {
        relationship_map.insert(*entity, *relationship);
        command_buffer.add_component(*entity, RelationshipMapped {});
    }

#[system(for_each)]
#[filter(!component::<LeftOffsetMapped>())]
pub fn build_left_offset_map(
    #[resource] offset_map: &mut LeftOffsetMap,
    command_buffer: &mut CommandBuffer,
    entity: &Entity,
    offset: &Left) {
        offset_map.insert(*entity, *offset);
        command_buffer.add_component(*entity, LeftOffsetMapped {});
    }

#[system(for_each)]
#[filter(!component::<TopOffsetMapped>())]
pub fn build_top_offset_map(
    #[resource] offset_map: &mut TopOffsetMap,
    command_buffer: &mut CommandBuffer,
    entity: &Entity,
    offset: &Top) {
        offset_map.insert(*entity, *offset);
        command_buffer.add_component(*entity, TopOffsetMapped {});
    }

#[system(for_each)]
#[filter(!component::<WidthMapped>())]
pub fn build_width_map(
    #[resource] width_map: &mut WidthMap,
    command_buffer: &mut CommandBuffer,
    entity: &Entity,
    width: &Width) {
        width_map.insert(*entity, *width);
        command_buffer.add_component(*entity, WidthMapped {});
    }

#[system(for_each)]
#[filter(!component::<HeightMapped>())]
pub fn build_height_map(
    #[resource] height_map: &mut HeightMap,
    command_buffer: &mut CommandBuffer,
    entity: &Entity,
    height: &Height) {
        height_map.insert(*entity, *height);
        command_buffer.add_component(*entity, HeightMapped {});
    }