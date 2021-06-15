use shrev::*;
use legion::*;
use legion::systems::*;
use zodiac::*;
use crate::*;

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

    fn register_components_for_world_serializiation(&self, world_serializer: &mut WorldSerializer) {
        world_serializer.register_component::<Style>(stringify!(Style));
        world_serializer.register_component::<Span>(stringify!(Span));
        world_serializer.register_component::<TextSize>(stringify!(TextSize));
        world_serializer.register_component::<TextColour>(stringify!(TextColour));
        world_serializer.register_component::<FontSize>(stringify!(FontSize));
        world_serializer.register_component::<Colour>(stringify!(Colour));
    }
}