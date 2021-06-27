use legion::*;
use legion::systems::*;
use log::info;
use zodiac::*;
use zodiac_html::*;
use crate::borders::*;
use crate::colours::*;
use crate::dimensions::*;
use crate::render_primitive::*;

#[system(for_each)]
#[filter(component::<Renderable>())] 
#[filter(!component::<RenderPrimitive>())] 
pub fn add_render_primitives(
    entity: &Entity,  
    id: &ComponentId,
    command_buffer: &mut CommandBuffer
) {
    info!("adding primitive for {:?}", entity);
    command_buffer.add_component(*entity, RenderPrimitive::from(id));
}

#[system(for_each)]
#[filter(component::<Renderable>())]
#[filter(component::<LayoutChange>())]
pub fn layout_render_primitives(
    entity: &Entity,
    layout: &ResolvedLayoutBox,
    primitive: &mut RenderPrimitive
) {
    info!("layout primitive for {:?}", entity);
    primitive.dimensions = WrappedLayout::from(layout).into();
}

#[system(for_each)]
#[filter(component::<Renderable>())]
#[filter(component::<Rebuild>())]
pub fn rebuild_render_primitives(
    entity: &Entity, 
    background_colour: &BackgroundColour,
    border: &FullBorder,
    primitive: &mut RenderPrimitive) {
        info!("rebuilding primitive for {:?}", entity);
        primitive.border = WrappedBorder::from(border).into();
        primitive.background_colour = ColourF::from(background_colour).into();
    }