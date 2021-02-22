use legion::*;
use zodiac::world_building::entities::*;
use zodiac::systems::layout::*;
use zodiac_entities::components::*;
use zodiac::systems::relationships::*;
use zodiac::systems::measurement::*;

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
fn screen_resize_system_resizes_root() {
    let mut world = World::default();
    let mut resources = Resources::default();
    let mut schedule = Schedule::builder()
        .add_system(resize_screen_system())
        .build();

    let mut builder = WorldEntityBuilder::for_world(&mut world);
    builder.create_entity_with_component(RootWindowResized { width: 10, height: 20 });

    schedule.execute(&mut world, &mut resources);

    let root_resizes: Vec::<&ResizeRequest> = <&ResizeRequest>::query()
        .filter(component::<Root>())
        .iter(&mut world)
        .collect();

    assert_eq!(root_resizes[0].left, 0);
    assert_eq!(root_resizes[0].top, 0);
    assert_eq!(root_resizes[0].width, 10);
    assert_eq!(root_resizes[0].height, 20);
    assert_eq!(root_resizes.len(), 1);
}

#[test]
fn resize_system_performs_absolute_positioning_on_screen() {
    let mut world = World::default();
    let mut resources = Resources::default();
    let mut schedule = Schedule::builder()
        .add_system(build_relationship_map_system())
        .add_system(build_left_offset_map_system())
        .add_system(build_top_offset_map_system())
        .add_system(build_width_map_system())
        .add_system(build_height_map_system())
        .add_system(build_layout_type_map_system())
        .flush()
        .add_thread_local(resize_system())
        .build();

    let mut builder = WorldEntityBuilder::for_world(&mut world);
    builder.add_component_to_current_entity(ResizeRequest { left: 0, top: 0, width: 100, height: 100});
    builder.create_rectangle_entity();
    builder.add_left_component(10);
    builder.add_top_component(11);
    builder.add_width_component(12);
    builder.add_height_component(13);
    builder.complete_entity();

    builder.create_circle_entity();
    builder.add_left_component(11);
    builder.add_top_component(12);
    builder.add_width_component(13);
    builder.add_height_component(14);
    builder.complete_entity();

    builder.create_text_entity();
    builder.add_left_component(12);
    builder.add_top_component(13);
    builder.add_width_component(14);
    builder.add_height_component(15);
    builder.complete_entity();

    resources.insert(create_relationship_map());
    resources.insert(create_layout_type_map());
    resources.insert(create_left_offset_map());
    resources.insert(create_top_offset_map());
    resources.insert(create_width_map());
    resources.insert(create_height_map());
    resources.insert(create_minimum_width_map());
    resources.insert(create_minimum_height_map());

    schedule.execute(&mut world, &mut resources);
    
    let changes: Vec::<&LayoutChange> = <&LayoutChange>::query()
        .iter(&mut world)
        .collect();

    assert_eq!(changes[0].left, 10);
    assert_eq!(changes[0].top, 11);
    assert_eq!(changes[0].width, 12);
    assert_eq!(changes[0].height, 13);
    assert_eq!(changes[1].left, 11);
    assert_eq!(changes[1].top, 12);
    assert_eq!(changes[1].width, 13);
    assert_eq!(changes[1].height, 14);
    assert_eq!(changes[2].left, 12);
    assert_eq!(changes[2].top, 13);
    assert_eq!(changes[2].width, 14);
    assert_eq!(changes[2].height, 15);
    assert_eq!(changes.len(), 3);
}

#[test]
fn layouts_system_performs_absolute_positioning_on_canvas_offset_from_screen() {
    let mut world = World::default();
    let mut resources = Resources::default();
    let mut schedule = Schedule::builder()
        .add_system(build_relationship_map_system())
        .add_system(build_left_offset_map_system())
        .add_system(build_top_offset_map_system())
        .add_system(build_width_map_system())
        .add_system(build_height_map_system())
        .add_system(build_layout_type_map_system())
        .flush()
        .add_thread_local(resize_system())
        .build();

    let mut builder = WorldEntityBuilder::for_world(&mut world);
    builder.add_component_to_current_entity(ResizeRequest { left: 10, top: 11, width: 100, height: 110});

    builder.create_canvas_layout_content_entity();
    builder.add_left_component(10);
    builder.add_top_component(11);
    
    builder.create_rectangle_entity();
    builder.add_left_component(10);
    builder.add_top_component(11);
    builder.complete_entity();

    builder.complete_entity();

    resources.insert(create_relationship_map());
    resources.insert(create_layout_type_map());
    resources.insert(create_left_offset_map());
    resources.insert(create_top_offset_map());
    resources.insert(create_width_map());
    resources.insert(create_height_map());
    resources.insert(create_minimum_width_map());
    resources.insert(create_minimum_height_map());

    schedule.execute(&mut world, &mut resources);
    
    let changes: Vec::<&LayoutChange> = <&LayoutChange>::query()
        .iter(&mut world)
        .collect();

    assert_eq!(changes[0].left, 30);
    assert_eq!(changes[0].top, 33);
    assert_eq!(changes[0].width, 100);
    assert_eq!(changes[0].height, 110);
    assert_eq!(changes.len(), 1);
}

#[test]
fn resize_system_makes_dimensions_fit_parent_when_not_specified() {
    let mut world = World::default();
    let mut resources = Resources::default();
    let mut schedule = Schedule::builder()
        .add_system(build_relationship_map_system())
        .add_system(build_left_offset_map_system())
        .add_system(build_top_offset_map_system())
        .add_system(build_width_map_system())
        .add_system(build_height_map_system())
        .add_system(build_layout_type_map_system())
        .flush()
        .add_thread_local(resize_system())
        .build();

    let mut builder = WorldEntityBuilder::for_world(&mut world);
    builder.add_component_to_current_entity(ResizeRequest { left: 10, top: 11, width: 100, height: 110});
    
    builder.create_rectangle_entity();
    builder.complete_entity();

    resources.insert(create_relationship_map());
    resources.insert(create_layout_type_map());
    resources.insert(create_left_offset_map());
    resources.insert(create_top_offset_map());
    resources.insert(create_width_map());
    resources.insert(create_height_map());
    resources.insert(create_minimum_width_map());
    resources.insert(create_minimum_height_map());

    schedule.execute(&mut world, &mut resources);
    
    let changes: Vec::<&LayoutChange> = <&LayoutChange>::query()
        .iter(&mut world)
        .collect();

    assert_eq!(changes[0].left, 10);
    assert_eq!(changes[0].top, 11);
    assert_eq!(changes[0].width, 100);
    assert_eq!(changes[0].height, 110);
    assert_eq!(changes.len(), 1);
}