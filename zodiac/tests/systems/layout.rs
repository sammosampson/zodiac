use legion::*;
use zodiac::world_building::entities::*;
use zodiac::systems::layout::*;
use zodiac_entities::components::*;

#[test]
fn system_performs_absolute_positioning_on_screen() {
    let mut world = World::default();
    let mut resources = Resources::default();
    let mut schedule = Schedule::builder()
        .add_thread_local(position_children_of_canvases_system())
        .build();

    let mut builder = WorldEntityBuilder::for_world(&mut world);
    builder.create_rectangle_entity();
    builder.add_offset_component(10, 11);
    builder.complete_entity();

    builder.create_circle_entity();
    builder.add_offset_component(11, 12);
    builder.complete_entity();

    builder.create_text_entity();
    builder.add_offset_component(12, 13);
    builder.complete_entity();

    resources.insert(AbsoluteOffsetMap::new()); 
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
fn absolute_positioning_on_canvas_offset_from_screen() {
    let mut world = World::default();
    let mut resources = Resources::default();
    let mut schedule = Schedule::builder()
        .add_thread_local(position_children_of_canvases_system())
        .build();

    let mut builder = WorldEntityBuilder::for_world(&mut world);
    builder.create_canvas_layout_content_entity();
    builder.add_offset_component(10, 11);

    builder.create_horizontal_layout_content_entity();
    
    builder.create_rectangle_entity();
    builder.add_offset_component(10, 11);
    builder.complete_entity();

    builder.create_rectangle_entity();
    builder.add_offset_component(11, 12);
    builder.complete_entity();

    builder.complete_entity();
    
    builder.complete_entity();

    resources.insert(AbsoluteOffsetMap::new()); 
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