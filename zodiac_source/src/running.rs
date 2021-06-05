
use moxie::*;
use moxie::runtime::Runtime as MoxieRuntime;
use illicit::*;
use crate::changes::*;
use crate::embedding::*;
use crate::application_state::*;

pub fn create_moxie_runner<TState: State, TRootFunc: FnMut() -> RootNode<TState> +'static>(
    root_func: TRootFunc,
    state: TState) -> MoxieRunner<TState> {
    MoxieRunner::<TState>::new(root_func, state)
}
pub struct MoxieRunner<TState: State> {
    root_func: Box::<dyn FnMut() -> RootNode<TState>>,
    runtime: MoxieRuntime
}

impl<TState: State> MoxieRunner<TState> {
    fn new<TRootFunc: FnMut() -> RootNode<TState> +'static>(
        mut root_func: TRootFunc,
        default_state: TState) -> Self {
        Self {
            root_func: Box::new(move || {
                Layer::new()
                    .offer(state(|| default_state.clone()).1)
                    .offer(state(|| NodeBuildCache::default()).1)
                    .offer(state(|| SourceBuildChangeState::default()).1)
                    .enter(|| topo::root(|| root_func()))
            }),
            runtime: MoxieRuntime::new()
        }
    }    

    pub fn run_once(&mut self) -> RootNode<TState> {
        self.runtime.run_once(&mut self.root_func)
    }   
}