
use crate::tokenization::source::{SourceTokenResult, SourceTokenError, SourceToken, SourceTokenPropertyValue};
use crate::tokenization::tuple::{TupleTokenizer, TupleTokenFloatIterator, TupleTokenUnsignedShortIterator};

#[derive(PartialEq, PartialOrd, Debug)]
pub enum AbstractSyntaxToken {
    Circle,
    Rectangle,
    Text,
    CanvasLayoutContent,
    HorizontalLayoutContent,
    VerticalLayoutContent,
    Left(u16),
    Top(u16),
    Width(u16),
    Height(u16),
    Radius(u16),
    GlyphIndex(u16),
    StrokeColour((f32, f32, f32, f32)),
    Colour((f32, f32, f32, f32)),
    CornerRadii((u16, u16, u16, u16)),
    StrokeWidth(u16),
    CompleteControl,
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum AbstractSyntaxTokenError {
    SourceTokenError(SourceTokenError),
    UnknownControl,
    UnusedPropertyType,
    UnknownProperty,
    BadColourValue,
    BadStrokeColourValue,
    BadCornerRadiiValue
}

impl<'a> From<SourceTokenError> for AbstractSyntaxTokenError {
    fn from(error: SourceTokenError) -> Self {
        AbstractSyntaxTokenError::SourceTokenError(error)
    }
}

pub type AbstractSyntaxTokenResult = Result<AbstractSyntaxToken, AbstractSyntaxTokenError>;
pub type AbstractSyntaxTokenOption = Option<AbstractSyntaxTokenResult>;

pub struct AbstractSyntaxTokenizer<'a, I> where I : Iterator<Item=SourceTokenResult<'a>> {
    source_token_iterator: I,
    current_property: &'a str
}

impl <'a, I> Iterator for AbstractSyntaxTokenizer<'a, I> where I : Iterator<Item=SourceTokenResult<'a>> {
    type Item = AbstractSyntaxTokenResult;
    fn next(&mut self) -> AbstractSyntaxTokenOption {
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
    
    fn transition(&mut self, token: SourceToken<'a>) -> AbstractSyntaxTokenOption {
        match token {
            SourceToken::Control("canvas") => Some(Ok(AbstractSyntaxToken::CanvasLayoutContent)),
            SourceToken::Control("horizontal-stack") => Some(Ok(AbstractSyntaxToken::HorizontalLayoutContent)),
            SourceToken::Control("vertical-stack") => Some(Ok(AbstractSyntaxToken::VerticalLayoutContent)),
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
                            "left" => Some(Ok(AbstractSyntaxToken::Left(value as u16))),
                            "top" => Some(Ok(AbstractSyntaxToken::Top(value as u16))),
                            "width" => Some(Ok(AbstractSyntaxToken::Width(value as u16))),
                            "height" => Some(Ok(AbstractSyntaxToken::Height(value as u16))),
                            "stroke-width" => Some(Ok(AbstractSyntaxToken::StrokeWidth(value as u16))),
                            "radius" => Some(Ok(AbstractSyntaxToken::Radius(value as u16))),
                            "glyph-index" => Some(Ok(AbstractSyntaxToken::GlyphIndex(value as u16))),
                            _ => Some(Err(AbstractSyntaxTokenError::UnknownProperty))
                        }
                    },
                    SourceTokenPropertyValue::Tuple(value) => {
                        let tuple_tokenizer = TupleTokenizer::from_string(value);
                        return match self.current_property {
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
                                match TupleTokenUnsignedShortIterator::from_iterator(tuple_tokenizer).collect_specific_amount(4) {
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
