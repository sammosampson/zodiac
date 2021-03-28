pub mod tokenization;

use legion::*;
use legion::world::*;
use legion::systems::*;
use zodiac_entities::components::*;

#[system(simple)]
#[read_component(SourceFileChange)]
#[read_component(SourceFileRemoval)]
pub fn create_abstract_syntax(world: &mut SubWorld, command_buffer: &mut CommandBuffer) {
}
