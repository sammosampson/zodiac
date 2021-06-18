use legion::*;
use legion::world::*;
use log::info;
use zodiac::*;

use crate::rendering::HtmlWebRenderRenderer;
use crate::render_primitive::RenderPrimitive;

#[system(simple)]
#[read_component(Renderable)]
#[read_component(LayoutChange)]
#[read_component(Rebuild)]
#[read_component(RenderPrimitive)]
pub fn render_primitives(
    world: &mut SubWorld,
    #[resource] renderer: &mut HtmlWebRenderRenderer) {
    
    if <(&Renderable, &LayoutChange)>::query().iter(world).count() == 0 {
        if <&Rebuild>::query().iter(world).count() == 0 {
            return;
        }
    }
    let primitives: Vec::<RenderPrimitive> = <&RenderPrimitive>::query()
        .iter(world)
        .map(|primitive| *primitive)
        .collect();
     
    info!("rendering primitives: {:?}", primitives.len());

    renderer.render(primitives);
}