use log::{debug};
use legion::*;
use legion::systems::*;
use crate::components::*;
use crate::primitives::*;

pub fn create_glium_render_queue() -> GliumRenderQueue {
    GliumRenderQueue::default()
}              

#[derive(Default)]
pub struct GliumRenderQueue {
}

impl GliumRenderQueue {  
    fn queue_primitive_for_render(&mut self, command_buffer: &mut CommandBuffer, entity: &Entity, to_queue: RenderPrimitive) {
        debug!("Queueing: {:?}", to_queue);
        command_buffer.add_component(*entity, to_queue)
    }
}

impl GliumRenderQueue {
    pub fn queue_rectangle_for_render(
        &mut self,
        command_buffer: &mut CommandBuffer,
        entity: &Entity,
        position: [u16; 2],
        dimensions: [u16; 2],
        inner_colour: Colour,
        outer_colour: StrokeColour,
        stroke_width: u16,
        corner_radii: [u16; 4]) {
        self.queue_primitive_for_render(
            command_buffer,
            entity,
            RenderPrimitive::rectangle(
                position,
                dimensions,
                inner_colour.into(),
                outer_colour.into(),
                stroke_width as f32,
                corner_radii));
    }

    pub fn queue_circle_for_render(
        &mut self,
        command_buffer: &mut CommandBuffer,
        entity: &Entity,
        position: [u16; 2],
        radius: u16,
        inner_colour: Colour,
        outer_colour: StrokeColour,
        stroke_width: u16) {
        self.queue_primitive_for_render(
            command_buffer,
            entity,
            RenderPrimitive::circle(
                position,
                radius,
                inner_colour.into(),
                outer_colour.into(),
                stroke_width as f32));
    }
}