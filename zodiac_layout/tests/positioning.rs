use legion::*;
use zodiac_entities::*;
use zodiac_layout::*;

#[test]
fn system_builds_left_offset_map() {
    let mut world = World::default();
    let mut resources = Resources::default();
    let mut schedule = Schedule::builder()
        .add_system(build_left_offset_map_system())
        .build();

    let mut builder = world_entity_builder_for_world_with_root(&mut world);
    let screen = builder.get_current_entity();
    builder.add_left_component(10);
    
    builder.create_rectangle_entity();
    let rectangle = builder.get_current_entity();
    builder.add_left_component(12);
    builder.complete_entity();

    resources.insert(create_left_offset_map()); 
    schedule.execute(&mut world, &mut resources);

    let offset_map = resources.get::<LeftOffsetMap>().unwrap();
    let screen_offset = offset_map.get(&screen).unwrap();
    let rectangle_offset = offset_map.get(&rectangle).unwrap();

    assert_eq!(screen_offset.left, 10);
    assert_eq!(rectangle_offset.left, 12);
}

#[test]
fn system_does_not_add_left_offsets_already_mapped() {
    let mut world = World::default();
    let mut resources = Resources::default();
    let mut schedule = Schedule::builder()
        .add_system(build_left_offset_map_system())
        .build();

    let mut builder = world_entity_builder_for_world_with_root(&mut world);
    let screen = builder.get_current_entity();
    builder.add_left_component(12);
    builder.add_component_to_current_entity(Mapped {});

    resources.insert(create_left_offset_map());  
    schedule.execute(&mut world, &mut resources);
    
    assert_eq!(resources.get::<LeftOffsetMap>().unwrap().get(&screen), None);
}

#[test]
fn system_builds_top_offset_map() {
    let mut world = World::default();
    let mut resources = Resources::default();
    let mut schedule = Schedule::builder()
        .add_system(build_top_offset_map_system())
        .build();

    let mut builder = world_entity_builder_for_world_with_root(&mut world);
    let screen = builder.get_current_entity();
    builder.add_top_component(10);
    
    builder.create_rectangle_entity();
    let rectangle = builder.get_current_entity();
    builder.add_top_component(12);
    builder.complete_entity();

    resources.insert(create_top_offset_map()); 
    schedule.execute(&mut world, &mut resources);

    let offset_map = resources.get::<TopOffsetMap>().unwrap();
    let screen_offset = offset_map.get(&screen).unwrap();
    let rectangle_offset = offset_map.get(&rectangle).unwrap();

    assert_eq!(screen_offset.top, 10);
    assert_eq!(rectangle_offset.top, 12);
}

#[test]
fn system_does_not_add_top_offsets_already_mapped() {
    let mut world = World::default();
    let mut resources = Resources::default();
    let mut schedule = Schedule::builder()
        .add_system(build_top_offset_map_system())
        .build();

    let mut builder = world_entity_builder_for_world_with_root(&mut world);
    let screen = builder.get_current_entity();
    builder.add_top_component(12);
    builder.add_component_to_current_entity(Mapped {});

    resources.insert(create_top_offset_map());  
    schedule.execute(&mut world, &mut resources);
    
    assert_eq!(resources.get::<TopOffsetMap>().unwrap().get(&screen), None);
}