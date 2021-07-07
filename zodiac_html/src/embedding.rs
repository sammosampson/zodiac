use zodiac::*;
use mox::*;
use crate::style::*;
use crate::borders::*;
use crate::layout::*;
use crate::size::*;
use crate::colour::*;
use crate::window::*;

element! {
    <style>
    Style::default(),
    extra_components {
        [ElementSelector::default()]
        [BorderWidth::default()]
        [BorderColour::default()]
        [BorderTop::default()]
        [BorderTopStyle::default()]
        [BorderTopColour::default()]
        [BorderTopWidth::default()]
        [BorderBottom::default()]
        [BorderBottomStyle::default()]
        [BorderBottomColour::default()]
        [BorderBottomWidth::default()]
        [BorderLeft::default()]
        [BorderLeftStyle::default()]
        [BorderLeftColour::default()]
        [BorderLeftWidth::default()]
        [BorderRight::default()]
        [BorderRightStyle::default()]
        [BorderRightColour::default()]
        [BorderRightWidth::default()]
        [BorderRadius::default()]
        [BorderStyle::default()]
        [FullBorder::default()]
        [Margin::default()]
        [Padding::default()]
        [BackgroundColour::default()]
    }
    attributes {
        element_selector(ElementType)
        border(BorderValues)
        border_bottom(BorderValues)
        border_bottom_colour(Colour)
        border_bottom_style(BorderStyles)
        border_bottom_width(Size)
        border_colour(Colour)
        border_left(BorderValues)
        border_left_colour(Colour)
        border_left_style(BorderStyles)
        border_left_width(Size)
        border_radius(Size)
        border_right(BorderValues)
        border_right_colour(Colour)
        border_right_style(BorderStyles)
        border_right_width(Size)
        border_style(BorderStyles)
        border_top(BorderValues)
        border_top_colour(Colour)
        border_top_style(BorderStyles)
        border_top_width(Size)
        border_width(Size)
        background_colour(Colour)
        display(DisplayTypes)
        margin(MarginSizes)
        padding(PaddingSizes)
    }
}

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
        [Renderable::default()]
    }
}

element! {
    <div>
    Element::from(ElementType::Div),
    extra_components {
        [Renderable::default()]
    }
}

pub fn default_style() -> Node {
    mox!(
        <style>
            <style
                element_selector=ElementType::Div
                display=DisplayTypes::Block
            />
            <style
                element_selector=ElementType::Span
                display=DisplayTypes::Inline
            />
        </style>
    )
}
