use legion::*;
use log::{debug};
use legion::systems::*;
use zodiac_entities::*;
use crate::text::*;

#[system(for_each)]
#[filter(!component::<Mapped>())]
pub fn build_text_colour_map(#[resource] colour_map: &mut TextColourMap, entity: &Entity, colour: &Colour) {
    colour_map.insert(*entity, *colour);
}

#[system(for_each)]
#[filter(component::<Removed>())]
pub fn remove_from_text_colour_map(#[resource] colour_map: &mut TextColourMap, entity: &Entity) {
    debug!("removing from text colour map {:?}", entity);
    colour_map.remove(entity);
}

#[system(for_each)]
#[filter(!component::<GlyphIndex>())]
pub fn format_glyphs(
    #[resource] colour_map: &TextColourMap,
    command_buffer: &mut CommandBuffer,
    relationship: &Relationship,
    character: &Character,
    entity: &Entity) {
    command_buffer.add_component(*entity, Left { left: character.position as u16 * 16 });
    command_buffer.add_component(*entity, Top { top: 0 });
    command_buffer.add_component(*entity, Width { width: 16 });
    command_buffer.add_component(*entity, Height { height: 16 });
    
    command_buffer.add_component(*entity, GlyphIndex { index: (character.character as u16) - 62 });
    if let Some(parent) = relationship.parent {
        if let Some(colour) = colour_map.get(&parent) {
            command_buffer.add_component(*entity, *colour); 
        }   
    }
}