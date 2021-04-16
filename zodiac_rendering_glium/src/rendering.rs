use log::{debug};

use glium::*;
use glium::index::*;
use glium::glutin::event_loop::*;
use zodiac_entities::*;
use zodiac_rendering::*;

use crate::primitives::*;
use crate::shaders::*;
use crate::display::*;

pub fn create_glium_renderer(event_loop: &EventLoop<()>) -> Result<GliumRenderer, RendererError> {
    GliumRenderer::new(event_loop)
}

pub struct GliumRenderer {
    display: Display,
    shader_program: Program,
    vertex_buffer: VertexBuffer::<RenderPrimitive>
}

impl Renderer for GliumRenderer {
    fn get_window_dimensions(&self) -> Dimensions {
        Dimensions::from(self.display.get_framebuffer_dimensions())
    }
}

fn get_shader_error_message(from: ProgramCreationError) -> String {
    match from {
        ProgramCreationError::CompilationError(message, _) => message,
        ProgramCreationError::LinkingError(message) => message,
        _ => String::from("Unknown shader program error")
    }
}

impl GliumRenderer {
    pub fn new(event_loop: &EventLoop<()>) -> Result<Self, RendererError> {
        let display = create_display(event_loop).map_err(|_|RendererError::FailedToDisplayWindow)?;
        let shader_program = create_shader_program(&display).map_err(|e|RendererError::FailedToCreateShaders(get_shader_error_message(e)))?;
        let vertex_buffer = VertexBuffer::<RenderPrimitive>::empty_dynamic(&display, 0).map_err(|_|RendererError::BufferCreationError)?;
        Ok(Self {
            display,
            shader_program,
            vertex_buffer
        })
    }

    pub fn set_primitives(&mut self, to_set: &Vec::<RenderPrimitive>) -> Result<(), RendererError>  {
        self.vertex_buffer = VertexBuffer::<RenderPrimitive>::dynamic(&self.display, to_set).map_err(|_|RendererError::BufferCreationError)?;
        Ok(())
    }
      
    pub fn render(&mut self) -> Result<(), RendererError> {
        
        let draw_frame_start = std::time::Instant::now();
    
        let indices = NoIndices(glium::index::PrimitiveType::Points);

        let params = glium::DrawParameters {
            blend: glium::Blend::alpha_blending(),
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            .. Default::default()
        };

        let mut target = self.display.draw();
        let (width, height) = target.get_dimensions();

        //debug!("width: {}, height: {}", width, height);

        let uniforms = uniform! {
            uResolution: [width as f32, height as f32]
        };

        target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
        target.draw(&self.vertex_buffer, &indices, &self.shader_program, &uniforms, &params).map_err(|_|RendererError::DrawError)?;
        target.finish().map_err(|_|RendererError::BufferSwapError)?;
        
        // TODO: log timing better here
        let draw_time = std::time::Instant::now() - draw_frame_start;
        debug!("frame draw time: {:?}", draw_time);
        
        Ok(())
    }
}