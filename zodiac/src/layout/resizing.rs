use legion::*;
use log::{debug};
use legion::world::*;
use legion::systems::*;
use crate::*;

pub struct LayoutMaps<'a> {
    pub relationship_map: &'a RelationshipMap,
    pub layout_map: &'a LayoutTypeMap,
    pub left_map: &'a LeftOffsetMap,
    pub top_map: &'a TopOffsetMap,
    pub width_map: &'a WidthMap,
    pub minimum_width_map: &'a MinimumWidthMap,
    pub height_map: &'a HeightMap,
    pub minimum_height_map: &'a MinimumHeightMap
}

pub fn perform_resize(
    maps: &LayoutMaps,
    world: &mut SubWorld,
    command_buffer: &mut CommandBuffer,
    entity: &Entity, 
    constraints: &LayoutConstraints) {
        command_buffer.add_component(*entity, Width { width: constraints.width });
        command_buffer.add_component(*entity, Height { height: constraints.height });
        command_buffer.add_component(*entity, Resized::default());
        command_buffer.remove_component::<LayoutRequest>(*entity);
        perform_layout(maps, world, command_buffer, entity, constraints);
}

pub fn perform_layout(
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
        command_buffer.add_component(*entity, CurrentLayoutConstraints::from(constraints));
}

pub fn layout_canvas(
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
        debug!("Layout change for {:?} {:?}", entity, layout_change);
        command_buffer.add_component(*entity, layout_change);
}