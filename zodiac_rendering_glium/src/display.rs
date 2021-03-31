use glium::*;
use glium::glutin::event_loop::*;
use glium::glutin::window::*;
use glium::glutin::*;
use glium::backend::glutin::*;

pub fn create_display(event_loop: &EventLoop<()>) -> Result<Display, DisplayCreationError> {
    Display::new(WindowBuilder::new(), ContextBuilder::new().with_depth_buffer(24), event_loop)
}