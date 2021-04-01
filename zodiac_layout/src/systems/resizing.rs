
use legion::*;
use legion::systems::*;
use legion::world::*;
use zodiac_entities::*;

use crate::measurement::*;
use crate::positioning::*;
use crate::resizing::*;
use crate::constraints::*;

#[system(simple)]
#[read_component(RootWindowResized)]
#[read_component(Root)]
#[write_component(LayoutRequest)]
pub fn resize_screen(world: &mut SubWorld, command_buffer: &mut CommandBuffer) {
    for (entity, window_resized) in <(Entity, &RootWindowResized)>::query()
        .iter(world) {
            for root in <Entity>::query()
                .filter(component::<Root>())
                .iter(world) {
                    command_buffer.add_component(*root, LayoutRequest::from(window_resized));
            }
            command_buffer.remove(*entity);
    } 
}

#[system(for_each)]
#[filter(component::<SourceFileChange>())]
#[filter(!component::<LayoutRequest>())]
pub fn resize_after_source_file_change(
    command_buffer: &mut CommandBuffer,
    entity: &Entity,
    current_layout_constraints: &CurrentLayoutConstraints
) {
    println!("source file change {:?}", current_layout_constraints);
    command_buffer.add_component(*entity, LayoutRequest::from(current_layout_constraints));
}

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

#[system(for_each)]
#[filter(component::<Resized>())]
pub fn remove_resized(command_buffer: &mut CommandBuffer, entity: &Entity) {
    command_buffer.remove_component::<Resized>(*entity);
    command_buffer.remove_component::<Mapped>(*entity);
}