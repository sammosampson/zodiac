
use legion::*;
use zodiac_entities::*;

use crate::relationships::*;

#[system(for_each)]
#[filter(!component::<Mapped>())]
pub fn build_relationship_map(
    #[resource] relationship_map: &mut RelationshipMap,
    entity: &Entity,
    relationship: &Relationship) {
        relationship_map.insert(*entity, *relationship);
}

