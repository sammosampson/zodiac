use legion::*;
use legion::world::*;
use crate::components::*;

pub trait Pretty {
    fn to_pretty(&mut self);
}

impl Pretty for World {
    fn to_pretty(&mut self) {
        let mut query = <(Entity, &Relationship, &LayoutContent)>::query();
        for (entity, relationship, layout) in query.iter(self) {
            match layout.layout_type {
                LayoutType::Canvas => println!("canvas"),
                LayoutType::Horizontal => println!("horizontal"),
                LayoutType::Vertical => println!("vertical")
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