use std::collections::{ HashMap };
use legion::*;
use crate::systems::relationships::*;
use zodiac_entities::components::*;

pub type WidthMap = HashMap<Entity, Width>;

pub fn create_width_map() -> WidthMap {
    WidthMap::new()
}

pub type HeightMap = HashMap<Entity, Height>;

pub fn create_height_map() -> HeightMap {
    HeightMap::new()
}

pub type MinimumWidthMap = HashMap<Entity, MinimumWidth>;

pub fn create_minimum_width_map() -> MinimumWidthMap {
    MinimumWidthMap::new()
}

pub type MinimumHeightMap = HashMap<Entity, MinimumHeight>;

pub fn create_minimum_height_map() -> MinimumHeightMap {
    MinimumHeightMap::new()
}

#[system(for_each)]
#[filter(!component::<Mapped>())]
pub fn build_width_map(
    #[resource] width_map: &mut WidthMap,
    entity: &Entity,
    width: &Width) {
        width_map.insert(*entity, *width);
    }

#[system(for_each)]
#[filter(!component::<Mapped>())]
pub fn build_height_map(
    #[resource] height_map: &mut HeightMap,
    entity: &Entity,
    height: &Height) {
        height_map.insert(*entity, *height);
    }


#[system(for_each)]
#[filter(!component::<Mapped>())]
pub fn build_width_and_height_maps_from_radius(
    #[resource] height_map: &mut HeightMap,
    #[resource] width_map: &mut WidthMap,
    entity: &Entity,
    radius: &Radius) {
        width_map.insert(*entity, Width::from(radius));
        height_map.insert(*entity, Height::from(radius));
    }

fn measure_fixed_widths(
    relationship_map: &RelationshipMap,
    width_map: &WidthMap,
    minimum_width_map: &mut MinimumWidthMap,
    entity: &Entity) -> u16 {
        let mut minimum_width = 0;

        for child in relationship_map.get_children(entity) {
            minimum_width += measure_fixed_widths(relationship_map, width_map, minimum_width_map, &child);
        }

        if let Some(width) = width_map.get(entity) {
            minimum_width = width.width;
        }

        if minimum_width > 0 {
            minimum_width_map.insert(*entity, MinimumWidth { width: minimum_width });
        }

        minimum_width
}  

#[system(for_each)]
#[filter(component::<Root>() & !component::<Mapped>())]
pub fn measure_fixed_width_constraints(
    #[resource] relationship_map: &RelationshipMap,
    #[resource] width_map: &WidthMap,
    #[resource] minimum_width_map: &mut MinimumWidthMap,
    entity: &Entity) {
        measure_fixed_widths(relationship_map, width_map, minimum_width_map, entity);
}

fn measure_fixed_heights(
    relationship_map: &RelationshipMap,
    height_map: &HeightMap,
    minimum_height_map: &mut MinimumHeightMap,
    entity: &Entity) -> u16 {
        let mut minimum_height = 0;

        for child in relationship_map.get_children(entity) {
            minimum_height += measure_fixed_heights(relationship_map, height_map, minimum_height_map, &child);
        }

        if let Some(height) = height_map.get(entity) {
            minimum_height = height.height;
        }

        if minimum_height > 0 {
            minimum_height_map.insert(*entity, MinimumHeight { height: minimum_height });
        }

        minimum_height
} 

#[system(for_each)]
#[filter(component::<Root>() & !component::<Mapped>())]
pub fn measure_fixed_height_constraints(
    #[resource] relationship_map: &RelationshipMap,
    #[resource] height_map: &HeightMap,
    #[resource] minimum_height_map: &mut MinimumHeightMap,
    entity: &Entity) {
        measure_fixed_heights(relationship_map, height_map, minimum_height_map, entity);
}                   