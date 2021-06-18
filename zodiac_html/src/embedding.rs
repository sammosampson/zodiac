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
        [BorderWidth::default()]
        [BackgroundColour::default()]
        [BorderColour::default()]
        [BorderTopStyle::default()]
        [BorderBottomStyle::default()]
        [BorderLeftStyle::default()]
        [BorderRightStyle::default()]
        [BorderStyle::default()]
        [BorderRadius::default()]
        [Border::default()]
    }
    attributes {
        border_width(Size)
        background_colour(Colour)
        border_top_style(BorderStyles)
        border_bottom_style(BorderStyles)
        border_left_style(BorderStyles)
        border_right_style(BorderStyles)
        border_style(BorderStyles)
        border_radius(Size)
        border((Colour, Size, BorderStyles, Size))
        border_colour(Colour)
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
