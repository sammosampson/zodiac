use shrev::*;
use legion::*;
use legion::systems::*;
use zodiac::*;
use crate::components::*;
use crate::*;

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
            .add_thread_local(initial_window_size_notification_system::<GliumRenderer>())
            .flush()
            .add_thread_local(event_loop_system())
            .flush();
    }

    fn setup_layout_systems(&self, builder: &mut Builder) {
        builder
            .add_system(build_width_and_height_maps_from_radius_system());
    }

    fn setup_rendering_systems(&self, builder: &mut Builder) {
        builder
            .add_thread_local(queue_render_rectangle_primitives_system())
            .add_thread_local(queue_render_circle_primitives_system())
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

    fn register_components_for_world_serializiation(&self, world_serializer: &mut WorldSerializer) {
        world_serializer.register_component::<Content>(stringify!(Content));
        world_serializer.register_component::<Radius>(stringify!(Radius));
        world_serializer.register_component::<Colour>(stringify!(Colour));
        world_serializer.register_component::<StrokeWidth>(stringify!(StrokeWidth));
        world_serializer.register_component::<StrokeColour>(stringify!(StrokeColour));
        world_serializer.register_component::<CornerRadii>(stringify!(CornerRadii));  
        world_serializer.register_component::<RenderPrimitive>(stringify!(RenderPrimitive));
        world_serializer.register_component::<Circle>(stringify!(Circle));
        world_serializer.register_component::<Rectangle>(stringify!(Rectangle));
    }
}