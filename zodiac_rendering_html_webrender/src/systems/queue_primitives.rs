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
#[filter(component::<Style>())]
pub fn queue_render_primitives(
    entity: &Entity, 
    layout_change: &LayoutChange, 
    id: &ComponentId,
    background_colour: &BackgroundColour,
    border: &FullBorder,
    command_buffer: &mut CommandBuffer) {

        info!("queuing primitive for {:?}", entity);

        let primitive = RenderPrimitive {
            id: id.into(),
            dimensions: WrappedLayoutChange::from(layout_change).into(),
            border: WrappedBorder::from(border).into(),
            background_colour: ColourF::from(background_colour).into(),
            is_interactive: true,
        };

        info!("queued: {:?}", primitive);

        command_buffer.add_component(*entity, primitive);
    }