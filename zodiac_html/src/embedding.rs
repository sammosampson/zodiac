use zodiac::*;
use crate::borders::*;
use crate::components::*;
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
    }
    attributes {
        border((Colour, Size, BorderStyles))
        border_bottom((Size, BorderStyles, Colour))
        border_bottom_colour(Colour)
        border_bottom_style(BorderStyles)
        border_bottom_width(Size)
        border_colour(Colour)
        border_left((Size, BorderStyles, Colour))
        border_left_colour(Colour)
        border_left_style(BorderStyles)
        border_left_width(Size)
        border_radius(Size)
        border_right((Size, BorderStyles, Colour))
        border_right_colour(Colour)
        border_right_style(BorderStyles)
        border_right_width(Size)
        border_style(BorderStyles)
        border_top((Size, BorderStyles, Colour))
        border_top_colour(Colour)
        border_top_style(BorderStyles)
        border_top_width(Size)
        border_width(Size)
        background_colour(Colour)
    }
}

element! {
    <span>
    [Span::default()]
    extra_components {
        [LayoutContent::canvas()]
    }
}

element! {
    <div>
    [LayoutContent::vertical()]
}
