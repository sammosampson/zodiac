use std::collections::{ HashMap };
use legion::*;
use zodiac_entities::*;

pub type TextColourMap = HashMap<Entity, Colour>;

pub fn create_text_colour_map() -> TextColourMap {
    TextColourMap::new()
}