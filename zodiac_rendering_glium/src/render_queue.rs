use log::{info};
use legion::*;
use legion::systems::*;
use zodiac_rendering::*;

use crate::primitives::*;

pub fn create_glium_render_queue() -> GliumRenderQueue {
    GliumRenderQueue::default()
}              

#[derive(Default)]
pub struct GliumRenderQueue {
}

impl GliumRenderQueue {  
    fn queue_primitive_for_render(&mut self, command_buffer: &mut CommandBuffer, entity: &Entity, to_queue: RenderPrimitive) {
        info!("Queueing: {:?}", to_queue);
        command_buffer.add_component(*entity, to_queue)
    }
}

impl RenderQueue for GliumRenderQueue {
    fn queue_rectangle_for_render(
        &mut self,
        command_buffer: &mut CommandBuffer,
        entity: &Entity,
        position: [u16; 2],
        dimensions: [u16; 2],
        inner_colour: [f32; 4],
        outer_colour: [f32; 4],
        stroke_width: f32,
        corner_radii: [u16; 4]) {
        self.queue_primitive_for_render(
            command_buffer,
            entity,
            RenderPrimitive::rectangle(
                position,
                dimensions,
                inner_colour,
                outer_colour,
                stroke_width,
                corner_radii));
    }

    fn queue_circle_for_render(
        &mut self,
        command_buffer: &mut CommandBuffer,
        entity: &Entity,
        position: [u16; 2],
        radius: u16,
        inner_colour: [f32; 4],
        outer_colour: [f32; 4],
        stroke_width: f32) {
        self.queue_primitive_for_render(
            command_buffer,
            entity,
            RenderPrimitive::circle(
                position,
                radius,
                inner_colour,
                outer_colour,
                stroke_width));
    }
    
    fn queue_glyph_for_render(
        &mut self,
        command_buffer: &mut CommandBuffer,
        entity: &Entity,
        position: [u16; 2],
        dimensions: [u16; 2],
        colour: [f32; 4],
        glyph_index: u16) {
        self.queue_primitive_for_render(
            command_buffer,
            entity,
            RenderPrimitive::glyph(
                position,
                dimensions,
                colour,
                glyph_index));
    }
}