use legion::*;
use serde::*;
use legion::systems::*;
use log::debug;
use shrev::EventChannel;
use zodiac_entities::*;
use zodiac_rendering::*;

pub fn standard_test_rendering() -> RendereringBuilder<TestRenderer, TestRenderQueue> {
    RendereringBuilder::<TestRenderer, TestRenderQueue>::new()
}


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RenderPrimitive {
    render_type: RenderPrimitiveType
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum RenderPrimitiveType {
    Rectangle([u16; 2], [u16; 2], [f32; 4], [f32; 4], f32, [u16; 4]),
    Circle([u16; 2], u16, [f32; 4], [f32; 4], f32),
    Text([u16; 2], [u16; 2], [f32; 4], String, f32),
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
        inner_colour: [f32; 4],
        outer_colour: [f32; 4],
        stroke_width: f32,
        corner_radii: [u16; 4]) -> Self {
        RenderPrimitive { render_type:  RenderPrimitiveType::Rectangle(position, dimensions, inner_colour, outer_colour, stroke_width, corner_radii) }
    }

    pub fn circle(
        position: [u16; 2],
        radius: u16,
        inner_colour: [f32; 4],
        outer_colour: [f32; 4],
        stroke_width: f32) -> Self {
        RenderPrimitive { render_type:  RenderPrimitiveType::Circle(position, radius, inner_colour, outer_colour, stroke_width) }
    }

    pub fn text(
        position: [u16; 2],
        dimensions: [u16; 2],
        colour: [f32; 4],
        text: String,
        font_size: f32) -> Self {
        RenderPrimitive { render_type: RenderPrimitiveType::Text(position, dimensions, colour, text, font_size) }
    }
}

impl TestRenderQueue {  
    fn queue_primitive_for_render(&mut self, command_buffer: &mut CommandBuffer, entity: &Entity, to_queue: RenderPrimitive) {
        debug!("Queueing: {:?}", to_queue);
        command_buffer.add_component(*entity, to_queue)
    }
}

impl RenderQueue for TestRenderQueue {
    fn queue_rectangle_for_render(
        &mut self,
        command_buffer: &mut CommandBuffer,
        entity: &Entity,
        position: [u16; 2],
        dimensions: [u16; 2],
        inner_colour: [f32; 4],
        outer_colour: [f32; 4],
        stroke_width: f32,
        corner_radii: [u16; 4]) {
        self.queue_primitive_for_render(
            command_buffer,
            entity,
            RenderPrimitive::rectangle(
                position,
                dimensions,
                inner_colour,
                outer_colour,
                stroke_width,
                corner_radii));
    }

    fn queue_circle_for_render(
        &mut self,
        command_buffer: &mut CommandBuffer,
        entity: &Entity,
        position: [u16; 2],
        radius: u16,
        inner_colour: [f32; 4],
        outer_colour: [f32; 4],
        stroke_width: f32) {
        self.queue_primitive_for_render(
            command_buffer,
            entity,
            RenderPrimitive::circle(
                position,
                radius,
                inner_colour,
                outer_colour,
                stroke_width));
    }
    
    fn queue_text_for_render(
        &mut self,
        command_buffer: &mut CommandBuffer,
        entity: &Entity,
        position: [u16; 2],
        dimensions: [u16; 2],
        colour: [f32; 4],
        text: String,
        font_size: f32) {
        self.queue_primitive_for_render(
            command_buffer,
            entity,
            RenderPrimitive::text(
                position,
                dimensions,
                colour,
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
        resources.insert(create_test_render_queue());

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