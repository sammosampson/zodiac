pub enum AbstractSyntaxNode {
    Circle(),
    Rectangle(),
    Text(),
    Position((u16, u16)),
    Dimensions((u16, u16)),
    Radius(u16),
    GlyphIndex(u16),
    StrokeColour((f32, f32, f32, f32)),
    Colour((f32, f32, f32, f32)),
    CornerRadii((f32, f32, f32, f32)),
    StrokeWidth(f32),
    CompleteControl(),
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
            Token::Control("rect") => Some(Ok(AbstractSyntaxNode::Rectangle())),
            Token::Control("circle") => Some(Ok(AbstractSyntaxNode::Circle())),
            Token::Control("text") => Some(Ok(AbstractSyntaxNode::Text())),
            Token::Control(_) => Some(Err(AbstractSyntaxParseError::UnknownControl)),
            Token::Property(name) => { 
                self.current_property = name; 
                None
            },
            Token::PropertyValue(value) => {
                match value {
                    TokenPropertyValue::Float(value) => {
                        match self.current_property {
                            "stroke_width" => Some(Ok(AbstractSyntaxNode::StrokeWidth(value as f32))),
                            _ => Some(Err(AbstractSyntaxParseError::UnknownProperty))
                        }
                    },
                    TokenPropertyValue::UnsignedInt(value) => {
                        match self.current_property {
                            "radius" => Some(Ok(AbstractSyntaxNode::Radius(value as u16))),
                            "glyph_index" => Some(Ok(AbstractSyntaxNode::GlyphIndex(value as u16))),
                            _ => Some(Err(AbstractSyntaxParseError::UnknownProperty))
                        }
                    },
                    TokenPropertyValue::Tuple(value) => {
                        let tuple_lexer = TupleLexer::parse(value);
                        None
                    },
                    _ => Some(Err(AbstractSyntaxParseError::UnusedPropertyType))
                }
                
            },
            Token::EndControl(_) => Some(Ok(AbstractSyntaxNode::CompleteControl()))
        }
    }
}