use log::info;
use legion::*;
use zodiac::*;
use crate::Colour;
use crate::Size;
use crate::borders::*;

#[system(for_each)]
#[filter(component::<Rebuild>())]
pub fn deconstruct_border(
    border: &Border,
    colour: &mut BorderColour,
    width: &mut BorderWidth,
    style: &mut BorderStyle,
) {
    if border.is_set() {
        info!("deconstruct_border: {:?}", border);
        let (border_width, border_style, border_colour) = border.into();
        colour.set(border_colour);
        width.set(border_width);
        style.set(border_style);
    }
}

#[system(for_each)]
#[filter(component::<Rebuild>())]
pub fn deconstruct_border_colour(
    colour: &BorderColour,
    top: &mut BorderTopColour,
    left: &mut BorderLeftColour,
    bottom: &mut BorderBottomColour,
    right: &mut BorderRightColour,
) {
    if colour.is_set() {
        info!("deconstruct_border_colour");
        let colour: Colour = colour.into();
        top.set(colour);
        left.set(colour);
        bottom.set(colour);
        right.set(colour);
    }
}

#[system(for_each)]
#[filter(component::<Rebuild>())]
pub fn deconstruct_border_width(
    width: &BorderWidth,
    top: &mut BorderTopWidth,
    left: &mut BorderLeftWidth,
    bottom: &mut BorderBottomWidth,
    right: &mut BorderRightWidth,
) {
    if width.is_set() {
        info!("deconstruct_border_width");
        let width: Size = width.into();
        top.set(width);
        left.set(width);
        bottom.set(width);
        right.set(width);
    }
}

#[system(for_each)]
#[filter(component::<Rebuild>())]
pub fn deconstruct_border_style(
    style: &BorderStyle,
    top: &mut BorderTopStyle,
    left: &mut BorderLeftStyle,
    bottom: &mut BorderBottomStyle,
    right: &mut BorderRightStyle,
) {
    if style.is_set() {
        info!("deconstruct_border_style");
        let style: BorderStyles = style.into();
        top.set(style);
        left.set(style);
        bottom.set(style);
        right.set(style);
    }
}

#[system(for_each)]
#[filter(component::<Rebuild>())]
pub fn upconstruct_border_top(
    style: &BorderTopStyle,
    width: &BorderTopWidth,
    colour: &BorderTopColour,
    top: &mut BorderTop,
) {
    if !top.is_set() {
        info!("upconstruct_border_top");
        top.set(BorderValues::from((width.into(), style.into(), colour.into())));
    }
}

#[system(for_each)]
#[filter(component::<Rebuild>())]
pub fn upconstruct_border_left(
    style: &BorderLeftStyle,
    width: &BorderLeftWidth,
    colour: &BorderLeftColour,
    left: &mut BorderLeft,
) {
    if !left.is_set() {
        info!("upconstruct_border_left");
        left.set(BorderValues::from((width.into(), style.into(), colour.into())));
    }
}

#[system(for_each)]
#[filter(component::<Rebuild>())]
pub fn upconstruct_border_bottom(
    style: &BorderBottomStyle,
    width: &BorderBottomWidth,
    colour: &BorderBottomColour,
    bottom: &mut BorderBottom,
) {
    if !bottom.is_set() {
        info!("upconstruct_border_bottom");
        bottom.set(BorderValues::from((width.into(), style.into(), colour.into())));
    }
}

#[system(for_each)]
#[filter(component::<Rebuild>())]
pub fn upconstruct_border_right(
    style: &BorderRightStyle,
    width: &BorderRightWidth,
    colour: &BorderRightColour,
    right: &mut BorderRight,
) {
    if !right.is_set() {
        info!("upconstruct_border_right");
        right.set(BorderValues::from((width.into(), style.into(), colour.into())));
    }
}

#[system(for_each)]
#[filter(component::<Rebuild>())]
pub fn compose_full_border(
    radius: &BorderRadius,
    top: &BorderTop,
    left: &BorderLeft,
    bottom: &BorderBottom,
    right: &BorderRight,
    full_border: &mut FullBorder
) {
    info!("composing full border {:?}", full_border);
    full_border.set((top, left, bottom, right, radius));
}