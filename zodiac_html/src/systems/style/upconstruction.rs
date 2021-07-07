use log::info;
use legion::*;
use zodiac::*;
use crate::borders::*;


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