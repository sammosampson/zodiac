
use legion::*;
use zodiac::*;
use crate::components::Radius;

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