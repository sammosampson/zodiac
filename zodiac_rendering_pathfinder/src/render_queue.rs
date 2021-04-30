use log::{debug};
use legion::*;
use legion::systems::*;
use zodiac_rendering::*;
use pathfinder_canvas::*;

#[derive(Clone, Debug, PartialEq)]
pub struct RenderPrimitive {
    pub definition: RenderPrimitiveDefinition,
}

#[derive(Clone, Debug, PartialEq)]
pub enum RenderPrimitiveDefinition {
    Rectangle(Vector2F, Vector2F, ColorU, ColorU, f32),
    Circle(Vector2F, Vector2F, ColorU, ColorU, f32),
    Text(Vector2F, Vector2F, ColorU, String),
}

pub fn create_pathfinder_render_queue() -> PathFinderRenderQueue {
    PathFinderRenderQueue::default()
}


#[derive(Default)]
pub struct PathFinderRenderQueue {
}

impl PathFinderRenderQueue {
    fn queue_primitive_for_render(&mut self, command_buffer: &mut CommandBuffer, entity: &Entity, to_queue: RenderPrimitiveDefinition) {
        debug!("Queueing: {:?}", to_queue);
        command_buffer.add_component(*entity, RenderPrimitive { definition: to_queue });
    }
}

impl RenderQueue for PathFinderRenderQueue {
    fn queue_rectangle_for_render(
        &mut self,
        command_buffer: &mut CommandBuffer,
        entity: &Entity,
        position: [u16; 2],
        dimensions: [u16; 2],
        inner_colour: [f32; 4],
        outer_colour: [f32; 4],
        stroke_width: f32,
        _: [u16; 4]) {
            self.queue_primitive_for_render(
                command_buffer,
                entity,
                RenderPrimitiveDefinition::Rectangle(
                    vec2f(position[0] as f32, position[1] as f32),
                    vec2f(dimensions[0] as f32, dimensions[1] as f32),
                    rgbaf(inner_colour[0], inner_colour[1], inner_colour[2], inner_colour[3]).to_u8(),
                    rgbaf(outer_colour[0], outer_colour[1], outer_colour[2], outer_colour[3]).to_u8(),
                    stroke_width
                ));
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
                RenderPrimitiveDefinition::Circle(
                    vec2f((position[0] + radius) as f32 ,(position[1] + radius) as f32),
                    vec2f(radius as f32, radius as f32),
                    rgbaf(inner_colour[0], inner_colour[1], inner_colour[2], inner_colour[3]).to_u8(),
                    rgbaf(outer_colour[0], outer_colour[1], outer_colour[2], outer_colour[3]).to_u8(),
                    stroke_width
                ));

    }
    
    fn queue_text_for_render(
        &mut self,
        command_buffer: &mut CommandBuffer,
        entity: &Entity,
        position: [u16; 2],
        dimensions: [u16; 2],
        colour: [f32; 4],
        text: String) {
            self.queue_primitive_for_render(
                command_buffer,
                entity,
                RenderPrimitiveDefinition::Text(
                    vec2f(position[0] as f32, position[1] as f32),
                    vec2f(dimensions[0] as f32, dimensions[1] as f32),
                    rgbaf(colour[0], colour[1], colour[2], colour[3]).to_u8(),
                    text
                ));

    }
}