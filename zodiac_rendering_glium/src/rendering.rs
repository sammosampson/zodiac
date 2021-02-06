use glium::*;

use glium::texture::Texture2dArray;
use glium::uniforms::*;
use glium::index::*;
use glium::vertex::BufferCreationError;
use glium::glutin::event_loop::*;
use glium::backend::glutin::*;
use glium::texture::*;
use crate::primitives::*;
use crate::shaders::*;
use crate::display::*;
use crate::fonts::*;

#[derive(Debug)]
pub enum RendererError {
    FailedToDisplayWindow,
    FailedToCreateShaders,
    FailedToLoadFont,
    BufferSwapError,
    BufferCreationError,
    DrawError
}

impl From<SwapBuffersError> for RendererError {
    fn from(_: SwapBuffersError) -> Self {
        RendererError::BufferSwapError
    }
}

impl From<BufferCreationError> for RendererError {
    fn from(_: BufferCreationError) -> Self {
        RendererError::BufferCreationError
    }
}

impl From<DrawError> for RendererError {
    fn from(_: DrawError) -> Self {
        RendererError::DrawError
    }
}

impl From<DisplayCreationError> for RendererError {
    fn from(_: DisplayCreationError) -> Self {
        RendererError::FailedToDisplayWindow
    }
}

impl From<ProgramCreationError> for RendererError {
    fn from(_: ProgramCreationError) -> Self {
        RendererError::FailedToCreateShaders
    }
}

impl From<TextureCreationError> for RendererError {
    fn from(_: TextureCreationError) -> Self {
        RendererError::FailedToLoadFont
    }
}

pub struct GliumRenderer {
    display: Display,
    shader_program: Program,
    font_array: Texture2dArray,
    resolution: [f32;2]
}

impl GliumRenderer {
    pub fn new(event_loop: &EventLoop<()>) -> Result<Self, RendererError> {
        let display = create_display(event_loop)?;
        let shader_program = create_shader_program(&display)?;
        let font_array = create_font_array(&display)?;
        let framebuffer_dimensions = display.get_framebuffer_dimensions();
        
        Ok(Self {
            display,
            shader_program,
            font_array,
            resolution: [framebuffer_dimensions.0 as f32, framebuffer_dimensions.1 as f32]
        })
    }

    pub fn render(&mut self, primitives: &Vec::<RenderPrimitive>) -> Result<(), RendererError> {
        let vertices = VertexBuffer::dynamic(&self.display, primitives)?;
        let indices = NoIndices(glium::index::PrimitiveType::Points);

        let uniforms = uniform! {
            uResolution: self.resolution,
            font_buffer: self.font_array.sampled().magnify_filter(MagnifySamplerFilter::Linear)
        };

        let params = glium::DrawParameters {
            blend: glium::Blend::alpha_blending(),
            .. Default::default()
        };

        let mut target = self.display.draw();
        target.clear_color(0.3, 0.3, 0.5, 1.0);
        target.draw(&vertices, &indices, &self.shader_program, &uniforms, &params)?;
        target.finish()?;
        Ok(())
    }
}