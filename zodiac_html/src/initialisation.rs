use shrev::*;
use legion::*;
use legion::systems::*;
use zodiac::*;
use crate::*;
use crate::systems::*;

pub fn html_builder() -> HtmlBuilder {
    HtmlBuilder::default()
}

#[derive(Default, Debug)]
pub struct HtmlBuilder {
}

impl ApplicationBundleBuilder for HtmlBuilder {
    fn description(&self) -> String {
        "html webrender rendering".to_string()
    }

    fn setup_build_systems(&self, _: &mut Builder) {
    }

    fn setup_layout_systems(&self, builder: &mut Builder) {
        builder
            .add_system(deconstruct_border_system())
            .flush()
            .add_system(deconstruct_border_colour_system())
            .add_system(deconstruct_border_width_system())
            .add_system(deconstruct_border_style_system())
            .flush()
            .add_system(upconstruct_border_top_system())
            .add_system(upconstruct_border_left_system())
            .add_system(upconstruct_border_bottom_system())
            .add_system(upconstruct_border_right_system())
            .flush()
            .add_system(compose_full_border_system())
            .flush();
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

    fn register_components_for_world_serializiation(&self, world_serializer: &mut WorldSerializer) {
        world_serializer.register_component::<Style>(stringify!(Style));
        world_serializer.register_component::<Span>(stringify!(Span));
        world_serializer.register_component::<BorderWidth>(stringify!(BorderWidth));
        world_serializer.register_component::<BorderColour>(stringify!(BorderColour));
        world_serializer.register_component::<BorderTopStyle>(stringify!(BorderTopStyle));
        world_serializer.register_component::<BorderLeftStyle>(stringify!(BorderLeftStyle));
        world_serializer.register_component::<BorderBottomStyle>(stringify!(BorderBottomStyle));
        world_serializer.register_component::<BorderRightStyle>(stringify!(BorderRightStyle));
        world_serializer.register_component::<BorderStyles>(stringify!(BorderStyle));
        world_serializer.register_component::<BorderStyle>(stringify!(BorderStyles));
        world_serializer.register_component::<Border>(stringify!(Border));
        world_serializer.register_component::<BackgroundColour>(stringify!(BackgroundColour));
        world_serializer.register_component::<Size>(stringify!(Size));
        world_serializer.register_component::<Colour>(stringify!(Colour));
    }
}