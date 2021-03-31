use glium::*;

use glium::index::*;
use glium::glutin::event_loop::*;

use zodiac_rendering::*;

use crate::primitives::*;
use crate::shaders::*;
use crate::display::*;

pub struct GliumRenderer {
    display: Display,
    shader_program: Program,
    vertex_buffer: VertexBuffer::<RenderPrimitive>
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
        let vertex_buffer = VertexBuffer::<RenderPrimitive>::empty_dynamic(&display, 16384).map_err(|_|RendererError::BufferCreationError)?;
        Ok(Self {
            display,
            shader_program,
            vertex_buffer
        })
    }
      
    fn queue_primitive_for_render(&mut self, index: usize, to_queue: RenderPrimitive) {
        println!("Rendering: {:?}", to_queue);
        self.vertex_buffer.map_write().set(index, to_queue)
    }
}

impl Renderer for GliumRenderer {
    fn get_window_dimensions(&self) -> (u32, u32) {
        self.display.get_framebuffer_dimensions()
    }

    fn queue_rectangle_for_render(
        &mut self,
        index: usize,
        position: [u16; 2],
        dimensions: [u16; 2],
        inner_colour: [f32; 4],
        outer_colour: [f32; 4],
        stroke_width: f32,
        corner_radii: [u16; 4]) {
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
    
    fn queue_glyph_for_render(
        &mut self,
        index: usize,
        position: [u16; 2],
        dimensions: [u16; 2],
        colour: [f32; 4],
        glyph_index: u16) {
        self.queue_primitive_for_render(
            index,
            RenderPrimitive::glyph(
                position,
                dimensions,
                colour,
                glyph_index));
    }

    fn render(&mut self) -> Result<(), RendererError> {
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

        println!("width: {}, height: {}", width, height);

        let uniforms = uniform! {
            uResolution: [width as f32, height as f32]
        };

        target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
        target.draw(&self.vertex_buffer, &indices, &self.shader_program, &uniforms, &params).map_err(|_|RendererError::DrawError)?;
        target.finish().map_err(|_|RendererError::BufferSwapError)?;
        Ok(())
    }
}