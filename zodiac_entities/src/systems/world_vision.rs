use legion::*;
use crate::*;

pub fn log_world_view(world: &mut World, resources: &mut Resources) {
    let mut world_serializer = resources.get_mut::<WorldSerializer>().unwrap();
    world_serializer.log_world(world);
}