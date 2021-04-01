use legion::*;
use zodiac_entities::*;
use zodiac_layout::*;

#[test]
fn screen_resize_system_resizes_root() {
    let mut world = World::default();
    let mut resources = Resources::default();
    let mut schedule = Schedule::builder()
        .add_system(resize_screen_system())
        .build();

    let mut builder = world_entity_builder_for_world_with_root(&mut world);
    builder.create_root_entity();
    
    builder.create_entity_with_component(RootWindowResized { width: 10, height: 20 });

    schedule.execute(&mut world, &mut resources);

    let root_resizes: Vec::<&LayoutRequest> = <&LayoutRequest>::query()
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

    let mut builder = world_entity_builder_for_world_with_root(&mut world);
    builder.create_root_entity();
    
    builder.add_component_to_current_entity(LayoutRequest { left: 0, top: 0, width: 100, height: 100});
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

    builder.create_rectangle_entity();
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
fn resize_system_performs_absolute_positioning_on_canvas_offset_from_screen() {
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

    let mut builder = world_entity_builder_for_world_with_root(&mut world);
    builder.add_component_to_current_entity(LayoutRequest { left: 10, top: 11, width: 100, height: 110});

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

    let mut builder = world_entity_builder_for_world_with_root(&mut world);
    builder.add_component_to_current_entity(LayoutRequest { left: 10, top: 11, width: 100, height: 110});
    
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

#[test]
fn resize_system_performs_horizontal_layout_for_none_sized_children() {
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
        .add_thread_local(measure_fixed_width_constraints_system())
        .flush()
        .add_thread_local(resize_system())
        .build();

    let mut builder = world_entity_builder_for_world_with_root(&mut world);
    builder.add_component_to_current_entity(LayoutRequest { left: 0, top: 0, width: 100, height: 100});

    builder.create_horizontal_layout_content_entity();
    
    builder.create_rectangle_entity();
    builder.complete_entity();

    builder.create_rectangle_entity();
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

    assert_eq!(changes[0].left, 0);
    assert_eq!(changes[0].top, 0);
    assert_eq!(changes[0].width, 50);
    assert_eq!(changes[0].height, 100);

    assert_eq!(changes[1].left, 50);
    assert_eq!(changes[1].top, 0);
    assert_eq!(changes[1].width, 50);
    assert_eq!(changes[1].height, 100);
    assert_eq!(changes.len(), 2);
}

#[test]
fn resize_system_performs_horizontal_layout_for_sized_children() {
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
        .add_thread_local(measure_fixed_width_constraints_system())
        .flush()
        .add_thread_local(resize_system())
        .build();

    let mut builder = world_entity_builder_for_world_with_root(&mut world);
    
    builder.create_root_entity();
    
    builder.add_component_to_current_entity(LayoutRequest { left: 0, top: 0, width: 100, height: 100});

    builder.create_horizontal_layout_content_entity();
    
    builder.create_rectangle_entity();
    builder.add_width_component(25);
    builder.complete_entity();

    builder.create_rectangle_entity();
    builder.complete_entity();

    builder.create_rectangle_entity();
    builder.add_width_component(35);
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

    assert_eq!(changes[0].left, 0);
    assert_eq!(changes[0].top, 0);
    assert_eq!(changes[0].width, 25);
    assert_eq!(changes[0].height, 100);

    assert_eq!(changes[1].left, 65);
    assert_eq!(changes[1].top, 0);
    assert_eq!(changes[1].width, 35);
    assert_eq!(changes[1].height, 100);

    assert_eq!(changes[2].left, 25);
    assert_eq!(changes[2].top, 0);
    assert_eq!(changes[2].width, 40);
    assert_eq!(changes[2].height, 100);

    assert_eq!(changes.len(), 3);
}

#[test]
fn resize_system_performs_vertical_layout_for_none_sized_children() {
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
        .add_thread_local(measure_fixed_height_constraints_system())
        .flush()
        .add_thread_local(resize_system())
        .build();

    let mut builder = world_entity_builder_for_world_with_root(&mut world);
    builder.add_component_to_current_entity(LayoutRequest { left: 0, top: 0, width: 100, height: 100});

    builder.create_vertical_layout_content_entity();
    
    builder.create_rectangle_entity();
    builder.complete_entity();

    builder.create_rectangle_entity();
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

    assert_eq!(changes[0].top, 0);
    assert_eq!(changes[0].left, 0);
    assert_eq!(changes[0].height, 50);
    assert_eq!(changes[0].width, 100);

    assert_eq!(changes[1].top, 50);
    assert_eq!(changes[1].left, 0);
    assert_eq!(changes[1].height, 50);
    assert_eq!(changes[1].width, 100);
    assert_eq!(changes.len(), 2);
}

#[test]
fn resize_system_performs_vertical_layout_for_sized_children() {
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
        .add_thread_local(measure_fixed_height_constraints_system())
        .flush()
        .add_thread_local(resize_system())
        .build();

    let mut builder = world_entity_builder_for_world_with_root(&mut world);
    builder.create_root_entity();
    

    builder.add_component_to_current_entity(LayoutRequest { left: 0, top: 0, width: 100, height: 100});

    builder.create_vertical_layout_content_entity();
    
    builder.create_rectangle_entity();
    builder.add_height_component(25);
    builder.complete_entity();

    builder.create_rectangle_entity();
    builder.complete_entity();

    builder.create_rectangle_entity();
    builder.add_height_component(35);
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

    assert_eq!(changes[0].top, 0);
    assert_eq!(changes[0].left, 0);
    assert_eq!(changes[0].height, 25);
    assert_eq!(changes[0].width, 100);

    assert_eq!(changes[1].top, 65);
    assert_eq!(changes[1].left, 0);
    assert_eq!(changes[1].height, 35);
    assert_eq!(changes[1].width, 100);

    assert_eq!(changes[2].top, 25);
    assert_eq!(changes[2].left, 0);
    assert_eq!(changes[2].height, 40);
    assert_eq!(changes[2].width, 100);

    assert_eq!(changes.len(), 3);
}