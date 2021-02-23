use legion::*;
use legion::world::*;
use legion::systems::*;
use zodiac_entities::components::*;
use crate::relationships::*;
use crate::constraints::*;
use crate::positioning::*;
use crate::measurement::*;

struct LayoutMaps<'a> {
    relationship_map: &'a RelationshipMap,
    layout_map: &'a LayoutTypeMap,
    left_map: &'a LeftOffsetMap,
    top_map: &'a TopOffsetMap,
    width_map: &'a WidthMap,
    minimum_width_map: &'a MinimumWidthMap,
    height_map: &'a HeightMap,
    minimum_height_map: &'a MinimumHeightMap
}

#[system(simple)]
#[read_component(RootWindowResized)]
#[read_component(Root)]
#[write_component(ResizeRequest)]
pub fn resize_screen(world: &mut SubWorld, command_buffer: &mut CommandBuffer) {
    for (entity, window_resized) in <(Entity, &RootWindowResized)>::query()
        .iter(world) {
            for root in <Entity>::query()
                .filter(component::<Root>())
                .iter(world) {
                    command_buffer.add_component(*root, ResizeRequest::from(window_resized));
            }
            command_buffer.remove(*entity);
    } 
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
    resize_request: &ResizeRequest) {
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

fn perform_resize(
    maps: &LayoutMaps,
    world: &mut SubWorld,
    command_buffer: &mut CommandBuffer,
    entity: &Entity, 
    constraints: &LayoutConstraints) {
        command_buffer.add_component(*entity, Width { width: constraints.width });
        command_buffer.add_component(*entity, Height { height: constraints.height });
        command_buffer.add_component(*entity, Resized {});
        command_buffer.remove_component::<ResizeRequest>(*entity);
        perform_layout(maps, world, command_buffer, entity, constraints);
}

fn perform_layout(
    maps: &LayoutMaps,
    world: &mut SubWorld,
    command_buffer: &mut CommandBuffer,
    entity: &Entity, 
    constraints: &LayoutConstraints) {
        if let Some(layout_type) = maps.layout_map.get(entity) {
            match layout_type {
                LayoutType::Canvas => layout_canvas(maps, world, command_buffer, entity, constraints),
                LayoutType::Horizontal => layout_horizontal(maps, world, command_buffer, entity, constraints),
                LayoutType::Vertical => layout_vertical(maps, world, command_buffer, entity, constraints)
            }
        } else {
            layout_renderable(maps, command_buffer, entity, constraints);
        }
}

fn layout_canvas(
    maps: &LayoutMaps,
    world: &mut SubWorld,
    command_buffer: &mut CommandBuffer,
    entity: &Entity, 
    constraints: &LayoutConstraints) {
        let mut new_constraints = *constraints;
        if let Some(left) = maps.left_map.get(entity) {
            new_constraints = new_constraints + *left;
        }
        if let Some(top) = maps.top_map.get(entity) {
            new_constraints = new_constraints + *top;
        }
        for child in maps.relationship_map.get_children(entity) {
            perform_layout(maps, world, command_buffer, &child, &new_constraints);
        } 
}

fn layout_horizontal(
    maps: &LayoutMaps,
    world: &mut SubWorld,
    command_buffer: &mut CommandBuffer,
    entity: &Entity, 
    constraints: &LayoutConstraints) {
        let mut subdivider = constraints.into_width_subdivider(&maps.minimum_width_map);

        for child in maps.relationship_map.get_children(entity) {
            subdivider.subdivide_for_entity(&child);
        }
        
        for (child, new_constraints) in subdivider.iter() {
            perform_layout(maps, world, command_buffer, &child, &new_constraints);
        }
}

fn layout_vertical(
    maps: &LayoutMaps,
    world: &mut SubWorld,
    command_buffer: &mut CommandBuffer,
    entity: &Entity, 
    constraints: &LayoutConstraints) {
        let mut subdivider = constraints.into_height_subdivider(&maps.minimum_height_map);

        for child in maps.relationship_map.get_children(entity) {
            subdivider.subdivide_for_entity(&child);
        }
        
        for (child, new_constraints) in subdivider.iter() {
            perform_layout(maps, world, command_buffer, &child, &new_constraints);
        }
}

fn layout_renderable(
    maps: &LayoutMaps,
    command_buffer: &mut CommandBuffer,
    entity: &Entity, 
    constraints: &LayoutConstraints) {
        let mut layout_change = LayoutChange::from(constraints);
        
        if let Some(left) = maps.left_map.get(entity) {
            layout_change = layout_change + *left;
        }
        if let Some(top) = maps.top_map.get(entity) {
            layout_change = layout_change + *top;
        }
        if let Some(width) = maps.width_map.get(entity) {
            layout_change.width = width.width;
        }
        if let Some(height) = maps.height_map.get(entity) {
            layout_change.height = height.height;
        }
        println!("{:?}", layout_change);
        command_buffer.add_component(*entity, layout_change);
}