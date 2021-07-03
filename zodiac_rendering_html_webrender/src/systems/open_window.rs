use legion::*;
use legion::systems::*;
use zodiac_html::*;
use crate::rendering::*;

#[system(for_each)]
#[filter(component::<Window>())]
#[filter(!component::<WindowOpen>())]
pub fn open_window(
    command_buffer: &mut CommandBuffer,
    #[resource] renderer: &mut HtmlWebRenderRenderer,
    #[resource] event_loop: &mut glium::glutin::event_loop::EventLoop<()>,
    entity: &Entity,
    title: Option<&Title>,
    width: Option<&Width>,
    height: Option<&Height>) {

    let title = if let Some(title) = title { Some(title.into()) } else { None };
    let width: Option<Size> = if let Some(width) = width { Some(width.into()) } else { None };
    let height: Option<Size> = if let Some(height) = height { Some(height.into()) } else { None };

    let dimensions = if width != None && height != None {
        Some(glium::glutin::dpi::LogicalSize::<u32>::new(
            width.unwrap().into(), 
            height.unwrap().into()))
    } else {
        None
    };

    let is_maximised = false;
    let has_decorations = true;

    renderer.add_window(&event_loop, RenderableGliumWindow::new(event_loop, title, dimensions, is_maximised, has_decorations)).unwrap();
    command_buffer.add_component(*entity, WindowOpen::default())
}