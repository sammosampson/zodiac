use log::info;
use legion::*;
use legion::systems::*;
use zodiac::*;
use crate::Colour;
use crate::Size;
use crate::borders::*;

#[system(for_each)]
#[filter(component::<Rebuild>())]
pub fn deconstruct_border(
    command_buffer: &mut CommandBuffer, 
    entity: &Entity,
    border: &Border
) {
    if border.is_set() {
        info!("deconstruct_border: {:?}", border);
        let (colour, width, style) = border.into();
        command_buffer.remove_component::<Border>(*entity);
        command_buffer.add_component(*entity, BorderColour::from(colour));
        command_buffer.add_component(*entity, BorderWidth::from(width));
        command_buffer.add_component(*entity, BorderStyle::from(style));
    }
}

#[system(for_each)]
#[filter(component::<Rebuild>())]
pub fn deconstruct_border_colour(
    command_buffer: &mut CommandBuffer, 
    entity: &Entity,
    colour: &BorderColour
) {
    if colour.is_set() {
        info!("deconstruct_border_colour");
        let colour: Colour = colour.into();
        command_buffer.remove_component::<BorderStyle>(*entity);   
        command_buffer.add_component(*entity, BorderTopColour::from(colour));
        command_buffer.add_component(*entity, BorderLeftColour::from(colour));
        command_buffer.add_component(*entity, BorderBottomColour::from(colour));
        command_buffer.add_component(*entity, BorderRightColour::from(colour));
    }
}

#[system(for_each)]
#[filter(component::<Rebuild>())]
pub fn deconstruct_border_width(
    command_buffer: &mut CommandBuffer, 
    entity: &Entity,
    width: &BorderWidth
) {
    if width.is_set() {
        info!("deconstruct_border_width");
        let width: Size = width.into();
        command_buffer.remove_component::<BorderStyle>(*entity);   
        command_buffer.add_component(*entity, BorderTopWidth::from(width));
        command_buffer.add_component(*entity, BorderLeftWidth::from(width));
        command_buffer.add_component(*entity, BorderBottomWidth::from(width));
        command_buffer.add_component(*entity, BorderRightWidth::from(width));
    }
}

#[system(for_each)]
#[filter(component::<Rebuild>())]
pub fn deconstruct_border_style(
    command_buffer: &mut CommandBuffer, 
    entity: &Entity,
    style: &BorderStyle
) {
    if style.is_set() {
        info!("deconstruct_border_style");
        let style: BorderStyles = style.into();
        command_buffer.remove_component::<BorderStyle>(*entity);   
        command_buffer.add_component(*entity, BorderTopStyle::from(style));
        command_buffer.add_component(*entity, BorderLeftStyle::from(style));
        command_buffer.add_component(*entity, BorderBottomStyle::from(style));
        command_buffer.add_component(*entity, BorderRightStyle::from(style));
    }
}

#[system(for_each)]
#[filter(component::<Rebuild>())]
pub fn upconstruct_border_top(
    command_buffer: &mut CommandBuffer, 
    entity: &Entity,
    style: &BorderTopStyle,
    width: &BorderTopWidth,
    colour: &BorderTopColour,
    top: &BorderTop,
) {
    if !top.is_set() {
        info!("upconstruct_border_top");
        let width: Size = width.into();
        command_buffer.add_component(*entity, BorderTop::from((width, style.into(), colour.into())));
        command_buffer.remove_component::<BorderTopStyle>(*entity);
        command_buffer.remove_component::<BorderTopWidth>(*entity);
        command_buffer.remove_component::<BorderTopColour>(*entity);
    }
}

#[system(for_each)]
#[filter(component::<Rebuild>())]
pub fn upconstruct_border_left(
    command_buffer: &mut CommandBuffer, 
    entity: &Entity,
    style: &BorderLeftStyle,
    width: &BorderLeftWidth,
    colour: &BorderLeftColour,
    left: &BorderLeft,
) {
    if !left.is_set() {
        info!("upconstruct_border_left");
        let width: Size = width.into();
        command_buffer.add_component(*entity, BorderLeft::from((width, style.into(), colour.into())));
        command_buffer.remove_component::<BorderLeftStyle>(*entity);
        command_buffer.remove_component::<BorderLeftWidth>(*entity);
        command_buffer.remove_component::<BorderLeftColour>(*entity);
    }
}

#[system(for_each)]
#[filter(component::<Rebuild>())]
pub fn upconstruct_border_bottom(
    command_buffer: &mut CommandBuffer, 
    entity: &Entity,
    style: &BorderBottomStyle,
    width: &BorderBottomWidth,
    colour: &BorderBottomColour,
    top: &BorderBottom,
) {
    if !top.is_set() {
        info!("upconstruct_border_bottom");
        let width: Size = width.into();
        command_buffer.add_component(*entity, BorderBottom::from((width, style.into(), colour.into())));
        command_buffer.remove_component::<BorderBottomStyle>(*entity);
        command_buffer.remove_component::<BorderBottomWidth>(*entity);
        command_buffer.remove_component::<BorderBottomColour>(*entity);
    }
}

#[system(for_each)]
#[filter(component::<Rebuild>())]
pub fn upconstruct_border_right(
    command_buffer: &mut CommandBuffer, 
    entity: &Entity,
    style: &BorderRightStyle,
    width: &BorderRightWidth,
    colour: &BorderRightColour,
    right: &BorderRight,
) {
    if !right.is_set() {
        info!("upconstruct_border_right");
        let width: Size = width.into();
        command_buffer.add_component(*entity, BorderRight::from((width, style.into(), colour.into())));
        command_buffer.remove_component::<BorderRightStyle>(*entity);
        command_buffer.remove_component::<BorderRightWidth>(*entity);
        command_buffer.remove_component::<BorderRightColour>(*entity);
    }
}

#[system(for_each)]
#[filter(component::<Rebuild>())]
pub fn compose_full_border(
    command_buffer: &mut CommandBuffer, 
    entity: &Entity,
    radius: &BorderRadius,
    top: &BorderTop,
    left: &BorderLeft,
    bottom: &BorderBottom,
    right: &BorderRight
) {
    let full_border = FullBorder::from((top, left, bottom, right, radius));
    info!("composing full border {:?}", full_border);
    command_buffer.add_component(*entity, full_border);
}