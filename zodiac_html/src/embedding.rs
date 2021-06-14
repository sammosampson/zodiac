use zodiac::*;
use crate::components::*;
use crate::text::*;
use crate::colour::*;

element! {
    <style>
    [Style::default()]
    attributes {
        text_size(FontSize)
        text_colour(Colour)
    }
}

element! {
    <span>
    [Span::default()]
}
