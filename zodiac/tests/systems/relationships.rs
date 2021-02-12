use legion::*;
use zodiac::world_building::entities::*;
use zodiac::systems::relationships::*;

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
    builder.add_offset_component(10, 11);
    let rectangle = builder.get_current_entity();
    builder.complete_entity();

    builder.create_circle_entity();
    builder.add_offset_component(11, 12);
    let circle = builder.get_current_entity();
    builder.complete_entity();

    resources.insert(RelationshipMap::new()); 
    schedule.execute(&mut world, &mut resources);

    let relationship_map = resources.get::<RelationshipMap>().unwrap();
    let screen_relations = relationship_map.get_relationship_for_entity(&screen).unwrap();
    let rectangle_relations = relationship_map.get_relationship_for_entity(&rectangle).unwrap();
    let circle_relations = relationship_map.get_relationship_for_entity(&circle).unwrap();

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