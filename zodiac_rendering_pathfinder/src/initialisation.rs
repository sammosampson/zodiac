use shrev::*;
use legion::*;
use legion::systems::*;
use zodiac_entities::*;
use zodiac_rendering::*;
use crate::*;

pub fn standard_pathfinder_rendering() ->
    RendereringBuilder<PathFinderRenderer, PathFinderRenderQueue> {
    RendereringBuilder::<PathFinderRenderer, PathFinderRenderQueue>::new()
}

pub fn pathfinder_renderer() -> PathFinderRendererBuilder {
    PathFinderRendererBuilder::default()
}

#[derive(Default, Debug)]
pub struct PathFinderRendererBuilder {
}

impl ApplicationBundleBuilder for PathFinderRendererBuilder {
    fn description(&self) -> String {
        "path finder rendering".to_string()
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

    fn setup_resources(&self, resources: &mut Resources, event_channel: &mut EventChannel<SystemEvent>) -> Result<(), ZodiacError>  {
        let mut event_loop = create_system_event_loop();
        
        resources.insert(create_pathfinder_renderer(&mut event_loop)?);
        resources.insert(event_loop);
        resources.insert(create_pathfinder_event_reader_registry(event_channel));
        resources.insert(create_pathfinder_render_queue());
        
        Ok(())
    }
}