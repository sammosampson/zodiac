use zodiac_entities::*;
use zodiac_layout::*;
use zodiac_rendering::*;
use zodiac_source::*;

pub fn standard_builders<TState: State, TRootFunc: FnMut() -> RootNode<TState> + Copy + Clone + 'static>(state: TState, root_func: TRootFunc) -> Vec::<Box::<dyn ApplicationBundleBuilder>> {
    vec!(
        Box::new(standard_source_building(state, root_func)),
        Box::new(standard_layout()),
        Box::new(standard_rendering()),
        Box::new(renderer()),
    )
}

#[cfg(feature = "glium_rendering")]
use zodiac_rendering_glium::*;


#[cfg(feature = "glium_rendering")]
pub fn standard_rendering() -> RendereringBuilder<GliumRenderer> {
    standard_glium_rendering()
}

#[cfg(feature = "glium_rendering")]
pub fn renderer() -> GliumRendererBuilder {
    glium_renderer()
}

#[cfg(feature = "pathfinder_rendering")]
use zodiac_rendering_pathfinder::*;

#[cfg(feature = "pathfinder_rendering")]
pub fn standard_rendering() -> RendereringBuilder<PathFinderRenderer> {
    standard_pathfinder_rendering()
}

#[cfg(feature = "pathfinder_rendering")]
pub fn renderer() -> PathFinderRendererBuilder {
    pathfinder_renderer()
}