use crate::*;
use super::nodes::*;

element! {
    <canvas>
    [LayoutContent::canvas()]
    attributes {
        left(u16)
        top(u16)
        width(u16)
        height(u16)
    }
}

element! {
    <vertical_stack>
    [LayoutContent::vertical()]
    attributes {
        left(u16)
        top(u16)
        width(u16)
        height(u16)
    }
}

element! {
    <horizontal_stack>
    [LayoutContent::horizontal()]
    attributes {
        left(u16)
        top(u16)
        width(u16)
        height(u16)
    }
}