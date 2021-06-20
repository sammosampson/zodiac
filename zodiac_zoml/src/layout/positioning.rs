use std::collections::{ HashMap };
use legion::*;
use zodiac::*;
use crate::LayoutType;

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