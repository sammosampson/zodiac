use legion::*;
use zodiac::world_building::entities::*;
use zodiac::systems::layout::*;
use zodiac_entities::components::*;
use zodiac::systems::relationships::*;

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
    builder.add_component_to_current_entity(Mapped {});

    resources.insert(create_top_offset_map());  
    schedule.execute(&mut world, &mut resources);
    
    assert_eq!(resources.get::<TopOffsetMap>().unwrap().get(&screen), None);
}

#[test]
fn layout_system_performs_absolute_positioning_on_screen() {
    let mut world = World::default();
    let mut resources = Resources::default();
    let mut schedule = Schedule::builder()
        .add_system(build_relationship_map_system())
        .add_system(build_left_offset_map_system())
        .add_system(build_top_offset_map_system())
        .flush()
        .add_thread_local(layout_system())
        .build();

    let mut builder = WorldEntityBuilder::for_world(&mut world);
    builder.create_rectangle_entity();
    builder.add_left_component(10);
    builder.add_top_component(11);
    builder.complete_entity();

    builder.create_circle_entity();
    builder.add_left_component(11);
    builder.add_top_component(12);
    builder.complete_entity();

    builder.create_text_entity();
    builder.add_left_component(12);
    builder.add_top_component(13);
    builder.complete_entity();

    resources.insert(create_relationship_map()); 
    resources.insert(create_left_offset_map()); 
    resources.insert(create_top_offset_map()); 
    schedule.execute(&mut world, &mut resources);
    
    let positions: Vec::<&Position> = <&Position>::query()
        .iter(&mut world)
        .collect();

    assert_eq!(positions[0].x, 10);
    assert_eq!(positions[0].y, 11);
    assert_eq!(positions[1].x, 11);
    assert_eq!(positions[1].y, 12);
    assert_eq!(positions[2].x, 12);
    assert_eq!(positions[2].y, 13);
    assert_eq!(positions.len(), 3);
}

#[test]
fn layouts_system_performs_absolute_positioning_on_canvas_offset_from_screen() {
    let mut world = World::default();
    let mut resources = Resources::default();
    let mut schedule = Schedule::builder()
        .add_system(build_relationship_map_system())
        .add_system(build_left_offset_map_system())
        .add_system(build_top_offset_map_system())
        .flush()
        .add_thread_local(layout_system())
        .build();

    let mut builder = WorldEntityBuilder::for_world(&mut world);
    builder.create_horizontal_layout_content_entity();
    builder.add_left_component(10);
    builder.add_top_component(11);

    builder.create_canvas_layout_content_entity();
    
    builder.create_rectangle_entity();
    builder.add_left_component(10);
    builder.add_top_component(11);
    builder.complete_entity();

    builder.create_rectangle_entity();
    builder.add_left_component(11);
    builder.add_top_component(12);
    builder.complete_entity();

    builder.complete_entity();
    
    builder.complete_entity();

    resources.insert(create_relationship_map()); 
    resources.insert(create_left_offset_map()); 
    resources.insert(create_top_offset_map());
    schedule.execute(&mut world, &mut resources);
    
    let positions: Vec::<&Position> = <&Position>::query()
        .filter(component::<Rectangle>())
        .iter(&mut world)
        .collect();

    assert_eq!(positions[0].x, 20);
    assert_eq!(positions[0].y, 22);
    assert_eq!(positions[1].x, 21);
    assert_eq!(positions[1].y, 23);
    assert_eq!(positions.len(), 2);
}