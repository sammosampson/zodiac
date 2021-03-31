
#[derive(Debug)]
pub enum RendererError {
    FailedToDisplayWindow,
    FailedToCreateShaders(String),
    FailedToLoadFont,
    BufferSwapError,
    BufferCreationError,
    DrawError
}

pub trait Renderer {
    fn get_window_dimensions(&self) -> (u32, u32);

    fn queue_rectangle_for_render(
        &mut self,
        index: usize,
        position: [u16; 2],
        dimensions: [u16; 2],
        inner_colour: [f32; 4],
        outer_colour: [f32; 4],
        stroke_width: f32,
        corner_radii: [u16; 4]);

    fn queue_circle_for_render(
        &mut self,
        index: usize,
        position: [u16; 2],
        radius: u16,
        inner_colour: [f32; 4],
        outer_colour: [f32; 4],
        stroke_width: f32);

    fn queue_glyph_for_render(
        &mut self,
        index: usize,
        position: [u16; 2],
        dimensions: [u16; 2],
        colour: [f32; 4],
        glyph_index: u16);

    fn render(&mut self) -> Result<(), RendererError>;
}