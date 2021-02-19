use legion::*;
use zodiac::world_building::entities::*;
use zodiac::systems::measurement::*;
use zodiac_entities::components::*;
use zodiac::systems::relationships::*;

#[test]
fn system_builds_width_map() {
    let mut world = World::default();
    let mut resources = Resources::default();
    let mut schedule = Schedule::builder()
        .add_system(build_width_map_system())
        .build();

    let mut builder = WorldEntityBuilder::for_world(&mut world);
    let screen = builder.get_current_entity();
    builder.add_width_component(10);
    
    builder.create_rectangle_entity();
    let rectangle = builder.get_current_entity();
    builder.add_width_component(12);
    builder.complete_entity();

    resources.insert(create_width_map()); 
    schedule.execute(&mut world, &mut resources);

    let width_map = resources.get::<WidthMap>().unwrap();
    let screen_width = width_map.get(&screen).unwrap();
    let rectangle_width = width_map.get(&rectangle).unwrap();

    assert_eq!(screen_width.width, 10);
    assert_eq!(rectangle_width.width, 12);
}

#[test]
fn system_does_not_add_widths_already_mapped() {
    let mut world = World::default();
    let mut resources = Resources::default();
    let mut schedule = Schedule::builder()
        .add_system(build_width_map_system())
        .build();

    let mut builder = WorldEntityBuilder::for_world(&mut world);
    let screen = builder.get_current_entity();
    builder.add_width_component(12);
    builder.add_component_to_current_entity(Mapped {});

    resources.insert(create_width_map()); 
    schedule.execute(&mut world, &mut resources);
    
    assert_eq!(resources.get::<WidthMap>().unwrap().get(&screen), None);
}

#[test]
fn system_builds_height_map() {
    let mut world = World::default();
    let mut resources = Resources::default();
    let mut schedule = Schedule::builder()
        .add_system(build_height_map_system())
        .build();

    let mut builder = WorldEntityBuilder::for_world(&mut world);
    let screen = builder.get_current_entity();
    builder.add_height_component(10);
    
    builder.create_rectangle_entity();
    let rectangle = builder.get_current_entity();
    builder.add_height_component(12);
    builder.complete_entity();

    resources.insert(create_height_map()); 
    schedule.execute(&mut world, &mut resources);

    let height_map = resources.get::<HeightMap>().unwrap();
    let screen_height = height_map.get(&screen).unwrap();
    let rectangle_height = height_map.get(&rectangle).unwrap();

    assert_eq!(screen_height.height, 10);
    assert_eq!(rectangle_height.height, 12);
}


#[test]
fn system_does_not_add_heights_already_mapped() {
    let mut world = World::default();
    let mut resources = Resources::default();
    let mut schedule = Schedule::builder()
        .add_system(build_height_map_system())
        .build();

    let mut builder = WorldEntityBuilder::for_world(&mut world);
    let screen = builder.get_current_entity();
    builder.add_height_component(12);
    builder.add_component_to_current_entity(Mapped {});

    resources.insert(create_height_map()); 
    schedule.execute(&mut world, &mut resources);
    
    assert_eq!(resources.get::<HeightMap>().unwrap().get(&screen), None);
}

