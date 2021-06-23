use legion::*;
use legion::systems::*;
use log::info;
use zodiac::*;
use crate::*;
use crate::borders::*;
use crate::testing::*;

#[system(for_each)]
#[filter(component::<Style>())]
pub fn queue_render_primitives(
    entity: &Entity, 
    id: &ComponentId,
    layout_change: &LayoutChange, 
    border: &FullBorder,
    background_colour: &BackgroundColour,
    command_buffer: &mut CommandBuffer) {

        info!("queuing primitive for {:?}", entity);

        let primitive = RenderPrimitive::from((id, layout_change, border, background_colour));

        info!("queued: {:?}", primitive);

        command_buffer.add_component(*entity, primitive);
    }