use shrev::*;
use legion::*;
use legion::systems::*;
use zodiac_entities::*;
use zodiac_rendering::*;
use crate::*;

pub fn standard_glium_rendering() ->
    RendereringBuilder<GliumRenderer, GliumRenderQueue> {
    RendereringBuilder::<GliumRenderer, GliumRenderQueue>::new()
}

pub fn glium_renderer() -> GliumRendererBuilder {
    GliumRendererBuilder::default()
}

#[derive(Default, Debug)]
pub struct GliumRendererBuilder {
}

impl ApplicationBundleBuilder for GliumRendererBuilder {
    fn description(&self) -> String {
        "Glium rendering".to_string()
    }
    fn setup_build_systems(&self, builder: &mut Builder) {
        builder
            .add_thread_local(event_loop_system())
            .flush();
    }

    fn setup_layout_systems(&self, _: &mut Builder) {
    }

    fn setup_rendering_systems(&self, builder: &mut Builder) {
        builder
            .add_thread_local(render_primitives_system());
    }

    fn setup_cleanup_systems(&self, _: &mut Builder) {
    }

    fn setup_final_functions(&self, _: &mut Builder) {
    }

    fn setup_resources(&self, resources: &mut Resources, _: &mut EventChannel<SystemEvent>) -> Result<(), ZodiacError>  {
        let mut event_loop = create_system_event_loop();
        
        resources.insert(create_glium_renderer(&mut event_loop)?);
        resources.insert(event_loop);
        resources.insert(create_glium_render_queue());
        
        Ok(())
    }
}