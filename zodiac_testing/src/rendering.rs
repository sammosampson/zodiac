use legion::*;
use serde::*;
use legion::systems::*;
use log::debug;
use shrev::EventChannel;
use zodiac::*;
use super::*;
use super::systems::rendering::*;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RenderPrimitive {
    render_type: RenderPrimitiveType
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum RenderPrimitiveType {
    Rectangle([u16; 2], [u16; 2], [u8; 4], [u8; 4], u16, [u16; 4]),
    Circle([u16; 2], u16, [u8; 4], [u8; 4], u16),
    Text([u16; 2], [u16; 2], [u8; 4], String, u8),
}

pub fn create_test_render_queue() -> TestRenderQueue {
    TestRenderQueue::default()
}              

#[derive(Default)]
pub struct TestRenderQueue {
}

impl RenderPrimitive {
    pub fn rectangle(
        position: [u16; 2],
        dimensions: [u16; 2],
        inner_colour: [u8; 4],
        outer_colour: [u8; 4],
        stroke_width: u16,
        corner_radii: [u16; 4]) -> Self {
        RenderPrimitive { render_type:  RenderPrimitiveType::Rectangle(position, dimensions, inner_colour, outer_colour, stroke_width, corner_radii) }
    }

    pub fn circle(
        position: [u16; 2],
        radius: u16,
        inner_colour: [u8; 4],
        outer_colour: [u8; 4],
        stroke_width: u16) -> Self {
        RenderPrimitive { render_type:  RenderPrimitiveType::Circle(position, radius, inner_colour, outer_colour, stroke_width) }
    }

    pub fn text(
        position: [u16; 2],
        dimensions: [u16; 2],
        colour: [u8; 4],
        text: String,
        font_size: u8) -> Self {
        RenderPrimitive { render_type: RenderPrimitiveType::Text(position, dimensions, colour, text, font_size) }
    }
}

impl TestRenderQueue {  
    fn queue_primitive_for_render(&mut self, command_buffer: &mut CommandBuffer, entity: &Entity, to_queue: RenderPrimitive) {
        debug!("Queueing: {:?}", to_queue);
        command_buffer.add_component(*entity, to_queue)
    }
}

impl TestRenderQueue {
    pub fn queue_rectangle_for_render(
        &mut self,
        command_buffer: &mut CommandBuffer,
        entity: &Entity,
        position: [u16; 2],
        dimensions: [u16; 2],
        inner_colour: Colour,
        outer_colour: StrokeColour,
        stroke_width: u16,
        corner_radii: [u16; 4]) {
        self.queue_primitive_for_render(
            command_buffer,
            entity,
            RenderPrimitive::rectangle(
                position,
                dimensions,
                inner_colour.into(),
                outer_colour.into(),
                stroke_width,
                corner_radii));
    }

    pub fn queue_circle_for_render(
        &mut self,
        command_buffer: &mut CommandBuffer,
        entity: &Entity,
        position: [u16; 2],
        radius: u16,
        inner_colour: Colour,
        outer_colour: StrokeColour,
        stroke_width: u16) {
        self.queue_primitive_for_render(
            command_buffer,
            entity,
            RenderPrimitive::circle(
                position,
                radius,
                inner_colour.into(),
                outer_colour.into(),
                stroke_width));
    }
    
    pub fn queue_text_for_render(
        &mut self,
        command_buffer: &mut CommandBuffer,
        entity: &Entity,
        position: [u16; 2],
        dimensions: [u16; 2],
        colour: Colour,
        text: String,
        font_size: u8) {
        self.queue_primitive_for_render(
            command_buffer,
            entity,
            RenderPrimitive::text(
                position,
                dimensions,
                colour.into(),
                text,
                font_size));
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
            .add_thread_local(queue_render_rectangle_primitives_system())
            .add_thread_local(queue_render_circle_primitives_system())
            .add_thread_local(queue_render_text_primitives_system());
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
        world_serializer.register_component::<Circle>(stringify!(Circle));
        world_serializer.register_component::<Rectangle>(stringify!(Rectangle));
        world_serializer.register_component::<Text>(stringify!(Text));
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