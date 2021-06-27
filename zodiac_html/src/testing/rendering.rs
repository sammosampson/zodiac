use log::debug;
use legion::*;
use serde::*;
use legion::systems::*;
use shrev::EventChannel;
use zodiac::*;
use crate::layout::*;
use crate::size::*;
use crate::borders::*;
use crate::colour::*;
use super::systems::*;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RenderPrimitive {
    id: ComponentId,
    layout: ResolvedLayoutBox, 
    border: FullBorder,
    background_colour: BackgroundColour,
}  

impl RenderPrimitive {
    pub fn is_positioned_at(&self, left: u16, top: u16) -> bool {
        let (layout_left, layout_top) = self.layout.content_position().into();
        layout_left == left && layout_top == top 
    }
    
    pub fn has_dimensions_of(&self, width: u16, height: u16) -> bool {
        let (layout_width, layout_height) = self.layout.content_dimensions().into();
        layout_width == width && layout_height == height
    }
    
    pub fn has_border_top_of(&self, border_values: BorderValues) -> bool {
        let (top, _left, _bottom, _right, _radius) = self.border.into();
        top == BorderTop::from(border_values)
    }
    
    pub fn has_border_bottom_of(&self, border_values: BorderValues) -> bool {
        let (_top, _left, bottom, _right, _radius) = self.border.into();
        bottom == BorderBottom::from(border_values)
    }
    
    pub fn has_border_left_of(&self, border_values: BorderValues) -> bool {
        let (_top, left, _bottom, _right, _radius) = self.border.into();
        left == BorderLeft::from(border_values)
    }

    pub fn has_border_right_of(&self, border_values: BorderValues) -> bool {
        let (_top, _left, _bottom, right, _radius) = self.border.into();
        right == BorderRight::from(border_values)
    }

    pub fn has_border_radius_of(&self, border_radius: Size) -> bool {
        let (_top, _left, _bottom, _right, radius) = self.border.into();
        radius == BorderRadius::from(border_radius)
    }

    pub fn has_background_colour_of(&self, colour: Colour) -> bool {
        colour == self.background_colour.into()
    }
}

impl From<(&ComponentId, &ResolvedLayoutBox, &FullBorder, &BackgroundColour)> for RenderPrimitive {
    fn from(props: (&ComponentId, &ResolvedLayoutBox, &FullBorder, &BackgroundColour)) -> Self {
        Self {
            id: *props.0,
            layout: *props.1, 
            border: *props.2,
            background_colour: *props.3 
        }
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

        Ok(())
    }

    fn register_components_for_world_serializiation(&self, _world_serializer: &mut WorldSerializer) {
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