use legion::*;
use legion::systems::*;
use zodiac_entities::*;
use crate::render_queue::*;

#[system(for_each)]
pub fn queue_render_rectangle_primitives<T:RenderQueue + 'static> (
    entity: &Entity, 
    renderable: &Renderable, 
    layout_change: &LayoutChange, 
    colour: &Colour,
    stroke_colour: &StrokeColour,
    stroke_width: &StrokeWidth,
    corner_radii: &CornerRadii,
    command_buffer: &mut CommandBuffer,
    #[resource] render_queue: &mut T) {
    
    if renderable.render_type != RenderType::Rectangle {
        return;
    }

    render_queue.queue_rectangle_for_render(
        command_buffer,
        entity,
        [layout_change.left, layout_change.top],
        [layout_change.width, layout_change.height],
        [colour.r, colour.g, colour.b, colour.a], 
        [stroke_colour.r, stroke_colour.g, stroke_colour.b, stroke_colour.a], 
        stroke_width.width as f32, 
        [corner_radii.left_top, corner_radii.right_top, corner_radii.right_bottom, corner_radii.left_bottom]);
}

#[system(for_each)]
pub fn queue_render_circle_primitives<T:RenderQueue + 'static> (
    entity: &Entity, 
    renderable: &Renderable, 
    layout_change: &LayoutChange, 
    colour: &Colour,
    stroke_colour: &StrokeColour,
    stroke_width: &StrokeWidth,
    command_buffer: &mut CommandBuffer,
    #[resource] render_queue: &mut T) {
    
    if renderable.render_type != RenderType::Circle {
        return;
    }

    render_queue.queue_circle_for_render(
        command_buffer,
        entity,
        [layout_change.left, layout_change.top],
        layout_change.width,
        [colour.r, colour.g, colour.b, colour.a], 
        [stroke_colour.r, stroke_colour.g, stroke_colour.b, stroke_colour.a], 
        stroke_width.width as f32);
}



#[system(for_each)]
pub fn queue_render_text_primitives<T:RenderQueue + 'static> (
    entity: &Entity, 
    renderable: &Renderable, 
    layout_change: &LayoutChange, 
    colour: &Colour,
    content: &Content,
    font_size: &FontSize,
    command_buffer: &mut CommandBuffer,
    #[resource] render_queue: &mut T) {
    
    if renderable.render_type != RenderType::Text {
        return;
    }

    render_queue.queue_text_for_render(
        command_buffer,
        entity,
        [layout_change.left, layout_change.top],
        [layout_change.width, layout_change.height],
        [colour.r, colour.g, colour.b, colour.a],
        content.text.clone(),
        font_size.size as f32);
}