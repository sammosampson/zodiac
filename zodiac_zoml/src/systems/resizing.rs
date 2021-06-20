
use legion::*;
use legion::systems::*;
use legion::world::*;
use zodiac::*;
use crate::layout::*;

#[system(for_each)]
pub fn resize(
    #[resource] relationship_map: &RelationshipMap,
    #[resource] layout_map: &LayoutTypeMap,
    #[resource] left_map: &LeftOffsetMap,
    #[resource] top_map: &TopOffsetMap,
    #[resource] width_map: &WidthMap,
    #[resource] minimum_width_map: &MinimumWidthMap,
    #[resource] height_map: &HeightMap,
    #[resource] minimum_height_map: &MinimumHeightMap,
    world: &mut SubWorld,
    command_buffer: &mut CommandBuffer,
    entity: &Entity,
    resize_request: &LayoutRequest) {
        perform_resize(
            &LayoutMaps {
                relationship_map, 
                layout_map, 
                left_map,
                top_map,
                width_map, 
                minimum_width_map,
                height_map,
                minimum_height_map,
            },
            world,
            command_buffer,
            entity,
            &LayoutConstraints::from(resize_request));
}