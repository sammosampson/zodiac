use shrev::*;
use legion::*;
use legion::systems::*;
use zodiac::*;
use crate::components::*;
use crate::systems::*;
use crate::layout::*;

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

    fn setup_build_systems(&self, builder: &mut Builder) {
        builder.add_system(add_root_canvas_system());
    }

    fn setup_layout_systems(&self, builder: &mut Builder) {
        builder
            .add_system(resize_screen_system())
            .add_system(resize_after_rebuild_system())
            .add_system(build_width_and_height_maps_from_radius_system())
            .add_system(remove_from_left_offset_map_system())
            .add_system(build_left_offset_map_system())
            .add_system(remove_from_top_offset_map_system())
            .add_system(build_top_offset_map_system())
            .add_system(remove_from_minimum_width_map_system())
            .add_system(remove_from_width_map_system())
            .add_system(build_width_map_system())
            .add_system(remove_from_minimum_height_map_system())
            .add_system(remove_from_height_map_system())
            .add_system(build_height_map_system())
            .add_system(remove_from_layout_type_map_system())
            .add_system(build_layout_type_map_system())
            .flush()
            .add_system(measure_fixed_width_constraints_system())
            .add_system(measure_fixed_height_constraints_system())
            .flush()
            .add_system(resize_system());        
    }

    fn setup_rendering_systems(&self, _builder: &mut Builder) {
    }

    fn setup_cleanup_systems(&self, builder: &mut Builder) {
        builder.add_thread_local(remove_layout_change_system());
    }

    fn setup_final_functions(&self, _: &mut Builder) {
    }

    fn setup_resources(&self, resources: &mut Resources, event_channel: &mut EventChannel<SystemEvent>) -> Result<(), ZodiacError>  {
        resources.insert(create_layout_event_reader_registry(event_channel));
        resources.insert(create_layout_type_map());
        resources.insert(create_left_offset_map());
        resources.insert(create_top_offset_map());
        resources.insert(create_width_map());
        resources.insert(create_height_map());
        resources.insert(create_minimum_width_map());
        resources.insert(create_minimum_height_map());
        
        Ok(())
    }

    fn register_components_for_world_serializiation(&self, world_serializer: &mut WorldSerializer) {
        world_serializer.register_component::<CurrentLayoutConstraints>(stringify!(CurrentLayoutConstraints));
        world_serializer.register_component::<LayoutRequest>(stringify!(LayoutRequest));
        world_serializer.register_component::<Left>(stringify!(Left));
        world_serializer.register_component::<Top>(stringify!(Top));
        world_serializer.register_component::<Width>(stringify!(Width));
        world_serializer.register_component::<MinimumWidth>(stringify!(MinimumWidth));
        world_serializer.register_component::<Height>(stringify!(Height));
        world_serializer.register_component::<MinimumHeight>(stringify!(MinimumHeight));
        world_serializer.register_component::<LayoutChange>(stringify!(LayoutChange));
        world_serializer.register_component::<LayoutType>(stringify!(LayoutType));
        world_serializer.register_component::<LayoutContent>(stringify!(LayoutContent));
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