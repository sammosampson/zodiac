use legion::*;
use legion::world::*;
use glium::*;
use glium::texture::Texture2dArray;
use zodiac_entities::components::*;
use crate::primitives::*;

#[system(simple)]
#[read_component(Position)]
#[read_component(Dimensions)]
#[read_component(Radius)]
#[read_component(Colour)]
#[read_component(StrokeColour)]
#[read_component(StrokeWidth)]
#[read_component(CornerRadii)]
#[read_component(GlyphIndex)]
pub fn render_primitives(
    world: &mut SubWorld,
    #[resource] display: &Display,
    #[resource] shader_program: &Program,
    #[resource] font_array: &Texture2dArray) {

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

    let vertices = VertexBuffer::dynamic(display, &primitives).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::Points);

    let framebuffer_dimensions = display.get_framebuffer_dimensions();
    let resolution: [f32;2] = [framebuffer_dimensions.0 as f32, framebuffer_dimensions.1 as f32];

    let uniforms = uniform! {
        uResolution: resolution,
        font_buffer: font_array.sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Linear)
    };

    let params = glium::DrawParameters {
        blend: glium::Blend::alpha_blending(),
        .. Default::default()
    };

    let mut target = display.draw();
    let draw_frame_start = std::time::Instant::now();
    target.clear_color(0.3, 0.3, 0.5, 1.0);
    target.draw(&vertices, &indices, shader_program, &uniforms, &params).unwrap();
    target.finish().unwrap();
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