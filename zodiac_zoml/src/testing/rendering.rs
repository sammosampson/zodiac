use legion::*;
use serde::*;
use legion::systems::*;
use log::debug;
use shrev::EventChannel;
use zodiac::*;
use crate::testing::*;
use super::systems::rendering::*;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RenderPrimitive {
    render_type: RenderPrimitiveType
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum RenderPrimitiveType {
    TestRenderable([u16; 2], [u16; 2])
}

pub fn create_test_render_queue() -> TestRenderQueue {
    TestRenderQueue::default()
}              

#[derive(Default)]
pub struct TestRenderQueue {
}

impl RenderPrimitive {
    pub fn test_renderable (position: [u16; 2], dimensions: [u16; 2]) -> RenderPrimitive {
        RenderPrimitive { render_type:  RenderPrimitiveType::TestRenderable(position, dimensions) } 
    }
}

impl TestRenderQueue {  
    fn queue_primitive_for_render(&mut self, command_buffer: &mut CommandBuffer, entity: &Entity, to_queue: RenderPrimitive) {
        debug!("Queueing: {:?}", to_queue);
        command_buffer.add_component(*entity, to_queue)
    }
}

impl TestRenderQueue {
    pub fn queue_for_render(
        &mut self,
        command_buffer: &mut CommandBuffer,
        entity: &Entity,
        position: [u16; 2],
        dimensions: [u16; 2]) {
        self.queue_primitive_for_render(
            command_buffer,
            entity,
            RenderPrimitive::test_renderable(
                position,
                dimensions));
    }
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
    fn setup_build_systems(&self, builder: &mut Builder) {
        builder
            .add_thread_local(initial_window_size_notification_system::<TestRenderer>());
    }

    fn setup_layout_systems(&self, _: &mut Builder) {
    }

    fn setup_rendering_systems(&self, builder: &mut Builder) {
        builder
            .add_thread_local(queue_render_primitives_system());
    }

    fn setup_cleanup_systems(&self, _: &mut Builder) {
    }

    fn setup_final_functions(&self, _: &mut Builder) {
    }

    fn setup_resources(&self, resources: &mut Resources, _: &mut EventChannel<SystemEvent>) -> Result<(), ZodiacError>  {
        resources.insert(create_test_renderer(self.dimensions));
        resources.insert(create_test_render_queue());

        Ok(())
    }

    fn register_components_for_world_serializiation(&self, world_serializer: &mut WorldSerializer) {
        world_serializer.register_component::<TestRenderable>(stringify!(TestRenderable));
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