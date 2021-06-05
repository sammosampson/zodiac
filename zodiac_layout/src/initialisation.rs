use shrev::*;
use legion::*;
use legion::systems::*;
use zodiac_entities::*;
use crate::*;

pub fn standard_layout() -> LayoutBundleBuilder {
    LayoutBundleBuilder::default()
}

#[derive(Default, Debug, Copy, Clone)]
pub struct LayoutBundleBuilder {
}

impl ApplicationBundleBuilder for LayoutBundleBuilder {
    fn description(&self) -> String {
        "standard layout".to_string()
    }
    
    fn setup_build_systems(&self, _: &mut Builder) {
    }

    fn setup_layout_systems(&self, builder: &mut Builder) {
        builder
            .add_system(resize_screen_system())
            .add_system(resize_after_rebuild_system())
            .flush()
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
            .add_system(build_width_and_height_maps_from_radius_system())
            .add_system(remove_from_layout_type_map_system())
            .add_system(build_layout_type_map_system())
            .flush()
            .add_system(measure_fixed_width_constraints_system())
            .add_system(measure_fixed_height_constraints_system())
            .flush()
            .add_system(resize_system());
    }

    fn setup_rendering_systems(&self, _: &mut Builder) {
    }

    fn setup_cleanup_systems(&self, builder: &mut Builder) { 
        builder
            .add_thread_local(remove_layout_change_system())
            .add_thread_local(remove_resized_system());

    }

    fn setup_final_functions(&self, _: &mut Builder) {
    }
    
    fn setup_resources(&self, resources: &mut Resources, event_channel: &mut EventChannel<SystemEvent>) -> Result<(), ZodiacError>  {
        resources.insert(create_layout_event_reader_registry(event_channel));
        resources.insert(create_text_colour_map());
        resources.insert(create_layout_type_map());
        resources.insert(create_left_offset_map());
        resources.insert(create_top_offset_map());
        resources.insert(create_width_map());
        resources.insert(create_height_map());
        resources.insert(create_minimum_width_map());
        resources.insert(create_minimum_height_map());
        
        Ok(())
    }
}