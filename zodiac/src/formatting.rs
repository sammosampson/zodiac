use legion::*;
/* use zodiac_parsing::tokenization::{
    source::SourceTokenResult,
    abstract_syntax::AbstractSyntaxTokenizer
}; */

use crate::components::*;

pub trait Pretty {
    fn to_pretty(&mut self);
}

/* impl<'a, I> Pretty for I where I : Iterator<Item=SourceTokenResult<'a>> {
    fn to_pretty(&mut self) {
        for token in self {
            match token {
                Ok(value) => println!("{:?}", value),
                Err(error) => println!("{:?}", error) 
            }
        }
    }
} */

/* impl<'a, I> Pretty for AbstractSyntaxTokenizer<'a, I> where I : Iterator<Item=SourceTokenResult<'a>> {
    fn to_pretty(&mut self) {
        for token in self {
            match token {
                Ok(value) => println!("{:?}", value),
                Err(error) => println!("{:?}", error) 
            }
        }
    }
} */

impl Pretty for World {
    fn to_pretty(&mut self) {
        let mut query = <(&Position, &Dimensions)>::query();
        for (position, dimensions) in query.iter(self) {
            println!("{:?}", position);
            println!("{:?}", dimensions);
        }
    }
}