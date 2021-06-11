
use legion::*;
use log::{debug};
use legion::systems::*;
use zodiac::*;

use crate::positioning::*;


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
    debug!("removing from layout type map {:?}", entity);
    layout_map.remove(entity);
}

#[system(for_each)]
#[filter(component::<LayoutChange>())]
pub fn remove_layout_change(command_buffer: &mut CommandBuffer, entity: &Entity) {
    command_buffer.remove_component::<LayoutChange>(*entity);
}

#[system(for_each)]
#[filter(!component::<Mapped>())]
pub fn build_left_offset_map(#[resource] offset_map: &mut LeftOffsetMap, entity: &Entity, offset: &Left) {
    offset_map.insert(*entity, *offset);
}

#[system(for_each)]
#[filter(component::<Removed>())]
pub fn remove_from_left_offset_map(#[resource] offset_map: &mut LeftOffsetMap, entity: &Entity) {
    debug!("removing from left offset map {:?}", entity);
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
    debug!("removing from top offset map {:?}", entity);
    offset_map.remove(entity);
}