use shrev::*;
use legion::*;
use legion::systems::*;
use zodiac::*;

pub fn webrender_renderer() -> WebRenderRendererBuilder {
    WebRenderRendererBuilder::default()
}

#[derive(Default, Debug)]
pub struct WebRenderRendererBuilder {
}

impl ApplicationBundleBuilder for WebRenderRendererBuilder {
    fn description(&self) -> String {
        "webrender rendering".to_string()
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

    fn setup_resources(&self, _: &mut Resources, _: &mut EventChannel<SystemEvent>) -> Result<(), ZodiacError>  {
        Ok(())
    }

    fn register_components_for_world_serializiation(&self, _: &mut WorldSerializer) {
    }
}