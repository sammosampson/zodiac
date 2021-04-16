use zodiac::initialisation::*;
use zodiac_layout::*;
use zodiac_source_filesystem::*;
use zodiac_rendering::*;

fn main() {
    pretty_env_logger::init();

    Application::new()
        .with_builder(standard_source_file_building("examples\\assets\\test_zods"))
        .with_builder(standard_source_building())
        .with_builder(standard_layout())
        .with_builder(standard_rendering())
        .with_builder(renderer())
        .build()
        .unwrap()
        .run_until_closed();
}

#[cfg(feature = "glium_rendering")]
use zodiac_rendering_glium::*;

#[cfg(feature = "glium_rendering")]
pub fn standard_rendering() -> RendereringBuilder<GliumRenderer, GliumRenderQueue> {
    standard_glium_rendering()
}

#[cfg(feature = "glium_rendering")]
pub fn renderer() -> GliumRendererBuilder {
    glium_renderer()
}

#[cfg(feature = "pathfinder_rendering")]
use zodiac_rendering_pathfinder::*;

#[cfg(feature = "pathfinder_rendering")]
pub fn standard_rendering() -> RendereringBuilder<PathFinderRenderer, PathFinderRenderQueue> {
    standard_pathfinder_rendering()
}

#[cfg(feature = "pathfinder_rendering")]
pub fn renderer() -> PathFinderRendererBuilder {
    pathfinder_renderer()
}
