use legion::*;
use legion::systems::*;
use zodiac::*;
use crate::borders::*;

#[system(for_each)]
#[filter(component::<Rebuild>())]
pub fn compose_border_style(
    command_buffer: &mut CommandBuffer, 
    entity: &Entity,
    top_style: &BorderTopStyle,
    left_style: &BorderLeftStyle,
    bottom_style: &BorderBottomStyle,
    right_style: &BorderRightStyle,
    style: &BorderStyle
) {
    if !style.is_set() {
        command_buffer.add_component::<BorderStyle>(
            *entity, 
            BorderStyle::from((top_style, left_style, bottom_style, right_style)));
    }
    command_buffer.remove_component::<BorderTopStyle>(*entity);
    command_buffer.remove_component::<BorderLeftStyle>(*entity);
    command_buffer.remove_component::<BorderBottomStyle>(*entity);
    command_buffer.remove_component::<BorderRightStyle>(*entity);
}

#[system(for_each)]
#[filter(component::<Rebuild>())]
pub fn compose_border(
    command_buffer: &mut CommandBuffer, 
    entity: &Entity,
    colour: &BorderColour,
    width: &BorderWidth,
    style: &BorderStyle,
    radius: &BorderRadius,
    border: &Border
) {
    if !border.is_set() {
        command_buffer.add_component::<Border>(*entity, Border::from((colour, width, style, radius)));
    }
    command_buffer.remove_component::<BorderColour>(*entity);
    command_buffer.remove_component::<BorderWidth>(*entity);
    command_buffer.remove_component::<BorderStyle>(*entity);
    command_buffer.remove_component::<BorderRadius>(*entity);
}