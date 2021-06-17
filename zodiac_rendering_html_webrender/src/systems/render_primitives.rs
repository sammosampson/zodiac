use legion::{Resources, world::*};

use crate::rendering::*;

pub fn render_primitives(_: &mut World, resources: &mut Resources) {
    let mut renderer = resources.get_mut::<HtmlWebRenderRenderer>().unwrap();
    renderer.render();
}