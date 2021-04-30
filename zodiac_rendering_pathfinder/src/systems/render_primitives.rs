use legion::*;
use legion::world::SubWorld;
use shrev::*;
use zodiac_entities::*;

use crate::*;

#[system(simple)]
#[read_component(Renderable)]
#[read_component(LayoutChange)]
#[read_component(Rebuild)]
#[read_component(RenderPrimitive)]
pub fn render_primitives(
    world: &mut SubWorld,
    #[resource] renderer: &mut PathFinderRenderer,
    #[resource] event_channel: &mut EventChannel::<SystemEvent>,
    #[resource] event_readers: &mut PathFinderEventReaderRegistry) {
    
    for event in event_channel.read(&mut event_readers.render_primitives) {
        match event {
            SystemEvent::Window(SystemWindowEventType::RootWindowResize(dimensions)) => 
                renderer.reset(WrappedDimensions::from(dimensions)),
            _ => {}
        }
    }

    if <(&Renderable, &LayoutChange)>::query().iter(world).count() == 0 {
        if <&Rebuild>::query().iter(world).count() == 0 {
            return;
        }
    }

    let primitives: Vec::<RenderPrimitive> = <&RenderPrimitive>::query()
        .iter(world)
        .map(|primitive| primitive.clone())
        .collect();
       
    renderer.render(primitives).unwrap();
}