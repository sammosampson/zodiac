use legion::*;
use legion::world::*;
use zodiac_entities::components::*;
use crate::primitives::*;
use crate::rendering::*;

#[system(simple)]
#[read_component(Position)]
#[read_component(Dimensions)]
#[read_component(Radius)]
#[read_component(Colour)]
#[read_component(StrokeColour)]
#[read_component(StrokeWidth)]
#[read_component(CornerRadii)]
#[read_component(GlyphIndex)]
pub fn render_primitives(world: &mut SubWorld, #[resource] renderer: &mut GliumRenderer) {
    let mut primitives: Vec::<RenderPrimitive> = vec!();
    let mut query = <(&Position, &Dimensions, &Colour, &StrokeColour, &StrokeWidth, &CornerRadii)>
        ::query()
        .filter(component::<Rectangle>());
    
    for (position, dimensions, colour, stroke_colour, stroke_width, corner_radii) in query.iter_mut(world) {
        primitives.push(
            RenderPrimitive::rectangle(
                [position.x, position.y],
                [dimensions.x, dimensions.y],
                [colour.r, colour.g, colour.b, colour.a], 
                [stroke_colour.r, stroke_colour.g, stroke_colour.b, stroke_colour.a], 
                stroke_width.width as f32, 
                [corner_radii.left_top, corner_radii.right_top, corner_radii.right_bottom, corner_radii.left_bottom])
        );
    }

    let mut query = <(&Position, &Radius, &Colour, &StrokeColour, &StrokeWidth)>
        ::query()
        .filter(component::<Circle>());
    
    for (position, radius, colour, stroke_colour, stroke_width) in query.iter_mut(world) {
        primitives.push(
            RenderPrimitive::circle(
                [position.x, position.y],
                radius.radius,
                [colour.r, colour.g, colour.b, colour.a], 
                [stroke_colour.r, stroke_colour.g, stroke_colour.b, stroke_colour.a], 
                stroke_width.width as f32)
        );
    }

    let mut query = <(&Position, &Dimensions, &Colour, &GlyphIndex)>
        ::query()
        .filter(component::<Text>());
    
    for (position, dimensions, colour, glyph_index) in query.iter_mut(world) {
        primitives.push(
            RenderPrimitive::text(
                [position.x, position.y],
                [dimensions.x, dimensions.y],
                [colour.r, colour.g, colour.b, colour.a],
                glyph_index.index)
        );
    }

    let draw_frame_start = std::time::Instant::now();
    renderer.render(&primitives).unwrap();
    let draw_time = std::time::Instant::now() - draw_frame_start;
    println!("frame draw time: {:?}", draw_time);
}

#[system(for_each)]
pub fn window_event_loop(
    _rectangle: &Rectangle, 
    position: &Position, 
    dimensions: &Dimensions, 
    colour: &Colour,
    stroke_colour: &StrokeColour,
    stroke_width: &StrokeWidth, 
    corner_radii: &CornerRadii) {
        println!("{:?}", position);
        println!("{:?}", dimensions);
        println!("{:?}", colour);
        println!("{:?}", stroke_colour);
        println!("{:?}", stroke_width);
        println!("{:?}", corner_radii);
}