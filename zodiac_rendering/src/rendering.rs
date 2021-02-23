

use legion::*;
use legion::world::*;
use zodiac_entities::components::*;

#[derive(Debug)]
pub enum RendererError {
    FailedToDisplayWindow,
    FailedToCreateShaders,
    FailedToLoadFont,
    BufferSwapError,
    BufferCreationError,
    DrawError
}

pub trait Renderer {
    fn queue_rectangle_for_render(
        &mut self,
        index: usize,
        position: [u16; 2],
        dimensions: [u16; 2],
        inner_colour: [f32; 4],
        outer_colour: [f32; 4],
        stroke_width: f32,
        corner_radii: [f32; 4]);

    fn queue_circle_for_render(
        &mut self,
        index: usize,
        position: [u16; 2],
        radius: u16,
        inner_colour: [f32; 4],
        outer_colour: [f32; 4],
        stroke_width: f32);

    fn queue_text_for_render(
        &mut self,
        index: usize,
        position: [u16; 2],
        dimensions: [u16; 2],
        colour: [f32; 4],
        glyph_index: u16);

    fn render(&mut self) -> Result<(), RendererError>;
}

#[system(simple)]
#[read_component(Renderable)]
#[read_component(LayoutChange)]
#[read_component(Radius)]
#[read_component(Colour)]
#[read_component(StrokeColour)]
#[read_component(StrokeWidth)]
#[read_component(CornerRadii)]
#[read_component(GlyphIndex)]
#[read_component(Entity)]
pub fn render_primitives<T:Renderer + 'static>(world: &mut SubWorld, #[resource] renderer: &mut T) {
    let mut index = 0;
    let mut query = <(&Renderable, &LayoutChange, &Colour, TryRead<GlyphIndex>, TryRead<StrokeColour>, TryRead<StrokeWidth>, TryRead<CornerRadii>)>::query();
    for (renderable, layout_change, colour, glyph_index_option, stroke_colour_option, stroke_width_option, corner_radii_option) in query.iter_mut(world) {
        match renderable.render_type {
            RenderType::Rectangle => {
                let stroke_colour = stroke_colour_option.unwrap();
                let stroke_width = stroke_width_option.unwrap();
                let corner_radii = corner_radii_option.unwrap();
                renderer.queue_rectangle_for_render(
                    index,
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
                renderer.queue_circle_for_render(
                    index,
                    [layout_change.left, layout_change.top],
                    layout_change.width,
                    [colour.r, colour.g, colour.b, colour.a], 
                    [stroke_colour.r, stroke_colour.g, stroke_colour.b, stroke_colour.a], 
                    stroke_width.width as f32);
            },
            RenderType::Text => {
                let glyph_index = glyph_index_option.unwrap();
                renderer.queue_text_for_render(
                    index,
                    [layout_change.left, layout_change.top],
                    [layout_change.width, layout_change.height],
                    [colour.r, colour.g, colour.b, colour.a],
                    glyph_index.index);
            }
        }

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