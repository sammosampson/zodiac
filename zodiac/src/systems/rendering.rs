use legion::*;
use legion::world::*;
use legion::systems::*;
use zodiac_entities::components::*;
use zodiac_rendering::rendering::*;

#[system(simple)]
#[read_component(Position)]
#[read_component(Dimensions)]
#[read_component(Radius)]
#[read_component(Colour)]
#[read_component(StrokeColour)]
#[read_component(StrokeWidth)]
#[read_component(CornerRadii)]
#[read_component(GlyphIndex)]
#[read_component(Entity)]
pub fn render_primitives<T:Renderer + 'static>(world: &mut SubWorld, #[resource] renderer: &mut T) {
    let mut query = <(&Position, &Dimensions, &Colour, &StrokeColour, &StrokeWidth, &CornerRadii)>
        ::query()
        .filter(!component::<Rendered>());

    let mut index = 0;
    
    for (position, dimensions, colour, stroke_colour, stroke_width, corner_radii) in query.iter_mut(world) {
        renderer.queue_rectangle_for_render(
            index,
            [position.x, position.y],
            [dimensions.x, dimensions.y],
            [colour.r, colour.g, colour.b, colour.a], 
            [stroke_colour.r, stroke_colour.g, stroke_colour.b, stroke_colour.a], 
            stroke_width.width as f32, 
            [corner_radii.left_top, corner_radii.right_top, corner_radii.right_bottom, corner_radii.left_bottom]);
        index += 1;
    }

    let mut query = <(&Position, &Radius, &Colour, &StrokeColour, &StrokeWidth)>
        ::query()
        .filter(!component::<Rendered>());
    
    for (position, radius, colour, stroke_colour, stroke_width) in query.iter_mut(world) {
        renderer.queue_circle_for_render(
            index,
            [position.x, position.y],
            radius.radius,
            [colour.r, colour.g, colour.b, colour.a], 
            [stroke_colour.r, stroke_colour.g, stroke_colour.b, stroke_colour.a], 
            stroke_width.width as f32);
        index += 1;
    }

    let mut query = <(&Position, &Dimensions, &Colour, &GlyphIndex)>
        ::query()
        .filter(!component::<Rendered>());
    
    for (position, dimensions, colour, glyph_index) in query.iter_mut(world) {
        renderer.queue_text_for_render(
            index,
            [position.x, position.y],
            [dimensions.x, dimensions.y],
            [colour.r, colour.g, colour.b, colour.a],
            glyph_index.index);
        index += 1;
    }

    if index == 0 {
        return;
    }

    let draw_frame_start = std::time::Instant::now();
    renderer.render().unwrap();
    let draw_time = std::time::Instant::now() - draw_frame_start;
    println!("frame draw time: {:?}", draw_time);
}

#[system(for_each)]
#[filter(!component::<Rendered>())]
pub fn complete_render(command_buffer: &mut CommandBuffer, entity: &Entity) {
    command_buffer.add_component(*entity, Rendered {});
}