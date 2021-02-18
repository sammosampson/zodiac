use std::collections::{ HashMap };
use legion::*;
use crate::systems::maps::*;
use zodiac_entities::components::*;

pub type MinimumWidthMap = HashMap<Entity, MinimumWidth>;

pub fn create_minimum_width_map() -> MinimumWidthMap {
    MinimumWidthMap::new()
}

#[system(for_each)]
#[filter(component::<ResizeRequest>())]
pub fn measure_fixed_widths(
    #[resource] relationship_map: &RelationshipMap,
    #[resource] width_map: &WidthMap,
    #[resource] minimum_width_map: &mut MinimumWidthMap,
    entity: &Entity) {
        let mut minimum_width = 0;
        for child in relationship_map.get_children(entity) {
            if let Some(child_width) = width_map.get(&child) {
                minimum_width += child_width.width;
            }
        }
        if minimum_width > 0 {
            minimum_width_map.insert(*entity, MinimumWidth { width: minimum_width });
        }
}                      