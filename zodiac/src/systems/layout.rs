use legion::*;
use legion::world::*;
use legion::systems::*;
use crate::systems::maps::*;
use zodiac_entities::components::*;

fn get_absolute_offset(
    relationship_map: &mut RelationshipMap,
    left_offset_map: &mut LeftOffsetMap,
    top_offset_map: &mut TopOffsetMap,
    entity: &Entity) -> Position {
        let left_offset = match left_offset_map.get(entity) {
            Some(left) => Position { x: left.left, y: 0 },
            None => Position::default()
        };
        
        let top_offset = match top_offset_map.get(entity) {
            Some(top) => Position { x: 0, y: top.top },
            None => Position::default()
        };

        let mut offset = left_offset + top_offset;

        if let Some(parent) = relationship_map.get_parent(entity) {
            offset = offset + get_absolute_offset(relationship_map, left_offset_map, top_offset_map, &parent);
        }

        offset
    }

fn set_position_on_renderables(
    relationship_map: &mut RelationshipMap,
    left_offset_map: &mut LeftOffsetMap,
    top_offset_map: &mut TopOffsetMap,
    world: &mut SubWorld,
    command_buffer: &mut CommandBuffer) {
    for entity in <Entity>::query()
        .filter(!component::<Position>() & (component::<Rectangle>() | component::<Circle>() | component::<Text>()))
        .iter(world) {
            let position = get_absolute_offset(relationship_map, left_offset_map, top_offset_map, entity);
            command_buffer.add_component(*entity, position);
        }
}

fn set_dimensions_on_renderables(
    world: &mut SubWorld,
    command_buffer: &mut CommandBuffer) {
    for (entity, width, height) in <(Entity, &Width, &Height)>::query()
        .filter(!component::<Dimensions>() & (component::<Rectangle>() | component::<Text>()))
        .iter(world) {
            command_buffer.add_component(*entity, Dimensions { x: width.width, y: height.height })
        }
}

#[system(simple)]
#[read_component(Width)]
#[read_component(Height)]
pub fn layout(
    #[resource] relationship_map: &mut RelationshipMap,
    #[resource] left_offset_map: &mut LeftOffsetMap,
    #[resource] top_offset_map: &mut TopOffsetMap,
    world: &mut SubWorld,
    command_buffer: &mut CommandBuffer) {
        set_position_on_renderables(relationship_map, left_offset_map, top_offset_map, world, command_buffer);
        set_dimensions_on_renderables(world, command_buffer);
}