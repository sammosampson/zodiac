use legion::*;
use legion::world::*;
use crate::components::*;

pub trait Pretty {
    fn to_pretty(&mut self);
}

impl Pretty for World {
    fn to_pretty(&mut self) {
        todo!()
    }
}

impl<'a> Pretty for SubWorld<'a> {
    fn to_pretty(&mut self) {
        todo!()
    }
}