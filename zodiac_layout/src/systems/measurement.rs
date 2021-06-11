
use legion::*;
use log::{debug};
use zodiac::*;
use crate::measurement::*;

#[system(for_each)]
#[filter(!component::<Mapped>())]
pub fn build_width_map(#[resource] width_map: &mut WidthMap, entity: &Entity, width: &Width) {
    width_map.insert(*entity, *width);
}

#[system(for_each)]
#[filter(component::<Removed>())]
pub fn remove_from_minimum_width_map(#[resource] width_map: &mut MinimumWidthMap, entity: &Entity) {
    debug!("removing from min width map {:?}", entity);
    width_map.remove(entity);
}

#[system(for_each)]
#[filter(component::<Removed>())]
pub fn remove_from_width_map(#[resource] width_map: &mut WidthMap, entity: &Entity) {
    debug!("removing from width map {:?}", entity);
    width_map.remove(entity);
}

#[system(for_each)]
#[filter(!component::<Mapped>())]
pub fn build_height_map(#[resource] height_map: &mut HeightMap, entity: &Entity, height: &Height) {
    height_map.insert(*entity, *height);
}

#[system(for_each)]
#[filter(component::<Removed>())]
pub fn remove_from_minimum_height_map(#[resource] height_map: &mut MinimumHeightMap, entity: &Entity) {
    debug!("removing from min height map {:?}", entity);
    height_map.remove(entity);
}

#[system(for_each)]
#[filter(component::<Removed>())]
pub fn remove_from_height_map(#[resource] height_map: &mut HeightMap, entity: &Entity) {
    debug!("removing from height map {:?}", entity);
    height_map.remove(entity);
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

#[system(for_each)]
#[filter(component::<Root>() & !component::<Mapped>())]
pub fn measure_fixed_width_constraints(
    #[resource] relationship_map: &RelationshipMap,
    #[resource] width_map: &WidthMap,
    #[resource] minimum_width_map: &mut MinimumWidthMap,
    entity: &Entity) {
        measure_fixed_widths(relationship_map, width_map, minimum_width_map, entity);
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
