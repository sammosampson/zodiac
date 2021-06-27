use zodiac::*;
use crate::style::*;
use crate::borders::*;
use crate::layout::*;
use crate::size::*;
use crate::colour::*;

element! {
    <style>
    [Style::default()]
    extra_components {
        [Renderable::default()]
        [BackgroundColour::default()]
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
        [Margin::default()]
        [Padding::default()]
        [FullBorder::default()]
    }
    attributes {
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
    <span>
    [Span::default()]
    extra_components {
        [Display::inline()]
    }
}

element! {
    <div>
    [Div::default()]
    extra_components {
        [Display::block()]
    }
}
