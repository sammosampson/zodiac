use legion::*;
use zodiac::*;

use crate::layout::*;

#[system(for_each)]
#[filter(component::<Rebuild>())]
pub fn layout_display(
    display: &Display,
    margin: &Margin,
    layout_box: &mut IncumbentLayoutBox) {
        match display.into() {
            DisplayTypes::Block => { 
                layout_box.set((
                    LayoutDirection::Vertical,
                    margin.into(), 
                    LayoutDimensions::new(LayoutDistance::FromParent(1.0), LayoutDistance::FromChildren(1.0))));
            },
            DisplayTypes::Inline => { 
                layout_box.set((
                    LayoutDirection::Horizontal,
                    margin.into(), 
                    LayoutDimensions::new(LayoutDistance::FromChildren(1.0), LayoutDistance::FromChildren(1.0))));
            },
            DisplayTypes::None => { 
                layout_box.set((LayoutDirection::None, LayoutOffsetRect::default(), LayoutDimensions::default()));
            },
        }

}