use glutin::*;
use glutin::window::*;
use glutin::window::WindowBuilder;
use glutin::dpi::*;
use glutin::event_loop::*;
use pathfinder_geometry::vector::*;
use pathfinder_gl::*;
use pathfinder_resources::embedded::*;
use pathfinder_renderer::gpu::options::*;
use pathfinder_renderer::gpu::renderer::*;
use pathfinder_canvas::*;
use pathfinder_renderer::concurrent::scene_proxy::*;
use pathfinder_renderer::options::*;
use pathfinder_renderer::concurrent::rayon::*;

use crate::WrappedDimensions;

pub fn create_display(event_loop: &EventLoop<()>, physical_window_size: WrappedDimensions) -> Display {
    Display::new(event_loop, physical_window_size)
}
pub struct Display {
    context: ContextWrapper<PossiblyCurrent, Window>,
    renderer: Renderer<GLDevice>,
    physical_window_size: WrappedDimensions
}

impl Display {
    pub fn new(event_loop: &EventLoop<()>, physical_window_size: WrappedDimensions) -> Display {
        let window_builder = WindowBuilder::new()
            .with_inner_size(PhysicalSize::<u32>::from(physical_window_size));

        let gl_context = ContextBuilder::new()
            .with_gl(GlRequest::Latest)
            .with_gl_profile(GlProfile::Core)
            .build_windowed(window_builder, &event_loop)
            .unwrap();

        let gl_context = unsafe { gl_context.make_current().unwrap() };
        gl::load_with(|name| gl_context.get_proc_address(name) as *const _);

        let device = GLDevice::new(GLVersion::GL3, 0);

        let options = RendererOptions {
            background_color: Some(ColorF::black()),
            ..RendererOptions::default()
        };
    
        let renderer = Renderer::new(
            device, 
            &EmbeddedResourceLoader,
            DestFramebuffer::full_window(Vector2I::from(physical_window_size)), options);
            
        Display {
            context: gl_context,
            renderer,
            physical_window_size
        }
    }

    pub fn reset_renderer(&mut self, size: WrappedDimensions) {
        self.physical_window_size = size;        
        self.renderer.replace_dest_framebuffer(
            DestFramebuffer::full_window(
                Vector2I::from(self.physical_window_size)));
    }

    pub fn render_canvas<F>(&mut self, render_func: F) 
        where F: Fn(&mut CanvasRenderingContext2D) -> () {
        
        let font_context = CanvasFontContext::from_system_source();

        let mut canvas = Canvas
            ::new(Vector2F::from(self.physical_window_size))
            .get_context_2d(font_context);

        render_func(&mut canvas);
        
        let scene = SceneProxy::from_scene(canvas.into_canvas().into_scene(), RayonExecutor);
        scene.build_and_render(&mut self.renderer, BuildOptions::default());
    }

    pub fn swap_buffers(&self) {
        self.context.swap_buffers().unwrap();
    }

    pub fn get_window_dimensions(&self) -> WrappedDimensions {
        WrappedDimensions::from(self.context.window().inner_size())
    }
}