use legion::*;
use legion::systems::*;
use log::debug;
use shrev::EventChannel;
use zodiac_entities::*;
use zodiac_rendering::*;
use zodiac_rendering_glium::*;

pub fn standard_test_rendering() -> RendereringBuilder<TestRenderer, GliumRenderQueue> {
    RendereringBuilder::<TestRenderer, GliumRenderQueue>::new()
}

pub fn test_renderer(dimensions: Dimensions) -> TestRendererBuilder {
    TestRendererBuilder::new(dimensions)
}

#[derive(Default, Debug)]
pub struct TestRendererBuilder {
    dimensions: Dimensions
}

impl TestRendererBuilder {
    fn new(dimensions: Dimensions) -> Self {
        Self {
            dimensions
        }
    }
}

impl ApplicationBundleBuilder for TestRendererBuilder {
    fn description(&self) -> String {
        "test rendering".to_string()
    }
    fn setup_build_systems(&self, _: &mut Builder) {
    }

    fn setup_layout_systems(&self, _: &mut Builder) {
    }

    fn setup_rendering_systems(&self, _: &mut Builder) {
    }

    fn setup_cleanup_systems(&self, _: &mut Builder) {
    }

    fn setup_final_functions(&self, _: &mut Builder) {
    }

    fn setup_resources(&self, resources: &mut Resources, _: &mut EventChannel<SystemEvent>) -> Result<(), ZodiacError>  {
        resources.insert(create_test_renderer(self.dimensions));
        resources.insert(create_glium_render_queue());

        Ok(())
    }
}

fn create_test_renderer(dimensions: Dimensions) -> TestRenderer {
    TestRenderer::new(dimensions)
}

pub struct TestRenderer {
    dimensions: Dimensions
}

impl TestRenderer {
    fn new(dimensions: Dimensions) -> Self {
        Self {
            dimensions
        }
    }
}

impl Renderer for TestRenderer {
    fn get_window_dimensions(&self) -> Dimensions {
        debug!("Getting dimensions");
        self.dimensions
    }
}