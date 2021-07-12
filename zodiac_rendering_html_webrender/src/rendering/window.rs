use std::rc::Rc;
use log::info;
use glium::glutin::*;
use gleam::*;


pub struct RenderableGliumWindow {
    gl_context: ContextWrapper<PossiblyCurrent, window::Window>
}

impl RenderableGliumWindow {
    pub fn new(
        event_loop: &event_loop::EventLoop<()>,
        title: Option<String>,
        dimensions: Option<glium::glutin::dpi::LogicalSize<u32>>,
        is_maximised: bool,
        has_decorations: bool) -> Self {

        let mut window_builder = window::WindowBuilder::new()
            .with_transparent(true)
            .with_decorations(has_decorations)
            .with_maximized(is_maximised);

        if let Some(title) = title {
            window_builder = window_builder.with_title(title);
        }

        if let Some(dimensions) = dimensions {
            window_builder = window_builder.with_inner_size(dimensions);
        }

        if is_maximised {
            window_builder = window_builder;
        }
        
        let gl_context = ContextBuilder::new()
            .with_gl(GlRequest::GlThenGles {
                opengl_version: (3, 2),
                opengles_version: (3, 0),
            })
            .build_windowed(window_builder, event_loop)
            .unwrap();

        let gl_context = unsafe { 
            gl_context.make_current().unwrap() 
        };
    
        Self {
            gl_context
        }
    }

    pub fn swap_buffers(&self) {
        self.gl_context.swap_buffers().unwrap();
    }

    pub fn device_pixel_ratio(&self) -> f32 {
        self.gl_context.window().scale_factor() as f32
    }

    pub fn inner_size(&self) -> dpi::PhysicalSize<u32> {
        self.gl_context
            .window()
            .inner_size()
    }

    pub fn create_gl(&self) -> Rc<dyn gl::Gl> {
        let gl = match self.gl_context.get_api() {
            Api::OpenGl => unsafe {
                gl::GlFns::load_with(
                    |symbol| self.gl_context.get_proc_address(symbol) as *const _
                )
            },
            Api::OpenGlEs => unsafe {
                gl::GlesFns::load_with(
                    |symbol| self.gl_context.get_proc_address(symbol) as *const _
                )
            },
            Api::WebGl => unimplemented!(),
        };

        info!("OpenGL version {}", gl.get_string(gl::VERSION));
        
        gl
    }
}