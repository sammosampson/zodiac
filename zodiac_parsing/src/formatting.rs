use crate::lexing::Lexer;

pub trait Pretty {
    fn to_pretty(&mut self);
}

impl<'a> Pretty for Lexer<'a> {
    fn to_pretty(&mut self) {
        for token in self {
            match token {
                Ok(value) => println!("{:?}", value),
                Err(error) => println!("{:?}", error) 
            }
        }
    }
}