use legion::*;

use crate::layout::*;

#[system(for_each)]
pub fn layout_display(
    display: &Display,
    margin: &Margin,
    padding: &Padding,
    layout_box: &mut LayoutBox) {
        
}


pub struct LayoutBox;