use legion::*;
use legion::systems::*;
use legion::world::*;
use zodiac::*;

use crate::layout::*;

#[system(for_each)]
#[read_component(Display)]
pub fn resize(
    #[resource] relationship_map: &RelationshipMap,
    world: &mut SubWorld,
    command_buffer: &mut CommandBuffer,
    entity: &Entity,
    resize_request: &LayoutRequest) {
        perform_resize(relationship_map, world, command_buffer, entity, &LayoutConstraints::from(resize_request));
}

fn perform_resize(
    relationship_map: &RelationshipMap,
    world: &mut SubWorld,
    command_buffer: &mut CommandBuffer,
    entity: &Entity, 
    constraints: &LayoutConstraints) {
    
    command_buffer.add_component(*entity, Resized::default());
    command_buffer.remove_component::<LayoutRequest>(*entity);
    perform_layout(relationship_map, world, command_buffer, entity, constraints);
}

fn perform_layout(
    relationship_map: &RelationshipMap,
    world: &mut SubWorld,
    command_buffer: &mut CommandBuffer,
    entity: &Entity, 
    constraints: &LayoutConstraints) {
    if let Some(layout_type) = get_display(world, entity) {
        match layout_type {
            DisplayTypes::Block => layout_block(relationship_map, world, command_buffer, entity, constraints),
            DisplayTypes::Inline => layout_block(relationship_map, world, command_buffer, entity, constraints),
            DisplayTypes::None => todo!(),
        }
    } else {
        for child in relationship_map.get_children(entity) {
            perform_layout(relationship_map, world, command_buffer, &child, constraints);
        }
    }
}

fn get_display(world: &mut SubWorld, entity: &Entity) -> Option<DisplayTypes> {
    if let Some(display) = world.entry_ref(*entity).unwrap().get_component::<Display>().ok() {
        return Some(display.into())
    }
    None
}

fn layout_block(
    relationship_map: &RelationshipMap,
    world: &mut SubWorld,
    command_buffer: &mut CommandBuffer,
    entity: &Entity, 
    constraints: &LayoutConstraints) {
    
    
    for child in relationship_map.get_children(entity) {        
        let layout_change = LayoutChange::from(constraints);
        command_buffer.add_component(child, layout_change);
        perform_layout(relationship_map, world, command_buffer, &child, constraints);
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LayoutConstraints {
    pub left: u16,
    pub top: u16,
    pub width: u16,
    pub height: u16
}

impl From<&LayoutRequest> for LayoutConstraints {
    fn from(request: &LayoutRequest) -> Self {
        LayoutConstraints {
            left: request.left,
            top: request.top,
            width: request.width,
            height: request.height
        }
    }
}

impl From<&LayoutConstraints> for LayoutChange {
    fn from(constraints: &LayoutConstraints) -> Self {
        LayoutChange {
            left: constraints.left,
            top: constraints.top,
            width: constraints.width,
            height: constraints.height
        }
    }
}