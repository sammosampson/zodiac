use std::vec;

use shrev::*;
use legion::*;
use legion::systems::*;
use zodiac::*;
use zodiac_html::*;

use crate::events::*;
use crate::rendering::*;
use crate::systems::*;
use crate::render_primitive::*;

pub fn html_webrender_rendering() -> Vec::<Box::<dyn ApplicationBundleBuilder>> {
    vec!(
        Box::new(html_webrender_renderer_builder()),
        Box::new(html_builder())
    )
}

fn html_webrender_renderer_builder() -> HtmlWebRenderRendererBuilder {
    HtmlWebRenderRendererBuilder::default()
}

#[derive(Default, Debug)]
pub struct HtmlWebRenderRendererBuilder {
}

impl ApplicationBundleBuilder for HtmlWebRenderRendererBuilder {
    fn description(&self) -> String {
        "html webrender rendering".to_string()
    }

    fn setup_build_systems(&self, builder: &mut Builder) {
        builder
            .add_thread_local(initial_window_size_notification_system::<HtmlWebRenderRenderer>())
            .flush()
            .add_thread_local(event_loop_system());
    }

    fn setup_layout_systems(&self, _: &mut Builder) {
    }

    fn setup_rendering_systems(&self, builder: &mut Builder) {
        builder
            .add_system(add_render_primitives_system())
            .flush()
            .add_system(layout_render_primitives_system())
            .add_system(rebuild_render_primitives_system())
            .flush()
            .add_thread_local(render_primitives_system());
    }

    fn setup_cleanup_systems(&self, _: &mut Builder) {
    }

    fn setup_final_functions(&self, _: &mut Builder) {
    }

    fn setup_resources(&self, resources: &mut Resources, _: &mut EventChannel<SystemEvent>) -> Result<(), ZodiacError>  {
        let event_loop = create_system_event_loop();
        resources.insert(create_webrender_renderer(&event_loop)?);
        resources.insert(event_loop);
        
        Ok(())
    }

    fn register_components_for_world_serializiation(&self, world_serializer: &mut WorldSerializer) {
        world_serializer.register_component::<RenderPrimitiveBorder>(stringify!(RenderPrimitiveBorder));
        world_serializer.register_component::<RenderPrimitive>(stringify!(RenderPrimitive));
        
    }
}