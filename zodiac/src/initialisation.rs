use zodiac_entities::*;
use zodiac_layout::*;
use zodiac_source::*;

pub fn standard_builders<TState: State, TRootFunc: FnMut() -> RootNode<TState> + Copy + Clone + 'static>(state: TState, root_func: TRootFunc) -> Vec::<Box::<dyn ApplicationBundleBuilder>> {
    vec!(
        Box::new(standard_source_building(state, root_func)),
        Box::new(standard_layout()),
    )
}