use std::{rc::Rc, sync::mpsc};
use log::{info, debug};
use webrender::api::units::DeviceIntRect;
use glium::glutin::*;
use gleam::*;
use zodiac::*;
use crate::notification::*;
use crate::render_primitive::*;

pub fn create_webrender_renderer(event_loop: &event_loop::EventLoop<()>) -> Result<HtmlWebRenderRenderer, RendererError> {
    HtmlWebRenderRenderer::new(event_loop)
}

pub struct HtmlWebRenderRenderer {
    window: Window,
    renderer: Option<webrender::Renderer>,
    render_api: webrender::api::RenderApi,
    notifier_receiver: mpsc::Receiver<()>,
    document_id: webrender::api::DocumentId
}

impl HtmlWebRenderRenderer {
    pub fn new(event_loop: &event_loop::EventLoop<()>) -> Result<Self, RendererError> {        
        let window = Window::new(event_loop);
         
        info!("Device pixel ratio: {}", window.device_pixel_ratio());

        let size = window.inner_size();

        let client_size = webrender::api::units::DeviceIntSize::new(
            size.width as i32,
            size.height as i32);

        let gl = window.create_gl();
       
        let (notifier_sender, notifier_receiver) = mpsc::channel();
        
        let notifier = Box::new(Notifier::new(
            event_loop.create_proxy(),
            notifier_sender));

        let (renderer, renderer_sender) = webrender::Renderer::new(
            gl,
            notifier,
            webrender::RendererOptions {
                clear_color: Some(webrender::api::ColorF::new(0.0, 0.0, 0.0, 1.0)),
                device_pixel_ratio: window.device_pixel_ratio(),
                ..Default::default()
            },
            None,
            client_size,
        )
        .unwrap();

        let render_api = renderer_sender.create_api();
        let document_id = render_api.add_document(client_size, 0);

        Ok(
            Self { 
                window,
                renderer: Some(renderer),
                render_api,
                notifier_receiver,
                document_id
            }
        )
    }

    pub fn render(&mut self, primitives: Vec::<RenderPrimitive>) {
        let client_size = self.client_size();
        debug!("{:?}", client_size);

        let dpi_scale = self.window.device_pixel_ratio();
        let content_size = client_size.to_f32() / webrender::euclid::Scale::new(dpi_scale);
        debug!("{:?}", content_size);

        debug!("render()");
        let pipeline_id = webrender::api::PipelineId(0, 0);
        let mut builder = webrender::api::DisplayListBuilder::new(pipeline_id);
        let mut transaction = webrender::api::Transaction::new();

        for primitive in primitives {
            self.render_primitive(
                primitive, 
                pipeline_id,
                &mut builder
            );
        }

        transaction.set_display_list(webrender::api::Epoch(0), None, content_size, builder.finalize(), true);
        transaction.set_root_pipeline(pipeline_id);
        transaction.generate_frame();

        self.render_api.set_document_view(
            self.document_id,
            DeviceIntRect::new(webrender::euclid::Point2D::zero(), client_size.to_i32()),
            dpi_scale,
        );
        
        &mut self.render_api.send_transaction(self.document_id, transaction);

        debug!("Awaiting frame draw completion");
        self.notifier_receiver.recv().unwrap();
        debug!("Frame draw complete");

        if let Some(ref mut renderer) = &mut self.renderer {
            renderer.update();
            let _ = renderer.render(client_size.to_i32());
            let _ = renderer.flush_pipeline_info();
        }
        
        self.window.swap_buffers();
    }

    fn render_primitive(
        &self,
        primitive: RenderPrimitive,
        pipeline_id: webrender::api::PipelineId,
        builder: &mut webrender::api::DisplayListBuilder,
    ) {
        let space_and_clip = webrender::api::SpaceAndClipInfo::root_scroll(pipeline_id);

        if primitive.is_interactive || primitive.background_colour.a > 0.0 {
            let clip_id = 
                if let Some(border) = &primitive.border {
                    builder.define_clip(
                        &space_and_clip,
                        primitive.dimensions,
                        vec![webrender::api::ComplexClipRegion::new(
                            primitive.dimensions,
                            border.radius,
                            webrender::api::ClipMode::Clip
                        )],
                    )
                } else {
                    webrender::api::ClipId::root(pipeline_id)
                };
            
            let item_props = webrender::api::CommonItemProperties {
                clip_id,
                clip_rect: primitive.dimensions,
                spatial_id: webrender::api::SpatialId::root_scroll_node(pipeline_id),
                flags: webrender::api::PrimitiveFlags::empty(),
            };

            builder.push_rect(
                &item_props, 
                primitive.dimensions, 
                primitive.background_colour
            );

            builder.push_hit_test(&item_props, (primitive.id, 0));

            if let Some(border) = &primitive.border {
                let common = webrender::api::CommonItemProperties::new(primitive.dimensions, space_and_clip);
                builder.push_border(
                    &common,
                    primitive.dimensions,
                    border.widths,
                    border.details
                );
            }
        }
    }

    fn client_size(&self) -> webrender::euclid::Size2D::<i32, webrender::api::units::DevicePixel> {
        let inner_size = self.window.inner_size();
        webrender::euclid::Size2D::<i32, webrender::api::units::DevicePixel>::new(
            inner_size.width as i32, 
            inner_size.height as i32)
    }
}

impl zodiac::Renderer for HtmlWebRenderRenderer {
    fn get_window_dimensions(&self) -> Dimensions {
        let size = self.window.inner_size();
        Dimensions::new(size.width as u16, size.height as u16)
    }
}

impl Drop for HtmlWebRenderRenderer {
    fn drop(&mut self) {
        info!("Deinit renderer");
        self.renderer.take().unwrap().deinit();
    }
}

pub struct Window {
    gl_context: ContextWrapper<PossiblyCurrent, window::Window>
}

impl Window {
    pub fn new(event_loop: &event_loop::EventLoop<()>) -> Self {
        let window_builder = window::WindowBuilder::new()
            .with_title("TODO")
            .with_decorations(true)
            .with_transparent(true);

        
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