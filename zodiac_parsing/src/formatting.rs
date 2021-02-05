use crate::tokenization::{
    source::SourceTokenResult,
    abstract_syntax::AbstractSyntaxTokenizer
};

pub trait Pretty {
    fn to_pretty(&mut self);
}

impl<'a, I> Pretty for I where I : Iterator<Item=SourceTokenResult<'a>> {
    fn to_pretty(&mut self) {
        for token in self {
            match token {
                Ok(value) => println!("{:?}", value),
                Err(error) => println!("{:?}", error) 
            }
        }
    }
}

impl<'a, I> Pretty for AbstractSyntaxTokenizer<'a, I> where I : Iterator<Item=SourceTokenResult<'a>> {
    fn to_pretty(&mut self) {
        for token in self {
            match token {
                Ok(value) => println!("{:?}", value),
                Err(error) => println!("{:?}", error) 
            }
        }
    }
}