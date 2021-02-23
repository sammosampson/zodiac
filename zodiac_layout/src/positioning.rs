use std::collections::{ HashMap };
use legion::*;
use zodiac_entities::components::*;

pub type LeftOffsetMap = HashMap<Entity, Left>;

pub fn create_left_offset_map() -> LeftOffsetMap {
    LeftOffsetMap::new()
}

pub type TopOffsetMap = HashMap<Entity, Top>;

pub fn create_top_offset_map() -> TopOffsetMap {
    TopOffsetMap::new()
}

pub type LayoutTypeMap = HashMap<Entity, LayoutType>;

pub fn create_layout_type_map() -> LayoutTypeMap {
    LayoutTypeMap::new()
}

#[system(for_each)]
#[filter(!component::<Mapped>())]
pub fn build_layout_type_map(
    #[resource] layout_map: &mut LayoutTypeMap,
    entity: &Entity,
    layout: &LayoutContent) {
        layout_map.insert(*entity, layout.layout_type);
}

#[system(for_each)]
#[filter(!component::<Mapped>())]
pub fn build_left_offset_map(
    #[resource] offset_map: &mut LeftOffsetMap,
    entity: &Entity,
    offset: &Left) {
        offset_map.insert(*entity, *offset);
}

#[system(for_each)]
#[filter(!component::<Mapped>())]
pub fn build_top_offset_map(
    #[resource] offset_map: &mut TopOffsetMap,
    entity: &Entity,
    offset: &Top) {
        offset_map.insert(*entity, *offset);
}