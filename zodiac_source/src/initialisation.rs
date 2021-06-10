use shrev::*;
use legion::*;
use legion::systems::*;
use zodiac_entities::*;
use crate::*;
use crate::application_state::*;
use crate::building::create_entity_map;
use crate::embedding::*;
use crate::running::*;

pub fn standard_source_building<TState: State, TRootFunc: FnMut() -> RootNode<TState> +'static>(state: TState, root_func: TRootFunc) -> SourceBuildBundleBuilder<TState, TRootFunc> {
    SourceBuildBundleBuilder::new(state, root_func)
}

#[derive(Debug, Copy, Clone)]
pub struct SourceBuildBundleBuilder<TState: State, TRootFunc: FnMut() -> RootNode<TState> +'static>  {
    root_func: TRootFunc,
    state: TState
}

impl<TState: State, TRootFunc: FnMut() -> RootNode<TState> +'static> SourceBuildBundleBuilder<TState, TRootFunc> {
    pub fn new(state: TState, root_func: TRootFunc) -> Self  {
        Self {
            root_func,
            state
        }
    }    
}


impl<TState: State, TRootFunc: FnMut() -> RootNode<TState> + Copy + Clone + 'static> ApplicationBundleBuilder for SourceBuildBundleBuilder<TState, TRootFunc> {
    fn description(&self) -> String {
        "standard source build".to_string()
    }

    fn setup_build_systems(&self, builder: &mut Builder) {
        builder.add_thread_local(run_moxie_system::<TState>());
    }

    fn setup_layout_systems(&self, _: &mut Builder) {
    }

    fn setup_rendering_systems(&self, _: &mut Builder) {
    }

    fn setup_cleanup_systems(&self, builder: &mut Builder) {
        builder
            .add_thread_local(remove_rebuild_system());
    }
    
    fn setup_final_functions(&self, _: &mut Builder) {
    }

    fn setup_resources(&self, resources: &mut Resources, _: &mut EventChannel<SystemEvent>) -> Result<(), ZodiacError>  {
        resources.insert(create_moxie_runner::<TState, TRootFunc>(self.root_func, self.state));
        resources.insert(create_state_repository::<TState>());
        resources.insert(create_entity_map());
        Ok(())
    }    
    
    fn register_components_for_world_serializiation(&self, _: &mut WorldSerializer) {
    }
}