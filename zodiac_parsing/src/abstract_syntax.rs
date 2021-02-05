

use crate::source_tokenization::{SourceTokenResult, SourceTokenError, SourceToken, SourceTokenPropertyValue};
use crate::tuple_tokenization::{TupleTokenizer, TupleTokenUnsignedShortIterator, TupleTokenFloatIterator};

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

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum AbstractSyntaxTokenError<'a> {
    SourceTokenError(&'a str, usize, char),
    SourceValueError(&'a str, usize, &'a str),
    UnknownControl,
    UnusedPropertyType,
    UnknownProperty,
    BadPositionValue,
    BadDimensionsValue,
    BadColourValue,
    BadStrokeColourValue,
    BadCornerRadiiValue
}

impl<'a> From<SourceTokenError<'a>> for AbstractSyntaxTokenError<'a> {
    fn from(error: SourceTokenError<'a>) -> Self {
        match error {
            SourceTokenError::ControlError(text, index, character) => AbstractSyntaxTokenError::SourceTokenError(text, index, character),
            SourceTokenError::PropertyError(text, index, character) => AbstractSyntaxTokenError::SourceTokenError(text, index, character),
            SourceTokenError::ValueError(text, index, raw_value) => AbstractSyntaxTokenError::SourceValueError(text, index, raw_value),
        }
    }
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
                    Err(error) => Some(Err(AbstractSyntaxTokenError::from(error)))
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
                        return match self.current_property {
                            "position" => {
                                match TupleTokenUnsignedShortIterator::from_iterator(tuple_tokenizer).collect_specific_amount(2) {
                                    Ok(values) => Some(Ok(AbstractSyntaxToken::Position((values[0], values[1])))),
                                    Err(_) => Some(Err(AbstractSyntaxTokenError::BadPositionValue))
                                }
                            },
                            "dimensions" => {
                                match TupleTokenUnsignedShortIterator::from_iterator(tuple_tokenizer).collect_specific_amount(2) {
                                    Ok(values) => Some(Ok(AbstractSyntaxToken::Dimensions((values[0], values[1])))),
                                    Err(_) => Some(Err(AbstractSyntaxTokenError::BadDimensionsValue))
                                }
                            },
                            "colour" => {
                                match TupleTokenFloatIterator::from_iterator(tuple_tokenizer).collect_specific_amount(4) {
                                    Ok(values) => Some(Ok(AbstractSyntaxToken::Colour((values[0], values[1], values[2], values[3])))),
                                    Err(_) => Some(Err(AbstractSyntaxTokenError::BadColourValue))
                                }
                            },
                            "stroke-colour" => {
                                match TupleTokenFloatIterator::from_iterator(tuple_tokenizer).collect_specific_amount(4) {
                                    Ok(values) => Some(Ok(AbstractSyntaxToken::StrokeColour((values[0], values[1], values[2], values[3])))),
                                    Err(_) => Some(Err(AbstractSyntaxTokenError::BadStrokeColourValue))
                                }
                            },
                            "corner-radii" => {
                                match TupleTokenFloatIterator::from_iterator(tuple_tokenizer).collect_specific_amount(4) {
                                    Ok(values) => Some(Ok(AbstractSyntaxToken::CornerRadii((values[0], values[1], values[2], values[3])))),
                                    Err(_) => Some(Err(AbstractSyntaxTokenError::BadCornerRadiiValue))
                                }
                            },
                            _ => Some(Err(AbstractSyntaxTokenError::UnknownProperty))
                        };
                    },
                    _ => Some(Err(AbstractSyntaxTokenError::UnusedPropertyType))
                }
                
            },
            SourceToken::EndControl(_) => Some(Ok(AbstractSyntaxToken::CompleteControl))
        }
    }
}

pub enum SpecificCollectionError {
    NotEnoughItems(usize)
}

pub trait SpecificAmountCollector<IT, I> where IT: Iterator<Item=I> {
    fn collect_specific_amount(&mut self, sepecific_amount: usize) -> Result<Vec::<I>, SpecificCollectionError>;
}

impl<IT, I> SpecificAmountCollector<IT, I> for IT where IT: Iterator<Item=I> {
    fn collect_specific_amount(&mut self, specific_amount: usize) -> Result<Vec::<I>, SpecificCollectionError> {
        let items:Vec::<I> = self.collect();
        let count = items.len();
        if count == specific_amount {
            Ok(items) 
        } else {
            Err(SpecificCollectionError::NotEnoughItems(count))
        }
    }
}
