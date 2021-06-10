use legion::*;
use legion::systems::*;
use zodiac_entities::*;
use crate::testing::TestRenderQueue;
use crate::testing::components::*;

#[system(for_each)]
#[filter(component::<Rectangle>())]
pub fn queue_render_rectangle_primitives(
    entity: &Entity, 
    layout_change: &LayoutChange, 
    colour: &Colour,
    stroke_colour: &StrokeColour,
    stroke_width: &StrokeWidth,
    corner_radii: &CornerRadii,
    command_buffer: &mut CommandBuffer,
    #[resource] render_queue: &mut TestRenderQueue) {
    render_queue.queue_rectangle_for_render(
        command_buffer,
        entity,
        [layout_change.left, layout_change.top],
        [layout_change.width, layout_change.height],
        *colour,
        *stroke_colour, 
        stroke_width.width, 
        [corner_radii.left_top, corner_radii.right_top, corner_radii.right_bottom, corner_radii.left_bottom]);
}

#[system(for_each)]
#[filter(component::<Circle>())]
pub fn queue_render_circle_primitives(
    entity: &Entity,
    layout_change: &LayoutChange, 
    colour: &Colour,
    stroke_colour: &StrokeColour,
    stroke_width: &StrokeWidth,
    command_buffer: &mut CommandBuffer,
    #[resource] render_queue: &mut TestRenderQueue) {
    render_queue.queue_circle_for_render(
        command_buffer,
        entity,
        [layout_change.left, layout_change.top],
        layout_change.width,
        *colour,
        *stroke_colour, 
        stroke_width.width);
}

#[system(for_each)]
#[filter(component::<Text>())]
pub fn queue_render_text_primitives (
    entity: &Entity,
    layout_change: &LayoutChange, 
    colour: &Colour,
    content: &Content,
    font_size: &FontSize,
    command_buffer: &mut CommandBuffer,
    #[resource] render_queue: &mut TestRenderQueue) {
    render_queue.queue_text_for_render(
        command_buffer,
        entity,
        [layout_change.left, layout_change.top],
        [layout_change.width, layout_change.height],
        *colour,
        content.text.clone(),
        font_size.size);
}