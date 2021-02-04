
#[derive(PartialEq, PartialOrd, Debug)]
pub enum AbstractSyntaxToken {
    Circle,
    Rectangle,
    Text,
    Position((u16, u16)),
    Dimensions((u16, u16)),
    Radius(u16),
    GlyphIndex(u16),
    StrokeColour((f32, f32, f32, f32)),
    Colour((f32, f32, f32, f32)),
    CornerRadii((f32, f32, f32, f32)),
    StrokeWidth(u16),
    CompleteControl,
}

use crate::source_tokenization::{SourceTokenResult, SourceTokenError, SourceToken, SourceTokenPropertyValue};
use crate::tuple_tokenization::TupleTokenizer;

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum AbstractSyntaxTokenError<'a> {
    SourceTokenError(&'a str, usize, char),
    SourceValueError(&'a str, usize, &'a str),
    UnknownControl,
    UnusedPropertyType,
    UnknownProperty
}

pub type AbstractSyntaxTokenResult<'a> = Result<AbstractSyntaxToken, AbstractSyntaxTokenError<'a>>;
pub type AbstractSyntaxTokenOption<'a> = Option<AbstractSyntaxTokenResult<'a>>;

pub struct AbstractSyntaxTokenizer<'a, I> where I : Iterator<Item=SourceTokenResult<'a>> {
    source_token_iterator: I,
    current_property: &'a str
}

impl <'a, I> Iterator for AbstractSyntaxTokenizer<'a, I> where I : Iterator<Item=SourceTokenResult<'a>> {
    type Item = AbstractSyntaxTokenResult<'a>;
    fn next(&mut self) -> AbstractSyntaxTokenOption<'a> {
        loop {
            return match self.source_token_iterator.next() {
                Some(result) => match result {
                    Ok(token) => match self.transition(token) {
                        None => continue,
                        some => some
                    },
                    Err(error) => match error {
                        SourceTokenError::ControlError(text, index, character) => Some(Err(AbstractSyntaxTokenError::SourceTokenError(text, index, character))),
                        SourceTokenError::PropertyError(text, index, character) => Some(Err(AbstractSyntaxTokenError::SourceTokenError(text, index, character))),
                        SourceTokenError::ValueError(text, index, raw_value) => Some(Err(AbstractSyntaxTokenError::SourceValueError(text, index, raw_value))),
                    }
                },
                None => {
                    None
                },
            }
        }
    }
}

impl <'a, I> AbstractSyntaxTokenizer<'a, I>  where I : Iterator<Item=SourceTokenResult<'a>> {
    pub fn from_source(source_token_iterator: I) -> Self {
        Self {
            source_token_iterator,
            current_property: "",
        }
    }
    
    fn transition(&mut self, token: SourceToken<'a>) -> AbstractSyntaxTokenOption<'a> {
        match token {
            SourceToken::Control("rect") => Some(Ok(AbstractSyntaxToken::Rectangle)),
            SourceToken::Control("circle") => Some(Ok(AbstractSyntaxToken::Circle)),
            SourceToken::Control("text") => Some(Ok(AbstractSyntaxToken::Text)),
            SourceToken::Control(_) => Some(Err(AbstractSyntaxTokenError::UnknownControl)),
            SourceToken::Property(name) => { 
                self.current_property = name; 
                None
            },
            SourceToken::PropertyValue(value) => {
                match value {
                    SourceTokenPropertyValue::UnsignedInt(value) => {
                        match self.current_property {
                            "stroke-width" => Some(Ok(AbstractSyntaxToken::StrokeWidth(value as u16))),
                            "radius" => Some(Ok(AbstractSyntaxToken::Radius(value as u16))),
                            "glyph-index" => Some(Ok(AbstractSyntaxToken::GlyphIndex(value as u16))),
                            _ => Some(Err(AbstractSyntaxTokenError::UnknownProperty))
                        }
                    },
                    SourceTokenPropertyValue::Tuple(value) => {
                        let tuple_tokenizer = TupleTokenizer::from_string(value);
                        None
                    },
                    _ => return Some(Err(AbstractSyntaxTokenError::UnusedPropertyType))
                }
                
            },
            SourceToken::EndControl(_) => Some(Ok(AbstractSyntaxToken::CompleteControl))
        }
    }
}