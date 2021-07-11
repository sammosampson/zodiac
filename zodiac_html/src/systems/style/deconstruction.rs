use log::info;
use legion::*;
use zodiac::*;
use crate::Size;
use crate::borders::*;
use crate::colour::*;

#[system(par_for_each)]
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

#[system(par_for_each)]
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

#[system(par_for_each)]
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

#[system(par_for_each)]
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
