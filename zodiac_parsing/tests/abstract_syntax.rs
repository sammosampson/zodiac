extern crate zodiac_parsing;
use zodiac_parsing::abstract_syntax::{AbstractSyntaxTokenizer, AbstractSyntaxToken, AbstractSyntaxTokenError};
use zodiac_parsing::source_tokenization::{SourceTokenizer};

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
fn position_produces_glyph_position_node() {
    let mut tokenizer = AbstractSyntaxTokenizer::from_source(SourceTokenizer::from_string("<text position=(100, 200) />"));
    assert_eq!(AbstractSyntaxToken::Text, tokenizer.next().unwrap().unwrap());
    assert_eq!(AbstractSyntaxToken::Position((100, 200)), tokenizer.next().unwrap().unwrap());
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
    assert_eq!(Some(Err(AbstractSyntaxTokenError::UnusedPropertyType)), tokenizer.next());
    assert_eq!(AbstractSyntaxToken::CompleteControl, tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}

#[test]
fn parse_with_unknown_property_produces_error() {
    let mut tokenizer = AbstractSyntaxTokenizer::from_source(SourceTokenizer::from_string("<circle head=1 />"));
    assert_eq!(AbstractSyntaxToken::Circle, tokenizer.next().unwrap().unwrap());
    assert_eq!(Some(Err(AbstractSyntaxTokenError::UnknownProperty)), tokenizer.next());
    assert_eq!(AbstractSyntaxToken::CompleteControl, tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}

#[test]
fn parse_with_bad_tag_value_produces_error() {
    let mut tokenizer = AbstractSyntaxTokenizer::from_source(SourceTokenizer::from_string("x />"));
    assert_eq!(Some(Err(AbstractSyntaxTokenError::SourceTokenError("could not find control start tag (<)", 0, 'x'))), tokenizer.next());
}

#[test]
fn parse_with_bad_property_value_produces_error() {
    let mut tokenizer = AbstractSyntaxTokenizer::from_source(SourceTokenizer::from_string("<circle radius=1.x />"));
    assert_eq!(AbstractSyntaxToken::Circle, tokenizer.next().unwrap().unwrap());
    assert_eq!(Some(Err(AbstractSyntaxTokenError::SourceValueError("could not parse number value", 18, "1.x"))), tokenizer.next());
    assert_eq!(AbstractSyntaxToken::CompleteControl, tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}
