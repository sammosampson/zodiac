use log::info;
use winit::*;
use gleam::gl;
use zodiac::*;
use crate::notification::*;

pub fn create_webrender_renderer(event_loop: &EventsLoop) -> Result<HtmlWebRenderRenderer, RendererError> {
    HtmlWebRenderRenderer::new(event_loop)
}

#[derive(Default)]
pub struct HtmlWebRenderRenderer {
   
}

impl HtmlWebRenderRenderer {
    pub fn new(event_loop: &EventsLoop) -> Result<Self, RendererError> {
        let window_builder = winit::WindowBuilder::new()
            .with_title("TODO")
            .with_multitouch();

        let windowed_context = glutin::ContextBuilder::new()
            .with_gl(glutin::GlRequest::GlThenGles {
                opengl_version: (3, 2),
                opengles_version: (3, 0),
            })
            .build_windowed(window_builder, event_loop)
            .unwrap();
    
        let windowed_context = unsafe { 
            windowed_context.make_current().unwrap() 
        };

        let gl = match windowed_context.get_api() {
            glutin::Api::OpenGl => unsafe {
                gl::GlFns::load_with(
                    |symbol| windowed_context.get_proc_address(symbol) as *const _
                )
            },
            glutin::Api::OpenGlEs => unsafe {
                gl::GlesFns::load_with(
                    |symbol| windowed_context.get_proc_address(symbol) as *const _
                )
            },
            glutin::Api::WebGl => unimplemented!(),
        };

        info!("OpenGL version {}", gl.get_string(gl::VERSION));
        
        let device_pixel_ratio = windowed_context.window().get_hidpi_factor() as f32;
        
        info!("Device pixel ratio: {}", device_pixel_ratio);
        info!("Loading shaders...");

        let mut debug_flags = 
            webrender::DebugFlags::ECHO_DRIVER_MESSAGES 
            | webrender::DebugFlags::TEXTURE_CACHE_DBG;
    
        let opts = webrender::RendererOptions {
            resource_override_path: None,
            precache_flags: webrender::ShaderPrecacheFlags::EMPTY,
            clear_color: Some(webrender::api::ColorF::new(0.3, 0.0, 0.0, 1.0)),
            debug_flags,
            ..webrender::RendererOptions::default()
        };

        let device_size = {
            let size = windowed_context
                .window()
                .get_inner_size()
                .unwrap()
                .to_physical(device_pixel_ratio as f64);
            webrender::api::units::DeviceIntSize::new(size.width as i32, size.height as i32)
        };
        
        let _notifier = Box::new(Notifier::new(event_loop.create_proxy()));
        
        Ok(Self {})
    }
}

impl zodiac::Renderer for HtmlWebRenderRenderer {
    fn get_window_dimensions(&self) -> Dimensions {
        Dimensions::new(1000, 1000)
    }
}