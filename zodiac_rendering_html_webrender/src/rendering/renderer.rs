use glium::glutin::*;
use zodiac::*;
use crate::render_primitive::*;

use super::window::*;
use super::window_renderer::*;

pub fn create_webrender_renderer() -> HtmlWebRenderRenderer {
    HtmlWebRenderRenderer::default()
}

#[derive(Default)]
pub struct HtmlWebRenderRenderer {
    window_renderers: Vec<HtmlWebRenderWindowRenderer>
}

impl HtmlWebRenderRenderer {
    pub fn add_window(
        &mut self,
        event_loop: &event_loop::EventLoop<()>,
        window: RenderableGliumWindow) -> Result<(), RendererError> {
        let renderer = HtmlWebRenderWindowRenderer::new(window, event_loop)?;
        self.window_renderers.push(renderer);
        Ok(())
    }

    pub fn render(&mut self, primitives: Vec::<RenderPrimitive>) {
        if let Some(window_renderer) = self.window_renderers.last_mut() {
            window_renderer.render(primitives);
        }
    }
}

impl zodiac::Renderer for HtmlWebRenderRenderer {
    fn get_window_dimensions(&self) -> Dimensions {
        if let Some(window_renderer) = self.window_renderers.last() {
            let size = window_renderer.inner_window_size();
            return Dimensions::new(size.width as u16, size.height as u16);
        }
        Dimensions::default()
    }
}