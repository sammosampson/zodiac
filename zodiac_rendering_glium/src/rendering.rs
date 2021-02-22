use glium::*;

use glium::texture::Texture2dArray;
use glium::uniforms::*;
use glium::index::*;
use glium::glutin::event_loop::*;

use zodiac_rendering::rendering::*;

use crate::primitives::*;
use crate::shaders::*;
use crate::display::*;
use crate::fonts::*;
use crate::matrices::*;

pub struct GliumRenderer {
    display: Display,
    shader_program: Program,
    font_array: Texture2dArray,
    vertex_buffer: VertexBuffer::<RenderPrimitive>,
    resolution: [f32;2]
}

impl GliumRenderer {
    pub fn new(event_loop: &EventLoop<()>) -> Result<Self, RendererError> {
        let display = create_display(event_loop).map_err(|_|RendererError::FailedToDisplayWindow)?;
        let shader_program = create_shader_program(&display).map_err(|_|RendererError::FailedToCreateShaders)?;
        let font_array = create_font_array(&display).map_err(|_|RendererError::FailedToLoadFont)?;
        let framebuffer_dimensions = display.get_framebuffer_dimensions();
        let vertex_buffer = VertexBuffer::<RenderPrimitive>::empty_dynamic(&display, 16384).map_err(|_|RendererError::BufferCreationError)?;
        
        Ok(Self {
            display,
            shader_program,
            font_array,
            resolution: [framebuffer_dimensions.0 as f32, framebuffer_dimensions.1 as f32],
            vertex_buffer
        })
    }
      
    fn queue_primitive_for_render(&mut self, index: usize, to_queue: RenderPrimitive) {
        self.vertex_buffer.map_write().set(index, to_queue)
    }
}

impl Renderer for GliumRenderer {
    fn queue_rectangle_for_render(
        &mut self,
        index: usize,
        position: [u16; 2],
        dimensions: [u16; 2],
        inner_colour: [f32; 4],
        outer_colour: [f32; 4],
        stroke_width: f32,
        corner_radii: [f32; 4]) {
        self.queue_primitive_for_render(
            index,
            RenderPrimitive::rectangle(
                position,
                dimensions,
                inner_colour,
                outer_colour,
                stroke_width,
                corner_radii));
    }

    fn queue_circle_for_render(
        &mut self,
        index: usize,
        position: [u16; 2],
        radius: u16,
        inner_colour: [f32; 4],
        outer_colour: [f32; 4],
        stroke_width: f32) {
        self.queue_primitive_for_render(
            index,
            RenderPrimitive::circle(
                position,
                radius,
                inner_colour,
                outer_colour,
                stroke_width));
    }
    
    fn queue_text_for_render(
        &mut self,
        index: usize,
        position: [u16; 2],
        dimensions: [u16; 2],
        colour: [f32; 4],
        glyph_index: u16) {
        self.queue_primitive_for_render(
            index,
            RenderPrimitive::text(
                position,
                dimensions,
                colour,
                glyph_index));
    }

    fn render(&mut self) -> Result<(), RendererError> {
        let indices = NoIndices(glium::index::PrimitiveType::Points);

        let params = glium::DrawParameters {
            blend: glium::Blend::alpha_blending(),
            .. Default::default()
        };

        let mut target = self.display.draw();

        let (width, height) = target.get_dimensions();

        let uniforms = uniform! {
            uResolution: self.resolution,
            uCamera: orthographic_camera_matrix(self.resolution[0] as u32, self.resolution[1] as u32),
            uView: orthographic_view_matrix(width, height, self.resolution[0] as u32, self.resolution[1] as u32),
            font_buffer: self.font_array.sampled().magnify_filter(MagnifySamplerFilter::Linear)
        };

        target.clear_color(0.3, 0.3, 0.5, 1.0);
        target.draw(&self.vertex_buffer, &indices, &self.shader_program, &uniforms, &params).map_err(|_|RendererError::DrawError)?;
        target.finish().map_err(|_|RendererError::BufferSwapError)?;
        Ok(())
    }
}