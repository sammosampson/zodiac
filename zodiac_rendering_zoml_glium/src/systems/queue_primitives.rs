use legion::*;
use legion::systems::*;
use zodiac::*;
use zodiac_zoml::*;

use crate::RenderPrimitive;

#[system(for_each)]
#[filter(component::<Rectangle>())]
pub fn queue_render_rectangle_primitives(
    entity: &Entity, 
    layout_change: &LayoutChange, 
    colour: &Colour,
    stroke_colour: &StrokeColour,
    stroke_width: &StrokeWidth,
    corner_radii: &CornerRadii,
    command_buffer: &mut CommandBuffer) {

    command_buffer.add_component(
        *entity, 
        RenderPrimitive::rectangle(
            [layout_change.left, layout_change.top],
            [layout_change.width, layout_change.height],
            colour.into(),
            stroke_colour.into(), 
            stroke_width.width as f32, 
            [corner_radii.left_top, corner_radii.right_top, corner_radii.right_bottom, corner_radii.left_bottom]));
}

#[system(for_each)]
#[filter(component::<Circle>())]
pub fn queue_render_circle_primitives(
    entity: &Entity,
    layout_change: &LayoutChange, 
    colour: &Colour,
    stroke_colour: &StrokeColour,
    stroke_width: &StrokeWidth,
    command_buffer: &mut CommandBuffer) {
        
    command_buffer.add_component(
        *entity, 
        RenderPrimitive::circle(
            [layout_change.left, layout_change.top],
            layout_change.width,
            colour.into(),
            stroke_colour.into(), 
            stroke_width.width as f32));
}