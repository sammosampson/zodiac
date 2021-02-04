use crate::source_tokenization::SourceTokenizer;

pub trait Pretty {
    fn to_pretty(&mut self);
}

impl<'a> Pretty for SourceTokenizer<'a> {
    fn to_pretty(&mut self) {
        for token in self {
            match token {
                Ok(value) => println!("{:?}", value),
                Err(error) => println!("{:?}", error) 
            }
        }
    }
}