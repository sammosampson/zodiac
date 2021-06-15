use shrev::*;
use legion::*;
use legion::systems::*;
use zodiac::*;
use crate::components::*;
use crate::systems::measurement::*;

pub fn zoml_builder() -> ZomlBuilder {
    ZomlBuilder::default()
}

#[derive(Default, Debug)]
pub struct ZomlBuilder {
}

impl ApplicationBundleBuilder for ZomlBuilder {
    fn description(&self) -> String {
        "Html".to_string()
    }

    fn setup_build_systems(&self, _: &mut Builder) {
    }

    fn setup_layout_systems(&self, builder: &mut Builder) {
        builder
            .add_system(build_width_and_height_maps_from_radius_system());
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
        world_serializer.register_component::<Content>(stringify!(Content));
        world_serializer.register_component::<Radius>(stringify!(Radius));
        world_serializer.register_component::<Colour>(stringify!(Colour));
        world_serializer.register_component::<StrokeWidth>(stringify!(StrokeWidth));
        world_serializer.register_component::<StrokeColour>(stringify!(StrokeColour));
        world_serializer.register_component::<CornerRadii>(stringify!(CornerRadii));  
        world_serializer.register_component::<Circle>(stringify!(Circle));
        world_serializer.register_component::<Rectangle>(stringify!(Rectangle));
    }
}