use log::*;
use glutin::event_loop::*;
use pathfinder_canvas::*;
use zodiac_entities::*;
use zodiac_rendering::*;

use crate::*;
use crate::display::*;

const PI2:f32 = std::f64::consts::PI as f32 * 2.0;

pub fn create_pathfinder_renderer(event_loop: &EventLoop<()>) -> Result<PathFinderRenderer, RendererError> {
    PathFinderRenderer::new(event_loop)
}

pub struct PathFinderRenderer {
    display: Display,
    primitives: Vec::<RenderPrimitive>
}

impl Renderer for PathFinderRenderer {
    fn get_window_dimensions(&self) -> Dimensions {
        self.display.get_window_dimensions().into()
    }
}

impl PathFinderRenderer {
    pub fn new(event_loop: &EventLoop<()>) -> Result<Self, RendererError> {
        let display = create_display(event_loop,  WrappedDimensions::from((640, 480)));
        
        let renderer = Self {
            display: display,
            primitives: vec!()
        };

        Ok(renderer)
    }

    pub fn set_primitives(&mut self, to_set: Vec::<RenderPrimitive>) -> Result<(), RendererError> {
        self.primitives = to_set;
        Ok(())
    }

    pub fn reset(&mut self, size: WrappedDimensions) {
        self.display.reset_renderer(size);
    }
  
    pub fn render(&mut self) -> Result<(), RendererError> {  
        let draw_frame_start = std::time::Instant::now();
        let display = &mut self.display;
        let primitives = &self.primitives;
        
        display.render_canvas(|canvas| {
            for primitive in primitives {
                match primitive.definition {
                    RenderPrimitiveDefinition::Rectangle(position, dimensions, inner_colour, outer_colour, stroke_width) => {
                        canvas.set_fill_style(inner_colour);
                        canvas.fill_rect(RectF::new(position, dimensions));
                        
                        canvas.set_line_width(stroke_width);
                        canvas.set_stroke_style(outer_colour);
                        canvas.stroke_rect(RectF::new(position, dimensions));
                    
                    }
                    RenderPrimitiveDefinition::Circle(position, dimensions, inner_colour, outer_colour, stroke_width) => {
                        let mut path = Path2D::new();
                        canvas.set_fill_style(inner_colour);
                        path.ellipse(position, dimensions, 0.0, 0.0, PI2);
                        path.close_path();
                        canvas.fill_path(path, FillRule::Winding);
                        
                        let mut path = Path2D::new();
                        canvas.set_line_width(stroke_width);
                        canvas.set_stroke_style(outer_colour);
                        path.ellipse(position, dimensions, 0.0, 0.0, PI2);
                        path.close_path();
                        canvas.stroke_path(path);
        
    
                    }
                    RenderPrimitiveDefinition::Glyph(_, _, _, _) => {
                    }
                }
            }
        });

        display.swap_buffers();

        let draw_time = std::time::Instant::now() - draw_frame_start;
        debug!("frame draw time: {:?}", draw_time);
        
        Ok(())
    }
}