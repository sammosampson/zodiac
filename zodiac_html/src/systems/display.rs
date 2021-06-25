use legion::*;
use zodiac::*;

use crate::layout::*;

#[system(for_each)]
#[filter(component::<Rebuild>())]
pub fn layout_display(
    display: &Display,
    _margin: &Margin,
    layout_box: &mut IncumbentLayoutBox) {
        match display.into() {
            DisplayTypes::Block => { 
                layout_box.set((LayoutDirection::Vertical, LayoutOffsetRect::default(), LayoutDimensions::default()));
                todo!()
                // add conversions from margin to LayoutOffsetRect and set the LayoutDimensions
            },
            DisplayTypes::Inline => { 
                layout_box.set((LayoutDirection::Horizontal, LayoutOffsetRect::default(), LayoutDimensions::default()));
                todo!()
                // as above
            },
            DisplayTypes::None => { 
                layout_box.set((LayoutDirection::None, LayoutOffsetRect::default(), LayoutDimensions::default()));
                todo!()
                // as above
            },
        }

}