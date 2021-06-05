
use legion::systems::CommandBuffer;
use zodiac_entities::*;
use crate::changes::*;
use crate::application_state::*;
use crate::running::MoxieRunner;

#[legion::system(simple)]
#[read_component(Relationship)]
#[write_component(Relationship)]
pub fn run_moxie<TState: State>(
    command_buffer: &mut CommandBuffer,
    #[resource] moxie_runner: &mut MoxieRunner<TState>,
    #[resource] state_repository: &mut StateRepository<TState>, 
    #[resource] relationship_map: &mut RelationshipMap, 
    #[resource] entity_map: &mut EntityMap) {

    let root_node = moxie_runner.run_once();
    
    let mut maps = SourceBuildMaps {
        entity_map,
        relationship_map
    };

    let (changes, state_snapshot) = root_node.changes();
    changes.apply(command_buffer, &mut maps);
    state_repository.set(state_snapshot);
}