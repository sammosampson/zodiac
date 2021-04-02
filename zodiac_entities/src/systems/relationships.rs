
use legion::*;
use crate::*;
use crate::relationships::*;

#[system(for_each)]
#[filter(!component::<Mapped>())]
pub fn build_relationship_map(
    #[resource] relationship_map: &mut RelationshipMap,
    entity: &Entity,
    relationship: &Relationship) {
        relationship_map.insert(*entity, *relationship);
}

#[system(for_each)]
#[filter(component::<Removed>())]
pub fn remove_from_relationship_map(#[resource] relationship_map: &mut RelationshipMap, entity: &Entity) {
    relationship_map.remove(entity);
}