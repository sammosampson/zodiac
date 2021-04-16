use legion::*;
use log::{debug};
use legion::world::*;
use zodiac_entities::*;

use crate::{ GliumRenderer, primitives::RenderPrimitive };

#[system(simple)]
#[read_component(Renderable)]
#[read_component(LayoutChange)]
#[read_component(Rebuild)]
#[read_component(RenderPrimitive)]
pub fn render_primitives(
    world: &mut SubWorld,
    #[resource] renderer: &mut GliumRenderer) {
    
    if <(&Renderable, &LayoutChange)>::query().iter(world).count() == 0 {
        if <&Rebuild>::query().iter(world).count() == 0 {
            return;
        }
    }

    let primitives: Vec::<RenderPrimitive> = <&RenderPrimitive>::query()
        .iter(world)
        .map(|primitive| *primitive)
        .collect();
       
    renderer.set_primitives(&primitives).unwrap();
    
    renderer.render().unwrap();

    debug!("primitives rendered {:?}", primitives.len());
}