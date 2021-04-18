
use crate::tokenization::source::{SourceTokenResult, SourceToken, SourceTokenPropertyValue};
use crate::tokenization::tuple::{TupleTokenizer, TupleTokenFloatIterator, TupleTokenUnsignedShortIterator};
use zodiac_entities::*;

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum AbstractSyntaxToken {
    Root,
    Import,
    Circle,
    Rectangle,
    Text,
    CanvasLayoutContent,
    HorizontalLayoutContent,
    VerticalLayoutContent,
    Control,
    ControlImplementation(String),
    Left(u16),
    Top(u16),
    Width(u16),
    Height(u16),
    Radius(u16),
    Content(String),
    Path(String),
    Name(String),
    StrokeColour((f32, f32, f32, f32)),
    Colour((f32, f32, f32, f32)),
    CornerRadii((u16, u16, u16, u16)),
    StrokeWidth(u16),
    CompleteControl,
}

impl<'a> From<&AbstractSyntaxToken> for AbstractSyntaxNodeType {
    fn from(token: &AbstractSyntaxToken) -> AbstractSyntaxNodeType {
        match token {
            &AbstractSyntaxToken::Root => AbstractSyntaxNodeType::Root,
            &AbstractSyntaxToken::Import => AbstractSyntaxNodeType::Import,
            &AbstractSyntaxToken::Circle => AbstractSyntaxNodeType::Circle,
            &AbstractSyntaxToken::Rectangle => AbstractSyntaxNodeType::Rectangle,
            &AbstractSyntaxToken::Text => AbstractSyntaxNodeType::Text,
            &AbstractSyntaxToken::CanvasLayoutContent => AbstractSyntaxNodeType::CanvasLayoutContent,
            &AbstractSyntaxToken::HorizontalLayoutContent => AbstractSyntaxNodeType::HorizontalLayoutContent,
            &AbstractSyntaxToken::VerticalLayoutContent => AbstractSyntaxNodeType::VerticalLayoutContent,
            &AbstractSyntaxToken::Content(_) => AbstractSyntaxNodeType::Content,
            &AbstractSyntaxToken::Left(_) => AbstractSyntaxNodeType::Left,
            &AbstractSyntaxToken::Top(_) => AbstractSyntaxNodeType::Top,
            &AbstractSyntaxToken::Width(_) => AbstractSyntaxNodeType::Width,
            &AbstractSyntaxToken::Height(_) => AbstractSyntaxNodeType::Height,
            &AbstractSyntaxToken::Radius(_) => AbstractSyntaxNodeType::Radius,
            &AbstractSyntaxToken::StrokeWidth(_) => AbstractSyntaxNodeType::StrokeWidth,
            &AbstractSyntaxToken::Path(_) => AbstractSyntaxNodeType::Path,
            &AbstractSyntaxToken::Name(_) => AbstractSyntaxNodeType::Name,
            &AbstractSyntaxToken::Colour(_) => AbstractSyntaxNodeType::Colour,
            &AbstractSyntaxToken::StrokeColour(_) => AbstractSyntaxNodeType::StrokeColour,
            &AbstractSyntaxToken::CornerRadii(_) => AbstractSyntaxNodeType::CornerRadii,
            _ => AbstractSyntaxNodeType::Unknown
        }
    }
}

pub type AbstractSyntaxTokenResult = Result<AbstractSyntaxToken, AbstractSyntaxTokenError>;
pub type AbstractSyntaxTokenOption = Option<AbstractSyntaxTokenResult>;

pub struct AbstractSyntaxTokenizer<I> where I : Iterator<Item=SourceTokenResult> {
    source_token_iterator: I,
    current_property: String
}

impl <'a, I> Iterator for AbstractSyntaxTokenizer<I> where I : Iterator<Item=SourceTokenResult> {
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

