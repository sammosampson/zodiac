pub enum AbstractSyntaxNode {
    Empty
}

impl AbstractSyntaxNode {
    fn circle() -> AbstractSyntaxNode { AbstractSyntaxNode::Empty }
    fn rectangle() -> AbstractSyntaxNode { AbstractSyntaxNode::Empty }
    fn text() -> AbstractSyntaxNode { AbstractSyntaxNode::Empty }
    fn set_position(position: (u16, u16)) -> AbstractSyntaxNode { AbstractSyntaxNode::Empty }
    fn set_dimensions(dimensions: (u16, u16)) -> AbstractSyntaxNode { AbstractSyntaxNode::Empty }
    fn set_radius(radius: u16) -> AbstractSyntaxNode { AbstractSyntaxNode::Empty }
    fn set_glyph_index(glyph_index: u16) -> AbstractSyntaxNode { AbstractSyntaxNode::Empty }
    fn set_stroke_colour(outer_colour: (f32, f32, f32, f32)) -> AbstractSyntaxNode { AbstractSyntaxNode::Empty }
    fn set_colour(outer_colour: (f32, f32, f32, f32)) -> AbstractSyntaxNode { AbstractSyntaxNode::Empty }
    fn set_corner_radii(corner_radii: (f32, f32, f32, f32)) -> AbstractSyntaxNode { AbstractSyntaxNode::Empty }
    fn set_stroke_width(stroke_width: f32) -> AbstractSyntaxNode { AbstractSyntaxNode::Empty }
    fn complete_control() -> AbstractSyntaxNode { AbstractSyntaxNode::Empty }
}

use crate::lexing::{Lexer, LexerError, Token, TokenPropertyValue};
use crate::tuple_lexing::TupleLexer;


#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum AbstractSyntaxParseError<'a> {
    LexerParseError(&'a str, usize, char),
    LexerValueError(&'a str, usize, &'a str),
    UnknownControl,
    UnusedPropertyType,
    UnknownProperty
}

pub type AbstractSyntaxParserResult<'a> = Result<AbstractSyntaxNode, AbstractSyntaxParseError<'a>>;
pub type AbstractSyntaxParserOption<'a> = Option<AbstractSyntaxParserResult<'a>>;

pub struct AbstractSyntaxParser<'a> {
    lexer: Lexer<'a>,
    current_property: &'a str
}

impl <'a> Iterator for AbstractSyntaxParser<'a> {
    type Item = AbstractSyntaxParserResult<'a>;
    fn next(&mut self) -> AbstractSyntaxParserOption<'a> {
        loop {
            return match self.lexer.next() {
                Some(result) => match result {
                    Ok(token) => match self.transition(token) {
                        None => continue,
                        some => some
                    },
                    Err(error) => match error {
                        LexerError::ControlError(text, index, character) => Some(Err(AbstractSyntaxParseError::LexerParseError(text, index, character))),
                        LexerError::PropertyError(text, index, character) => Some(Err(AbstractSyntaxParseError::LexerParseError(text, index, character))),
                        LexerError::ValueError(text, index, raw_value) => Some(Err(AbstractSyntaxParseError::LexerValueError(text, index, raw_value))),
                    }
                },
                None => {
                    None
                },
            }
        }
    }
}

impl <'a> AbstractSyntaxParser<'a> {
    pub fn parse(input: &'a str) -> Self {
        Self {
            lexer: Lexer::parse(input),
            current_property: "",
        }
    }
    
    fn transition(&mut self, token: Token<'a>) -> AbstractSyntaxParserOption<'a> {
        match token {
            Token::Control("rect") => Some(Ok(AbstractSyntaxNode::rectangle())),
            Token::Control("circle") => Some(Ok(AbstractSyntaxNode::circle())),
            Token::Control("text") => Some(Ok(AbstractSyntaxNode::text())),
            Token::Control(_) => Some(Err(AbstractSyntaxParseError::UnknownControl)),
            Token::Property(name) => { 
                self.current_property = name; 
                None
            },
            Token::PropertyValue(value) => {
                match value {
                    TokenPropertyValue::Float(value) => {
                        match self.current_property {
                            "stroke_width" => Some(Ok(AbstractSyntaxNode::set_stroke_width(value as f32))),
                            _ => Some(Err(AbstractSyntaxParseError::UnknownProperty))
                        }
                    },
                    TokenPropertyValue::UnsignedInt(value) => {
                        match self.current_property {
                            "radius" => Some(Ok(AbstractSyntaxNode::set_radius(value as u16))),
                            "glyph_index" => Some(Ok(AbstractSyntaxNode::set_glyph_index(value as u16))),
                            _ => Some(Err(AbstractSyntaxParseError::UnknownProperty))
                        }
                    },
                    TokenPropertyValue::Tuple(value) => {
                        let tuple_lexer = TupleLexer::parse(value);
                        Some(Ok(AbstractSyntaxNode::Empty)) 
                    },
                    _ => Some(Err(AbstractSyntaxParseError::UnusedPropertyType))
                }
                
            },
            Token::EndControl(_) => Some(Ok(AbstractSyntaxNode::complete_control()))
        }
    }
}