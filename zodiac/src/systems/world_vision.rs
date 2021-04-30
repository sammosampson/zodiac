use legion::*;
use crate::formatting::*;

pub fn log_world_view(world: &mut World, _: &mut Resources) {
    world.to_pretty();
}