use zodiac::*;
use crate::style::*;
use crate::borders::*;
use crate::layout::*;
use crate::size::*;
use crate::window::*;

element! {
    <window>
    Window::default(),
    attributes {
        width(Size)
        height(Size)
        title(String)
    }
    default_children {
        [default_style()]
    }
}

element! {
    <span>
    Element::from(ElementType::Span),
    extra_components {
        [FullBorder::default()]
        [Renderable::default()]
    }
}

element! {
    <div>
    Element::from(ElementType::Div),
    extra_components {
        [FullBorder::default()]
        [Renderable::default()]
    }
}
