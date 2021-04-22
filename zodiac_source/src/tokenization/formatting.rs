use log::{info};
use crate::tokenization::{
    source::SourceTokenResult,
    abstract_syntax::AbstractSyntaxTokenizer
};

pub trait Pretty {
    fn to_pretty(&mut self);
}

impl<I> Pretty for I where I : Iterator<Item=SourceTokenResult> {
    fn to_pretty(&mut self) {
        for token in self {
            match token {
                Ok(value) => info!("{:?}", value),
                Err(error) => info!("{:?}", error) 
            }
        }
    }
}

impl<I> Pretty for AbstractSyntaxTokenizer<I> where I : Iterator<Item=SourceTokenResult> {
    fn to_pretty(&mut self) {
        for token in self {
            match token {
                Ok(value) => info!("{:?}", value),
                Err(error) => info!("{:?}", error) 
            }
        }
    }
}