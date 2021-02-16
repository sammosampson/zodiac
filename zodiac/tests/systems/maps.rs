use legion::*;
use zodiac::world_building::entities::*;
use zodiac::systems::maps::*;
use zodiac_entities::components::*;

#[test]
fn system_builds_relationship_map() {
    let mut world = World::default();
    let mut resources = Resources::default();
    let mut schedule = Schedule::builder()
        .add_system(build_relationship_map_system())
        .build();

    let mut builder = WorldEntityBuilder::for_world(&mut world);
    let screen = builder.get_current_entity();
    
    builder.create_rectangle_entity();
    let rectangle = builder.get_current_entity();
    builder.complete_entity();

    builder.create_circle_entity();
    let circle = builder.get_current_entity();
    builder.complete_entity();

    resources.insert(create_relationship_map()); 
    schedule.execute(&mut world, &mut resources);

    let relationship_map = resources.get::<RelationshipMap>().unwrap();
    let screen_relations = relationship_map.get(&screen).unwrap();
    let rectangle_relations = relationship_map.get(&rectangle).unwrap();
    let circle_relations = relationship_map.get(&circle).unwrap();

    assert_eq!(screen_relations.parent, None);
    assert_eq!(screen_relations.next_sibling, None);
    assert_eq!(screen_relations.first_child, Some(rectangle));
    assert_eq!(screen_relations.last_child, Some(circle));

    assert_eq!(rectangle_relations.parent, Some(screen));
    assert_eq!(rectangle_relations.next_sibling, Some(circle));
    assert_eq!(rectangle_relations.first_child, None);
    assert_eq!(rectangle_relations.last_child, None);

    assert_eq!(circle_relations.parent, Some(screen));
    assert_eq!(circle_relations.next_sibling, None);
    assert_eq!(circle_relations.first_child, None);
    assert_eq!(circle_relations.last_child, None);
}

#[test]
fn system_does_not_add_relationships_already_mapped() {
    let mut world = World::default();
    let mut resources = Resources::default();
    let mut schedule = Schedule::builder()
        .add_system(build_relationship_map_system())
        .build();

    let mut builder = WorldEntityBuilder::for_world(&mut world);
    let screen = builder.get_current_entity();
    builder.add_component_to_current_entity(RelationshipMapped {});

    resources.insert(create_relationship_map()); 
    schedule.execute(&mut world, &mut resources);
    
    assert_eq!(resources.get::<RelationshipMap>().unwrap().get(&screen), None);
}

#[test]
fn system_builds_left_offset_map() {
    let mut world = World::default();
    let mut resources = Resources::default();
    let mut schedule = Schedule::builder()
        .add_system(build_left_offset_map_system())
        .build();

    let mut builder = WorldEntityBuilder::for_world(&mut world);
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

    let mut builder = WorldEntityBuilder::for_world(&mut world);
    let screen = builder.get_current_entity();
    builder.add_left_component(12);
    builder.add_component_to_current_entity(LeftOffsetMapped {});

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

    let mut builder = WorldEntityBuilder::for_world(&mut world);
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

    let mut builder = WorldEntityBuilder::for_world(&mut world);
    let screen = builder.get_current_entity();
    builder.add_top_component(12);
    builder.add_component_to_current_entity(TopOffsetMapped {});

    resources.insert(create_top_offset_map());  
    schedule.execute(&mut world, &mut resources);
    
    assert_eq!(resources.get::<TopOffsetMap>().unwrap().get(&screen), None);
}

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
    builder.add_component_to_current_entity(WidthMapped {});

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
    builder.add_component_to_current_entity(HeightMapped {});

    resources.insert(create_height_map()); 
    schedule.execute(&mut world, &mut resources);
    
    assert_eq!(resources.get::<HeightMap>().unwrap().get(&screen), None);
}