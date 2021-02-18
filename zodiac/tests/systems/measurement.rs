use legion::*;
use zodiac::world_building::entities::*;
use zodiac::systems::measurement::*;
use zodiac_entities::components::*;
use zodiac::systems::maps::*;

#[test]
fn measurement_system_measures_fixed_width_children_to_one_level() {
    let mut world = World::default();
    let mut resources = Resources::default();
    let mut schedule = Schedule::builder()
        .add_system(build_relationship_map_system())
        .add_system(build_width_map_system())
        .flush()
        .add_thread_local(measure_fixed_widths_system())
        .build();

    let mut builder = WorldEntityBuilder::for_world(&mut world);
    let screen = builder.get_current_entity();
    builder.add_component_to_current_entity(ResizeRequest {});
    builder.create_rectangle_entity();
    builder.add_width_component(10);
    builder.complete_entity();

    builder.create_circle_entity();
    builder.add_width_component(20);
    builder.complete_entity();

    builder.create_text_entity();
    builder.complete_entity();

    resources.insert(create_relationship_map()); 
    resources.insert(create_width_map()); 
    resources.insert(create_minimum_width_map()); 
    schedule.execute(&mut world, &mut resources);
    
    let width_map = resources.get::<MinimumWidthMap>().unwrap();
    assert_ne!(width_map.get(&screen), None);
    assert_eq!(width_map.get(&screen).unwrap().width, 30);
}