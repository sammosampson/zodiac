use legion::*;
use legion::systems::*;
use zodiac_entities::*;
pub trait RenderQueue {
    fn queue_rectangle_for_render(
        &mut self,
        command_buffer: &mut CommandBuffer,
        entity: &Entity,
        position: [u16; 2],
        dimensions: [u16; 2],
        inner_colour: Colour,
        outer_colour: StrokeColour,
        stroke_width: u16,
        corner_radii: [u16; 4]);

    fn queue_circle_for_render(
        &mut self,
        command_buffer: &mut CommandBuffer,
        entity: &Entity,
        position: [u16; 2],
        radius: u16,
        inner_colour: Colour,
        outer_colour: StrokeColour,
        stroke_width: u16);

    fn queue_text_for_render(
        &mut self,
        command_buffer: &mut CommandBuffer,
        entity: &Entity,
        position: [u16; 2],
        dimensions: [u16; 2],
        colour: Colour,
        text: String,
        font_size: u8);
}