impl <I> AbstractSyntaxTokenizer<I>  where I : Iterator<Item=SourceTokenResult> {
    pub fn from_source(source_token_iterator: I) -> Self {
        Self {
            source_token_iterator,
            current_property: String::from("")
        }
    }
    
    fn transition(&mut self, token: SourceToken) -> AbstractSyntaxTokenOption {
        match token {
            SourceToken::Control(value) => AbstractSyntaxTokenizer::<I>::match_control(&value),
            SourceToken::Property(name) => {
                self.current_property = name; 
                None
            },
            SourceToken::PropertyValue(value) => {
                let property_name = &mut self.current_property;
                match value {
                    SourceTokenPropertyValue::String(value) => AbstractSyntaxTokenizer::<I>::match_string_property(property_name, value),
                    SourceTokenPropertyValue::UnsignedInt(value) =>AbstractSyntaxTokenizer::<I>::match_number_property(property_name, value),
                    SourceTokenPropertyValue::Tuple(value) => AbstractSyntaxTokenizer::<I>::match_tuple_property(property_name, value),
                    _ => Some(Err(AbstractSyntaxTokenError::UnusedPropertyType))
                }
                
            },
            SourceToken::EndControl(_) => Some(Ok(AbstractSyntaxToken::CompleteControl))
        }
    }

    fn match_control(value: &str) -> AbstractSyntaxTokenOption {
        match value { 
            "root" => Some(Ok(AbstractSyntaxToken::Root)),
            "control" => Some(Ok(AbstractSyntaxToken::Control)),
            "import" => Some(Ok(AbstractSyntaxToken::Import)),
            "canvas" => Some(Ok(AbstractSyntaxToken::CanvasLayoutContent)),
            "horizontal-stack" => Some(Ok(AbstractSyntaxToken::HorizontalLayoutContent)),
            "vertical-stack" => Some(Ok(AbstractSyntaxToken::VerticalLayoutContent)),
            "rect" => Some(Ok(AbstractSyntaxToken::Rectangle)),
            "circle" => Some(Ok(AbstractSyntaxToken::Circle)),
            "text" => Some(Ok(AbstractSyntaxToken::Text)),
            user_control => Some(Ok(AbstractSyntaxToken::ControlImplementation(String::from(user_control))))
        }
    }
    
    fn match_number_property(property_name: &str, value: u128) -> AbstractSyntaxTokenOption {
        match property_name { 
            "left" => Some(Ok(AbstractSyntaxToken::Left(value as u16))),
            "top" => Some(Ok(AbstractSyntaxToken::Top(value as u16))),
            "width" => Some(Ok(AbstractSyntaxToken::Width(value as u16))),
            "height" => Some(Ok(AbstractSyntaxToken::Height(value as u16))),
            "stroke-width" => Some(Ok(AbstractSyntaxToken::StrokeWidth(value as u16))),
            "radius" => Some(Ok(AbstractSyntaxToken::Radius(value as u16))),
            _ => Some(Err(AbstractSyntaxTokenError::UnknownProperty))
        }
    }

    fn match_string_property(property_name: &str, value: String) -> AbstractSyntaxTokenOption {
        match property_name { 
            "content" => Some(Ok(AbstractSyntaxToken::Content(value))),
            "path" => Some(Ok(AbstractSyntaxToken::Path(value))),
            "name" => Some(Ok(AbstractSyntaxToken::Name(value))),
            _ => Some(Err(AbstractSyntaxTokenError::UnknownProperty))
        }
    }
    
    fn match_tuple_property(property_name: &str, value: String) -> AbstractSyntaxTokenOption {
        let tuple_tokenizer = TupleTokenizer::from_string(&value);
        match property_name {
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

pub fn contains_root(tokens: &Vec<AbstractSyntaxTokenResult>) -> bool {
    tokens
        .iter()
        .any(|token_result| { 
            match token_result {
                Ok(token) => *token == AbstractSyntaxToken::Root,
                Err(_) => false
            }
        })
}