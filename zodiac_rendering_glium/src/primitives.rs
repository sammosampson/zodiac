use glium::*;

#[derive(Copy, Clone)]
pub struct RenderPrimitive {
    pub position: [u16; 2],
    pub dimensions: [u16; 2],
    pub inner_colour: [f32; 4],
    pub outer_colour: [f32; 4],
    pub identification: [u16; 2],
    pub extra_data_1: [f32; 4],
    pub extra_data_2: [f32; 4]
}

implement_vertex!(
    RenderPrimitive,
    position, 
    dimensions, 
    inner_colour,
    outer_colour,
    identification,
    extra_data_1,
    extra_data_2
);

impl RenderPrimitive {
    pub fn with_position_size_colours_identification_and_data(
        position: [u16; 2],
        dimensions: [u16; 2],
        inner_colour: [f32; 4],
        outer_colour: [f32; 4],
        identification: [u16; 2],
        extra_data_1: [f32; 4],
        extra_data_2: [f32; 4]) -> Self {
        Self {
            position,
            dimensions,
            inner_colour,
            outer_colour,
            identification,
            extra_data_1,
            extra_data_2
        }
    }
    
    pub fn circle(
        position: [u16; 2],
        radius: u16,
        inner_colour: [f32; 4],
        outer_colour: [f32; 4],
        stroke_width: f32) -> Self {
        RenderPrimitive::with_position_size_colours_identification_and_data(
            position,
            [radius, radius],
            inner_colour,
            outer_colour,
            [0, 0],
            [stroke_width, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0]
        ) 
    }

    pub fn rectangle(
        position: [u16; 2],
        dimensions: [u16; 2],
        inner_colour: [f32; 4],
        outer_colour: [f32; 4],
        stroke_width: f32,
        corner_radii: [f32; 4]) -> Self {
        RenderPrimitive::with_position_size_colours_identification_and_data(
            position,
            dimensions,
            inner_colour,
            outer_colour,
            [1, 0],
            [stroke_width, 0.0, 0.0, 0.0],
            corner_radii
        ) 
    }

    pub fn text(
        position: [u16; 2],
        dimensions: [u16; 2],
        colour: [f32; 4],
        glyph_index: u16) -> Self {
        RenderPrimitive::with_position_size_colours_identification_and_data(
            position,
            dimensions,
            colour,
            colour,
            [2, glyph_index],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0]
        ) 
    }
}
