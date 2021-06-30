use legion::*;
use zodiac::*;
use crate::style::*;
use crate::layout::*;

#[system(for_each)]
#[filter(component::<Rebuild>())]
#[filter(component::<Style>())]
pub fn compose_display_to_layout_box(
    display: &Display,
    margin: &Margin,
    padding: &Padding,
    layout_box: &mut StyleLayoutBox) {
        match display.into() {
            DisplayTypes::Block => { 
                layout_box.set((
                    LayoutDirection::Vertical,
                    margin.into(), 
                    padding.into(),
                    LayoutDimensions::new(LayoutDistance::FromParent(1.0), LayoutDistance::FromChildren(1.0))));
            },
            DisplayTypes::Inline => { 
                layout_box.set((
                    LayoutDirection::Horizontal,
                    margin.into(), 
                    padding.into(),
                    LayoutDimensions::new(LayoutDistance::FromChildren(1.0), LayoutDistance::FromChildren(1.0))));
            },
            DisplayTypes::None => { 
                layout_box.set((LayoutDirection::None, LayoutOffsetRect::default(), LayoutOffsetRect::default(), LayoutDimensions::default()));
            },
        }

}