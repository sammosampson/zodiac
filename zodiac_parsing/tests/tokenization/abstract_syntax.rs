use zodiac_parsing::tokenization::abstract_syntax::*;
use zodiac_parsing::tokenization::source::*;

#[test]
fn parse_rect_produces_rectangle_node() {
    let mut tokenizer = AbstractSyntaxTokenizer::from_source(SourceTokenizer::from_string("<rect />"));
    assert_eq!(AbstractSyntaxToken::Rectangle, tokenizer.next().unwrap().unwrap());
    assert_eq!(AbstractSyntaxToken::CompleteControl, tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}

#[test]
fn parse_circle_produces_circle_node() {
    let mut tokenizer = AbstractSyntaxTokenizer::from_source(SourceTokenizer::from_string("<circle />"));
    assert_eq!(AbstractSyntaxToken::Circle, tokenizer.next().unwrap().unwrap());
    assert_eq!(AbstractSyntaxToken::CompleteControl, tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}

#[test]
fn parse_text_produces_text_node() {
    let mut tokenizer = AbstractSyntaxTokenizer::from_source(SourceTokenizer::from_string("<text />"));
    assert_eq!(AbstractSyntaxToken::Text, tokenizer.next().unwrap().unwrap());
    assert_eq!(AbstractSyntaxToken::CompleteControl, tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}

#[test]
fn parse_multiple_controls_produces_multiple_completes() {
    let mut tokenizer = AbstractSyntaxTokenizer::from_source(SourceTokenizer::from_string("<rect /><circle /><text />"));
    assert_eq!(AbstractSyntaxToken::Rectangle, tokenizer.next().unwrap().unwrap());
    assert_eq!(AbstractSyntaxToken::CompleteControl, tokenizer.next().unwrap().unwrap());
    assert_eq!(AbstractSyntaxToken::Circle, tokenizer.next().unwrap().unwrap());
    assert_eq!(AbstractSyntaxToken::CompleteControl, tokenizer.next().unwrap().unwrap());
    assert_eq!(AbstractSyntaxToken::Text, tokenizer.next().unwrap().unwrap());
    assert_eq!(AbstractSyntaxToken::CompleteControl, tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}

#[test]
fn parse_stroke_width_produces_stroke_width_node() {
    let mut tokenizer = AbstractSyntaxTokenizer::from_source(SourceTokenizer::from_string("<rect stroke-width=1 />"));
    assert_eq!(AbstractSyntaxToken::Rectangle, tokenizer.next().unwrap().unwrap());
    assert_eq!(AbstractSyntaxToken::StrokeWidth(1), tokenizer.next().unwrap().unwrap());
    assert_eq!(AbstractSyntaxToken::CompleteControl, tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}

#[test]
fn parse_radius_produces_radius_node() {
    let mut tokenizer = AbstractSyntaxTokenizer::from_source(SourceTokenizer::from_string("<circle radius=1 />"));
    assert_eq!(AbstractSyntaxToken::Circle, tokenizer.next().unwrap().unwrap());
    assert_eq!(AbstractSyntaxToken::Radius(1), tokenizer.next().unwrap().unwrap());
    assert_eq!(AbstractSyntaxToken::CompleteControl, tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}

#[test]
fn glyph_index_produces_glyph_index_node() {
    let mut tokenizer = AbstractSyntaxTokenizer::from_source(SourceTokenizer::from_string("<text glyph-index=1 />"));
    assert_eq!(AbstractSyntaxToken::Text, tokenizer.next().unwrap().unwrap());
    assert_eq!(AbstractSyntaxToken::GlyphIndex(1), tokenizer.next().unwrap().unwrap());
    assert_eq!(AbstractSyntaxToken::CompleteControl, tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}

#[test]
fn position_produces_position_node() {
    let mut tokenizer = AbstractSyntaxTokenizer::from_source(SourceTokenizer::from_string("<text position=(100, 200) />"));
    assert_eq!(AbstractSyntaxToken::Text, tokenizer.next().unwrap().unwrap());
    assert_eq!(AbstractSyntaxToken::Position((100, 200)), tokenizer.next().unwrap().unwrap());
    assert_eq!(AbstractSyntaxToken::CompleteControl, tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}

#[test]
fn malformed_position_produces_error() {
    let mut tokenizer = AbstractSyntaxTokenizer::from_source(SourceTokenizer::from_string("<text position=(100) />"));
    assert_eq!(AbstractSyntaxToken::Text, tokenizer.next().unwrap().unwrap());
    assert_eq!(Err(AbstractSyntaxTokenError::BadPositionValue), tokenizer.next().unwrap());
    assert_eq!(AbstractSyntaxToken::CompleteControl, tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}

#[test]
fn dimensions_produces_dimensions_node() {
    let mut tokenizer = AbstractSyntaxTokenizer::from_source(SourceTokenizer::from_string("<rect dimensions=(100, 200) />"));
    assert_eq!(AbstractSyntaxToken::Rectangle, tokenizer.next().unwrap().unwrap());
    assert_eq!(AbstractSyntaxToken::Dimensions((100, 200)), tokenizer.next().unwrap().unwrap());
    assert_eq!(AbstractSyntaxToken::CompleteControl, tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}

#[test]
fn malformed_dimensions_produces_error() {
    let mut tokenizer = AbstractSyntaxTokenizer::from_source(SourceTokenizer::from_string("<rect dimensions=(100) />"));
    assert_eq!(AbstractSyntaxToken::Rectangle, tokenizer.next().unwrap().unwrap());
    assert_eq!(Err(AbstractSyntaxTokenError::BadDimensionsValue), tokenizer.next().unwrap());
    assert_eq!(AbstractSyntaxToken::CompleteControl, tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}

#[test]
fn colour_produces_colour_node() {
    let mut tokenizer = AbstractSyntaxTokenizer::from_source(SourceTokenizer::from_string("<rect colour=(1.0, 0.9, 0.0, 1.0) />"));
    assert_eq!(AbstractSyntaxToken::Rectangle, tokenizer.next().unwrap().unwrap());
    assert_eq!(AbstractSyntaxToken::Colour((1.0, 0.9, 0.0, 1.0)), tokenizer.next().unwrap().unwrap());
    assert_eq!(AbstractSyntaxToken::CompleteControl, tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}

#[test]
fn malformed_colour_produces_error() {
    let mut tokenizer = AbstractSyntaxTokenizer::from_source(SourceTokenizer::from_string("<rect colour=(1.0, 0.9, 0.0) />"));
    assert_eq!(AbstractSyntaxToken::Rectangle, tokenizer.next().unwrap().unwrap());
    assert_eq!(Err(AbstractSyntaxTokenError::BadColourValue), tokenizer.next().unwrap());
    assert_eq!(AbstractSyntaxToken::CompleteControl, tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}

#[test]
fn stroke_colour_produces_stroke_colour_node() {
    let mut tokenizer = AbstractSyntaxTokenizer::from_source(SourceTokenizer::from_string("<rect stroke-colour=(1.0, 0.9, 0.0, 1.0) />"));
    assert_eq!(AbstractSyntaxToken::Rectangle, tokenizer.next().unwrap().unwrap());
    assert_eq!(AbstractSyntaxToken::StrokeColour((1.0, 0.9, 0.0, 1.0)), tokenizer.next().unwrap().unwrap());
    assert_eq!(AbstractSyntaxToken::CompleteControl, tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}

#[test]
fn malformed_stroke_colour_produces_error() {
    let mut tokenizer = AbstractSyntaxTokenizer::from_source(SourceTokenizer::from_string("<rect stroke-colour=(1.0, 0.9, 0.0) />"));
    assert_eq!(AbstractSyntaxToken::Rectangle, tokenizer.next().unwrap().unwrap());
    assert_eq!(Err(AbstractSyntaxTokenError::BadStrokeColourValue), tokenizer.next().unwrap());
    assert_eq!(AbstractSyntaxToken::CompleteControl, tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}

#[test]
fn corner_radii_produces_stroke_colour_node() {
    let mut tokenizer = AbstractSyntaxTokenizer::from_source(SourceTokenizer::from_string("<rect corner-radii=(1.0, 0.9, 0.0, 1.0) />"));
    assert_eq!(AbstractSyntaxToken::Rectangle, tokenizer.next().unwrap().unwrap());
    assert_eq!(AbstractSyntaxToken::CornerRadii((1.0, 0.9, 0.0, 1.0)), tokenizer.next().unwrap().unwrap());
    assert_eq!(AbstractSyntaxToken::CompleteControl, tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}

#[test]
fn malformed_corner_radii_colour_produces_error() {
    let mut tokenizer = AbstractSyntaxTokenizer::from_source(SourceTokenizer::from_string("<rect corner-radii=(1.0, 0.9, 0.0) />"));
    assert_eq!(AbstractSyntaxToken::Rectangle, tokenizer.next().unwrap().unwrap());
    assert_eq!(Err(AbstractSyntaxTokenError::BadCornerRadiiValue), tokenizer.next().unwrap());
    assert_eq!(AbstractSyntaxToken::CompleteControl, tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}

#[test]
fn parse_with_multiple_properties_produces_correct_nodes() {
    let mut tokenizer = AbstractSyntaxTokenizer::from_source(SourceTokenizer::from_string("<circle radius=1 stroke-width=2 />"));
    assert_eq!(AbstractSyntaxToken::Circle, tokenizer.next().unwrap().unwrap());
    assert_eq!(AbstractSyntaxToken::Radius(1), tokenizer.next().unwrap().unwrap());
    assert_eq!(AbstractSyntaxToken::StrokeWidth(2), tokenizer.next().unwrap().unwrap());
    assert_eq!(AbstractSyntaxToken::CompleteControl, tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}

#[test]
fn parse_with_unknown_control_produces_error() {
    let mut tokenizer = AbstractSyntaxTokenizer::from_source(SourceTokenizer::from_string("<square />"));
    assert_eq!(Some(Err(AbstractSyntaxTokenError::UnknownControl)), tokenizer.next());
    assert_eq!(AbstractSyntaxToken::CompleteControl, tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}

#[test]
fn parse_with_unknown_property_type_produces_error() {
    let mut tokenizer = AbstractSyntaxTokenizer::from_source(SourceTokenizer::from_string("<circle radius=1.0 />"));
    assert_eq!(AbstractSyntaxToken::Circle, tokenizer.next().unwrap().unwrap());
    assert_eq!(Err(AbstractSyntaxTokenError::UnusedPropertyType), tokenizer.next().unwrap());
    assert_eq!(AbstractSyntaxToken::CompleteControl, tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}

#[test]
fn parse_with_unknown_property_produces_error() {
    let mut tokenizer = AbstractSyntaxTokenizer::from_source(SourceTokenizer::from_string("<circle head=1 />"));
    assert_eq!(AbstractSyntaxToken::Circle, tokenizer.next().unwrap().unwrap());
    assert_eq!(Err(AbstractSyntaxTokenError::UnknownProperty), tokenizer.next().unwrap());
    assert_eq!(AbstractSyntaxToken::CompleteControl, tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}

#[test]
fn parse_with_bad_tag_value_produces_error() {
    let mut tokenizer = AbstractSyntaxTokenizer::from_source(SourceTokenizer::from_string("x />"));
    assert_eq!(Err(AbstractSyntaxTokenError::SourceTokenError(SourceTokenError::CouldNotFindStartTag(0))), tokenizer.next().unwrap());
}

#[test]
fn parse_with_bad_property_value_produces_error() {
    let mut tokenizer = AbstractSyntaxTokenizer::from_source(SourceTokenizer::from_string("<circle radius=1.x />"));
    assert_eq!(AbstractSyntaxToken::Circle, tokenizer.next().unwrap().unwrap());
    assert_eq!(Some(Err(AbstractSyntaxTokenError::SourceTokenError(SourceTokenError::CouldNotParseNumberValue(18)))), tokenizer.next());
    assert_eq!(AbstractSyntaxToken::CompleteControl, tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}
