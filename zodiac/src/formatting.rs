use legion::*;
use legion::world::*;
 use zodiac_parsing::tokenization::{
    source::SourceTokenResult,
    abstract_syntax::AbstractSyntaxTokenizer
};

use zodiac_entities::components::*;

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

impl Pretty for World {
    fn to_pretty(&mut self) {
        let mut query = <(Entity, &Relationship, TryRead<CanvasLayoutContent>, TryRead<HorizontalLayoutContent>)>::query();
        for (entity, relationship, canvas_opiton, layout_opiton) in query.iter(self) {
            if let Some(_) = canvas_opiton {
                println!("canvas");
            }
            if let Some(_) = layout_opiton {
                println!("horizontal");
            }
            println!("{:?}", entity);
            println!("{:?}", relationship);
        }
    }
}

impl<'a> Pretty for SubWorld<'a> {
    fn to_pretty(&mut self) {
        let mut query = <(Entity, &Left, &Top, &Relationship)>::query();
        for (entity, left, top, relationship) in query.iter(self) {
            println!("{:?}", entity);
            println!("{:?}", left);
            println!("{:?}", top);
            println!("{:?}", relationship);
        }
    }
}