use zodiac_parsing::tokenization::abstract_syntax::*;
use zodiac_parsing::tokenization::source::*;

#[test]
fn parse_root_layout_container_produces_container_node() {
    let mut tokenizer = AbstractSyntaxTokenizer::from_source(SourceTokenizer::from_string("<root />"));
    assert_eq!(AbstractSyntaxToken::Root, tokenizer.next().unwrap().unwrap());
    assert_eq!(AbstractSyntaxToken::CompleteControl, tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}

#[test]
fn parse_canvas_layout_container_produces_container_node() {
    let mut tokenizer = AbstractSyntaxTokenizer::from_source(SourceTokenizer::from_string("<canvas />"));
    assert_eq!(AbstractSyntaxToken::CanvasLayoutContent, tokenizer.next().unwrap().unwrap());
    assert_eq!(AbstractSyntaxToken::CompleteControl, tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}

#[test]
fn parse_horizontal_layout_container_produces_container_node() {
    let mut tokenizer = AbstractSyntaxTokenizer::from_source(SourceTokenizer::from_string("<horizontal-stack />"));
    assert_eq!(AbstractSyntaxToken::HorizontalLayoutContent, tokenizer.next().unwrap().unwrap());
    assert_eq!(AbstractSyntaxToken::CompleteControl, tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}

#[test]
fn parse_container_with_children_produces_container_node_and_children() {
    let mut tokenizer = AbstractSyntaxTokenizer::from_source(SourceTokenizer::from_string("<horizontal-stack><circle/></horizontal-stack>"));
    assert_eq!(AbstractSyntaxToken::HorizontalLayoutContent, tokenizer.next().unwrap().unwrap());
    assert_eq!(AbstractSyntaxToken::Circle, tokenizer.next().unwrap().unwrap());
    assert_eq!(AbstractSyntaxToken::CompleteControl, tokenizer.next().unwrap().unwrap());
    assert_eq!(AbstractSyntaxToken::CompleteControl, tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}

#[test]
fn parse_vertical_layout_container_produces_container_node() {
    let mut tokenizer = AbstractSyntaxTokenizer::from_source(SourceTokenizer::from_string("<vertical-stack />"));
    assert_eq!(AbstractSyntaxToken::VerticalLayoutContent, tokenizer.next().unwrap().unwrap());
    assert_eq!(AbstractSyntaxToken::CompleteControl, tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}

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
fn text_content_produces_content_node() {
    let mut tokenizer = AbstractSyntaxTokenizer::from_source(SourceTokenizer::from_string("<text content=\"test\" />"));
    assert_eq!(AbstractSyntaxToken::Text, tokenizer.next().unwrap().unwrap());
    assert_eq!(AbstractSyntaxToken::Content("test"), tokenizer.next().unwrap().unwrap());
    assert_eq!(AbstractSyntaxToken::CompleteControl, tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}

#[test]
fn left_produces_left_node() {
    let mut tokenizer = AbstractSyntaxTokenizer::from_source(SourceTokenizer::from_string("<text left=100 />"));
    assert_eq!(AbstractSyntaxToken::Text, tokenizer.next().unwrap().unwrap());
    assert_eq!(AbstractSyntaxToken::Left(100), tokenizer.next().unwrap().unwrap());
    assert_eq!(AbstractSyntaxToken::CompleteControl, tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}

#[test]
fn top_produces_top_node() {
    let mut tokenizer = AbstractSyntaxTokenizer::from_source(SourceTokenizer::from_string("<text top=100 />"));
    assert_eq!(AbstractSyntaxToken::Text, tokenizer.next().unwrap().unwrap());
    assert_eq!(AbstractSyntaxToken::Top(100), tokenizer.next().unwrap().unwrap());
    assert_eq!(AbstractSyntaxToken::CompleteControl, tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}

#[test]
fn width_produces_width_node() {
    let mut tokenizer = AbstractSyntaxTokenizer::from_source(SourceTokenizer::from_string("<rect width=100 />"));
    assert_eq!(AbstractSyntaxToken::Rectangle, tokenizer.next().unwrap().unwrap());
    assert_eq!(AbstractSyntaxToken::Width(100), tokenizer.next().unwrap().unwrap());
    assert_eq!(AbstractSyntaxToken::CompleteControl, tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}

#[test]
fn height_produces_height_node() {
    let mut tokenizer = AbstractSyntaxTokenizer::from_source(SourceTokenizer::from_string("<rect height=100 />"));
    assert_eq!(AbstractSyntaxToken::Rectangle, tokenizer.next().unwrap().unwrap());
    assert_eq!(AbstractSyntaxToken::Height(100), tokenizer.next().unwrap().unwrap());
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
    let mut tokenizer = AbstractSyntaxTokenizer::from_source(SourceTokenizer::from_string("<rect corner-radii=(10, 9, 0, 10) />"));
    assert_eq!(AbstractSyntaxToken::Rectangle, tokenizer.next().unwrap().unwrap());
    assert_eq!(AbstractSyntaxToken::CornerRadii((10, 9, 0, 10)), tokenizer.next().unwrap().unwrap());
    assert_eq!(AbstractSyntaxToken::CompleteControl, tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}

#[test]
fn malformed_corner_radii_colour_produces_error() {
    let mut tokenizer = AbstractSyntaxTokenizer::from_source(SourceTokenizer::from_string("<rect corner-radii=(10, 9, 0) />"));
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
fn parse_with_unknown_float_property_type_produces_error() {
    let mut tokenizer = AbstractSyntaxTokenizer::from_source(SourceTokenizer::from_string("<circle radius=1.0 />"));
    assert_eq!(AbstractSyntaxToken::Circle, tokenizer.next().unwrap().unwrap());
    assert_eq!(Err(AbstractSyntaxTokenError::UnusedPropertyType), tokenizer.next().unwrap());
    assert_eq!(AbstractSyntaxToken::CompleteControl, tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}

#[test]
fn parse_with_unknown_int_property_produces_error() {
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
