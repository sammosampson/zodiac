use std::collections::{ HashMap };
use legion::*;
use legion::world::*;
use legion::systems::*;
use zodiac_entities::components::*;

pub struct AbsoluteOffsetMap {
    map: HashMap<Entity, Offset>
}

impl AbsoluteOffsetMap {
    pub fn new() -> Self {
        Self {
            map: HashMap::<Entity, Offset>::new()
        }
    }

    pub fn get_offset_for_entity(&self, entity: &Entity) -> Option<&Offset> {
        self.map.get(entity)
    }

    pub fn add_offset_for_entity(&mut self, entity: Entity, offset: Offset) {
        self.map.insert(entity, offset);
    }
}

#[system(simple)]
#[read_component(Offset)]
#[read_component(Relationship)]
pub fn position_children_of_canvases(
    #[resource] offset_map: &mut AbsoluteOffsetMap,
    world: &mut SubWorld,
    command_buffer: &mut CommandBuffer) {
        for (entity, possible_offset, relationship) in <(Entity, TryRead<Offset>, &Relationship)>::query()
            .filter(!component::<OffsetsMapped>())
            .iter(world) {
                let mut absolute_offset = Offset::default();
                
                if let Some(offset) = possible_offset {
                    absolute_offset = *offset;
                }

                if let Some(parent) = relationship.parent {
                    if let Some(parent_offset) = offset_map.get_offset_for_entity(&parent) {
                        absolute_offset = *parent_offset + absolute_offset;
                    }
                }

                offset_map.add_offset_for_entity(*entity, absolute_offset);
                command_buffer.add_component(*entity, OffsetsMapped {});
            }
        
        
        for entity in <Entity>::query()
            .filter(component::<Rectangle>() | component::<Circle>() | component::<Text>())
            .iter(world) {
                if let Some(offset) = offset_map.get_offset_for_entity(&entity) {
                    command_buffer.add_component(*entity, Position::from(*offset))
                }
            }

}