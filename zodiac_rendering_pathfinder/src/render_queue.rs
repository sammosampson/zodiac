use log::{debug};
use legion::*;
use legion::systems::*;
use zodiac_entities::*;
use pathfinder_canvas::*;

#[derive(Clone, Debug, PartialEq)]
pub struct RenderPrimitive {
    pub definition: RenderPrimitiveDefinition,
}

#[derive(Clone, Debug, PartialEq)]
pub enum RenderPrimitiveDefinition {
    Rectangle(Vector2F, Vector2F, ColorU, ColorU, u16),
    Circle(Vector2F, Vector2F, ColorU, ColorU, u16),
    Text(Vector2F, Vector2F, ColorU, String, u8),
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

impl PathFinderRenderQueue {
    pub fn queue_rectangle_for_render(
        &mut self,
        command_buffer: &mut CommandBuffer,
        entity: &Entity,
        position: [u16; 2],
        dimensions: [u16; 2],
        inner_colour: Colour,
        outer_colour: StrokeColour,
        stroke_width: u16) {
            self.queue_primitive_for_render(
                command_buffer,
                entity,
                RenderPrimitiveDefinition::Rectangle(
                    vec2f(position[0] as f32, position[1] as f32),
                    vec2f(dimensions[0] as f32, dimensions[1] as f32),
                    rgbau(inner_colour.r, inner_colour.r, inner_colour.r, inner_colour.r),
                    rgbau(outer_colour.r, outer_colour.r, outer_colour.r, outer_colour.r),
                    stroke_width
                ));
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
                RenderPrimitiveDefinition::Circle(
                    vec2f((position[0] + radius) as f32 ,(position[1] + radius) as f32),
                    vec2f(radius as f32, radius as f32),
                    rgbau(inner_colour.r, inner_colour.r, inner_colour.r, inner_colour.r),
                    rgbau(outer_colour.r, outer_colour.r, outer_colour.r, outer_colour.r),
                    stroke_width
                ));

    }
    
    pub fn queue_text_for_render(
        &mut self,
        command_buffer: &mut CommandBuffer,
        entity: &Entity,
        position: [u16; 2],
        dimensions: [u16; 2],
        colour: Colour,
        text: String,
        font_size: u8) {
            self.queue_primitive_for_render(
                command_buffer,
                entity,
                RenderPrimitiveDefinition::Text(
                    vec2f(position[0] as f32, position[1] as f32),
                    vec2f(dimensions[0] as f32, dimensions[1] as f32),
                    rgbau(colour.r, colour.g, colour.b, colour.a),
                    text,
                    font_size
                ));

    }
}