#[test]
fn measurement_system_measures_fixed_width_children_to_one_level() {
    let mut world = World::default();
    let mut resources = Resources::default();
    let mut schedule = Schedule::builder()
        .add_system(build_relationship_map_system())
        .add_system(build_width_map_system())
        .flush()
        .add_thread_local(measure_fixed_width_constraints_system())
        .build();

    let mut builder = WorldEntityBuilder::for_world(&mut world);
    let screen = builder.get_current_entity();
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

#[test]
fn measurement_system_measures_fixed_width_children_to_multiple_levels() {
    let mut world = World::default();
    let mut resources = Resources::default();
    let mut schedule = Schedule::builder()
        .add_system(build_relationship_map_system())
        .add_system(build_width_map_system())
        .flush()
        .add_thread_local(measure_fixed_width_constraints_system())
        .build();

    let mut builder = WorldEntityBuilder::for_world(&mut world);
    let screen = builder.get_current_entity();

    builder.create_rectangle_entity();
    builder.add_width_component(10);
    builder.complete_entity();

    builder.create_canvas_layout_content_entity();
    let layout = builder.get_current_entity();

    builder.create_circle_entity();
    builder.add_width_component(20);
    builder.complete_entity();

    builder.create_text_entity();
    builder.add_width_component(5);
    builder.complete_entity();

    builder.complete_entity();

    resources.insert(create_relationship_map()); 
    resources.insert(create_width_map()); 
    resources.insert(create_minimum_width_map()); 
    schedule.execute(&mut world, &mut resources);
    
    let width_map = resources.get::<MinimumWidthMap>().unwrap();
    assert_ne!(width_map.get(&screen), None);
    assert_eq!(width_map.get(&screen).unwrap().width, 35);
    assert_ne!(width_map.get(&layout), None);
    assert_eq!(width_map.get(&layout).unwrap().width, 25);
}

#[test]
fn measurement_system_measures_ignores_fixed_width_children_for_fixed_width_parent() {
    let mut world = World::default();
    let mut resources = Resources::default();
    let mut schedule = Schedule::builder()
        .add_system(build_relationship_map_system())
        .add_system(build_width_map_system())
        .flush()
        .add_thread_local(measure_fixed_width_constraints_system())
        .build();

    let mut builder = WorldEntityBuilder::for_world(&mut world);
    builder.add_width_component(10);
    let screen = builder.get_current_entity();

    builder.create_circle_entity();
    builder.add_width_component(20);
    builder.complete_entity();

    builder.create_text_entity();
    builder.add_width_component(5);
    builder.complete_entity();

    builder.complete_entity();

    resources.insert(create_relationship_map()); 
    resources.insert(create_width_map()); 
    resources.insert(create_minimum_width_map()); 
    schedule.execute(&mut world, &mut resources);
    
    let width_map = resources.get::<MinimumWidthMap>().unwrap();
    assert_ne!(width_map.get(&screen), None);
    assert_eq!(width_map.get(&screen).unwrap().width, 10);
}

#[test]
fn measurement_system_measures_fixed_height_children_to_one_level() {
    let mut world = World::default();
    let mut resources = Resources::default();
    let mut schedule = Schedule::builder()
        .add_system(build_relationship_map_system())
        .add_system(build_height_map_system())
        .flush()
        .add_thread_local(measure_fixed_height_constraints_system())
        .build();

    let mut builder = WorldEntityBuilder::for_world(&mut world);
    let screen = builder.get_current_entity();
    builder.create_rectangle_entity();
    builder.add_height_component(10);
    builder.complete_entity();

    builder.create_circle_entity();
    builder.add_height_component(20);
    builder.complete_entity();

    builder.create_text_entity();
    builder.complete_entity();

    resources.insert(create_relationship_map()); 
    resources.insert(create_height_map()); 
    resources.insert(create_minimum_height_map()); 
    schedule.execute(&mut world, &mut resources);
    
    let height_map = resources.get::<MinimumHeightMap>().unwrap();
    assert_ne!(height_map.get(&screen), None);
    assert_eq!(height_map.get(&screen).unwrap().height, 30);
}

#[test]
fn measurement_system_measures_fixed_height_children_to_multiple_levels() {
    let mut world = World::default();
    let mut resources = Resources::default();
    let mut schedule = Schedule::builder()
        .add_system(build_relationship_map_system())
        .add_system(build_height_map_system())
        .flush()
        .add_thread_local(measure_fixed_height_constraints_system())
        .build();

    let mut builder = WorldEntityBuilder::for_world(&mut world);
    let screen = builder.get_current_entity();

    builder.create_rectangle_entity();
    builder.add_height_component(10);
    builder.complete_entity();

    builder.create_canvas_layout_content_entity();
    let layout = builder.get_current_entity();

    builder.create_circle_entity();
    builder.add_height_component(20);
    builder.complete_entity();

    builder.create_text_entity();
    builder.add_height_component(5);
    builder.complete_entity();

    builder.complete_entity();

    resources.insert(create_relationship_map()); 
    resources.insert(create_height_map()); 
    resources.insert(create_minimum_height_map()); 
    schedule.execute(&mut world, &mut resources);
    
    let height_map = resources.get::<MinimumHeightMap>().unwrap();
    assert_ne!(height_map.get(&screen), None);
    assert_eq!(height_map.get(&screen).unwrap().height, 35);
    assert_ne!(height_map.get(&layout), None);
    assert_eq!(height_map.get(&layout).unwrap().height, 25);
}

#[test]
fn measurement_system_measures_ignores_fixed_height_children_for_fixed_height_parent() {
    let mut world = World::default();
    let mut resources = Resources::default();
    let mut schedule = Schedule::builder()
        .add_system(build_relationship_map_system())
        .add_system(build_height_map_system())
        .flush()
        .add_thread_local(measure_fixed_height_constraints_system())
        .build();

    let mut builder = WorldEntityBuilder::for_world(&mut world);
    builder.add_height_component(10);
    let screen = builder.get_current_entity();

    builder.create_circle_entity();
    builder.add_height_component(20);
    builder.complete_entity();

    builder.create_text_entity();
    builder.add_height_component(5);
    builder.complete_entity();

    builder.complete_entity();

    resources.insert(create_relationship_map()); 
    resources.insert(create_height_map()); 
    resources.insert(create_minimum_height_map()); 
    schedule.execute(&mut world, &mut resources);
    
    let height_map = resources.get::<MinimumHeightMap>().unwrap();
    assert_ne!(height_map.get(&screen), None);
    assert_eq!(height_map.get(&screen).unwrap().height, 10);
}