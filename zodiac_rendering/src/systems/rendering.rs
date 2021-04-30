use legion::*;
use legion::world::*;
use legion::systems::*;
use zodiac_entities::*;
use crate::render_queue::*;

#[system(simple)]
#[read_component(Renderable)]
#[read_component(LayoutChange)]
#[read_component(Radius)]
#[read_component(Colour)]
#[read_component(StrokeColour)]
#[read_component(StrokeWidth)]
#[read_component(CornerRadii)]
#[read_component(Content)]
pub fn queue_render_primitives<T:RenderQueue + 'static>(world: &mut SubWorld, command_buffer: &mut CommandBuffer, #[resource] render_queue: &mut T) {
    let mut query = <(Entity, &Renderable, &LayoutChange, &Colour, TryRead<Content>, TryRead<StrokeColour>, TryRead<StrokeWidth>, TryRead<CornerRadii>)>::query();
    for (entity, renderable, layout_change, colour, content_option, stroke_colour_option, stroke_width_option, corner_radii_option) in query.iter_mut(world) {
        match renderable.render_type {
            RenderType::Rectangle => {
                let stroke_colour = stroke_colour_option.unwrap();
                let stroke_width = stroke_width_option.unwrap();
                let corner_radii = corner_radii_option.unwrap();
                render_queue.queue_rectangle_for_render(
                    command_buffer,
                    entity,
                    [layout_change.left, layout_change.top],
                    [layout_change.width, layout_change.height],
                    [colour.r, colour.g, colour.b, colour.a], 
                    [stroke_colour.r, stroke_colour.g, stroke_colour.b, stroke_colour.a], 
                    stroke_width.width as f32, 
                    [corner_radii.left_top, corner_radii.right_top, corner_radii.right_bottom, corner_radii.left_bottom]);
            },
            RenderType::Circle => {
                let stroke_colour = stroke_colour_option.unwrap();
                let stroke_width = stroke_width_option.unwrap();
                render_queue.queue_circle_for_render(
                    command_buffer,
                    entity,
                    [layout_change.left, layout_change.top],
                    layout_change.width,
                    [colour.r, colour.g, colour.b, colour.a], 
                    [stroke_colour.r, stroke_colour.g, stroke_colour.b, stroke_colour.a], 
                    stroke_width.width as f32);
            },
            RenderType::Text => {
                let content = content_option.unwrap();
                render_queue.queue_text_for_render(
                    command_buffer,
                    entity,
                    [layout_change.left, layout_change.top],
                    [layout_change.width, layout_change.height],
                    [colour.r, colour.g, colour.b, colour.a],
                    content.text.clone());
            }
        }
    }
}