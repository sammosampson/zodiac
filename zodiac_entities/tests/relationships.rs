use legion::*;
use zodiac_entities::*;

#[test]
fn system_builds_relationship_map() {
    let mut world = World::default();
    let mut resources = Resources::default();
    let mut schedule = Schedule::builder()
        .add_system(build_relationship_map_system())
        .build();

    let mut builder = world_entity_builder_for_world_with_root(&mut world);
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

    let mut builder = world_entity_builder_for_world_with_root(&mut world);
    let screen = builder.get_current_entity();
    builder.add_component_to_current_entity(Mapped {});

    resources.insert(create_relationship_map()); 
    schedule.execute(&mut world, &mut resources);
    
    assert_eq!(resources.get::<RelationshipMap>().unwrap().get(&screen), None);
}


#[test]
fn system_marks_as_mapped() {
    let mut world = World::default();
    let mut resources = Resources::default();
    let mut schedule = Schedule::builder()
        .add_system(mark_as_mapped_system())
        .build();

    let mut builder = world_entity_builder_for_world_with_root(&mut world);
    builder.create_rectangle_entity();
    builder.complete_entity();
    
    schedule.execute(&mut world, &mut resources);
    
    let mapped: Vec::<&Mapped> = <&Mapped>::query()
        .iter(&mut world)
        .collect();

    assert_eq!(mapped.len(), 2);
}