
use legion::*;
use log::{info};
use zodiac::*;
use crate::layout::*;

#[system(for_each)]
#[filter(!component::<Mapped>())]
pub fn build_layout_type_map(
    #[resource] layout_map: &mut LayoutTypeMap,
    entity: &Entity,
    layout: &LayoutContent) {
        layout_map.insert(*entity, layout.layout_type);
}

#[system(for_each)]
#[filter(component::<Removed>())]
pub fn remove_from_layout_type_map(#[resource] layout_map: &mut LayoutTypeMap, entity: &Entity) {
    info!("removing from layout type map {:?}", entity);
    layout_map.remove(entity);
}
#[system(for_each)]
#[filter(!component::<Mapped>())]
pub fn build_left_offset_map(#[resource] offset_map: &mut LeftOffsetMap, entity: &Entity, offset: &Left) {
    offset_map.insert(*entity, *offset);
}

#[system(for_each)]
#[filter(component::<Removed>())]
pub fn remove_from_left_offset_map(#[resource] offset_map: &mut LeftOffsetMap, entity: &Entity) {
    info!("removing from left offset map {:?}", entity);
    offset_map.remove(entity);
}

#[system(for_each)]
#[filter(!component::<Mapped>())]
pub fn build_top_offset_map(#[resource] offset_map: &mut TopOffsetMap, entity: &Entity, offset: &Top) {
    offset_map.insert(*entity, *offset);
}

#[system(for_each)]
#[filter(component::<Removed>())]
pub fn remove_from_top_offset_map(#[resource] offset_map: &mut TopOffsetMap, entity: &Entity) {
    info!("removing from top offset map {:?}", entity);
    offset_map.remove(entity);
}
