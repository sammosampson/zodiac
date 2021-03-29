use legion::*;
use zodiac_entities::*;

#[test]
fn builder_creates_canvas_layout_content_entity() {
    let mut world = World::default();
    let mut builder = world_entity_builder_for_world_with_root(&mut world);
    builder.create_canvas_layout_content_entity();

    let entities:Vec::<&LayoutContent> = <&LayoutContent>::query()
        .iter(&mut world)
        .collect();
    
    assert_eq!(entities[0].layout_type, LayoutType::Canvas);
    assert_eq!(entities[1].layout_type, LayoutType::Canvas);
    assert_eq!(entities.len(), 2);
}

#[test]
fn builder_creates_horizontal_layout_content_entity() {
    let mut world = World::default();
    let mut builder = world_entity_builder_for_world_with_root(&mut world);
    builder.create_horizontal_layout_content_entity();

    let entities:Vec::<&LayoutContent> = <&LayoutContent>::query()
        .iter(&mut world)
        .collect();
    
    assert_eq!(entities[0].layout_type, LayoutType::Canvas);
    assert_eq!(entities[1].layout_type, LayoutType::Horizontal);
    assert_eq!(entities.len(), 2);
}

#[test]
fn builder_creates_vertical_layout_content_entity() {
    let mut world = World::default();
    let mut builder = world_entity_builder_for_world_with_root(&mut world);
    builder.create_vertical_layout_content_entity();

    let entities:Vec::<&LayoutContent> = <&LayoutContent>::query()
        .iter(&mut world)
        .collect();
    
    assert_eq!(entities[0].layout_type, LayoutType::Canvas);
    assert_eq!(entities[1].layout_type, LayoutType::Vertical);
    assert_eq!(entities.len(), 2);
}

#[test]
fn builder_creates_rectangle_entity() {
    let mut world = World::default();
    let mut builder = world_entity_builder_for_world_with_root(&mut world);
    builder.create_rectangle_entity();

    let entity_count = <&Renderable>::query().iter(&mut world).count();
    assert_eq!(entity_count, 1);
}

#[test]
fn builder_creates_circle_entity() {
    let mut world = World::default();
    let mut builder = world_entity_builder_for_world_with_root(&mut world);
    builder.create_circle_entity();

    let entity_count = <&Renderable>::query().iter(&mut world).count();
    assert_eq!(entity_count, 1);
}

#[test]
fn builder_creates_glyph_entity() {
    let mut world = World::default();
    let mut builder = world_entity_builder_for_world_with_root(&mut world);
    builder.create_glyph_entity();

    let entity_count = <&Renderable>::query().iter(&mut world).count();
    assert_eq!(entity_count, 1);
}

#[test]
fn builder_closes_entities() {
    let mut world = World::default();
    let mut builder = world_entity_builder_for_world_with_root(&mut world);
    
    let screen = builder.get_current_entity();
    builder.create_rectangle_entity();
    
    assert_ne!(builder.get_current_entity(), screen);

    builder.complete_entity();

    assert_eq!(builder.get_current_entity(), screen);
}


#[test]
fn builder_creates_hierarchical_relationships_to_one_level() {
    let mut world = World::default();
    let mut builder = world_entity_builder_for_world_with_root(&mut world);
    let screen = builder.get_current_entity();
    
    builder.create_horizontal_layout_content_entity();
    let layout = builder.get_current_entity();
    builder.complete_entity();

    let relationships: Vec::<&Relationship> = <&Relationship>::query()
        .iter(&mut world)
        .collect();

    let mut index = 0;
    assert_eq!(relationships[index].parent, None);
    assert_eq!(relationships[index].next_sibling, None);
    assert_eq!(relationships[index].first_child, Some(layout));
    assert_eq!(relationships[index].last_child, Some(layout));
    
    index += 1;
    assert_eq!(relationships[index].parent, Some(screen));
    assert_eq!(relationships[index].next_sibling, None);
    assert_eq!(relationships[index].first_child, None);
    assert_eq!(relationships[index].last_child, None);
    
    assert_eq!(relationships.len(), 2);
}

#[test]
fn builder_creates_hierarchical_relationships_to_two_levels() {
    let mut world = World::default();
    let mut builder = world_entity_builder_for_world_with_root(&mut world);
    let screen = builder.get_current_entity();
    
    builder.create_horizontal_layout_content_entity();
    let content = builder.get_current_entity();
    builder.create_rectangle_entity();

    let rectangle = builder.get_current_entity();
    builder.complete_entity();
    builder.complete_entity();

    let relationships: Vec::<&Relationship> = <&Relationship>::query()
        .iter(&mut world)
        .collect();

    assert_eq!(relationships[0].parent, None);
    assert_eq!(relationships[0].next_sibling, None);
    assert_eq!(relationships[0].first_child, Some(content));
    assert_eq!(relationships[0].last_child, Some(content));

    assert_eq!(relationships[1].parent, Some(screen));
    assert_eq!(relationships[1].next_sibling, None);
    assert_eq!(relationships[1].first_child, Some(rectangle));
    assert_eq!(relationships[1].last_child, Some(rectangle));

    assert_eq!(relationships[2].parent, Some(content));
    assert_eq!(relationships[2].next_sibling, None);
    assert_eq!(relationships[2].first_child, None);
    assert_eq!(relationships[2].last_child, None);
    
    assert_eq!(relationships.len(), 3);
}

#[test]
fn builder_creates_entity_with_left() {
    let mut world = World::default();
    let mut builder = world_entity_builder_for_world_with_root(&mut world);
    let mut entity_count = 0;
    builder.create_rectangle_entity();
    builder.add_left_component(1);
    builder.complete_entity();

    for left in <&Left>::query()
        .filter(component::<Renderable>())
        .iter(&mut world) {
            assert_eq!(left.left, 1);
            entity_count += 1;
        }

    assert_eq!(entity_count, 1);
}
#[test]
fn builder_creates_entity_with_top() {
    let mut world = World::default();
    let mut builder = world_entity_builder_for_world_with_root(&mut world);
    let mut entity_count = 0;
    builder.create_rectangle_entity();
    builder.add_top_component(2);
    builder.complete_entity();

    for top in <&Top>::query()
        .filter(component::<Renderable>())
        .iter(&mut world) {
            assert_eq!(top.top, 2);
            entity_count += 1;
        }

    assert_eq!(entity_count, 1);
}

#[test]
fn builder_creates_entity_with_width() {
    let mut world = World::default();
    let mut builder = world_entity_builder_for_world_with_root(&mut world);
    let mut entity_count = 0;
    builder.create_rectangle_entity();
    builder.add_width_component(2);
    builder.complete_entity();

    for width in <&Width>::query()
        .filter(component::<Renderable>())
        .iter(&mut world) {
            assert_eq!(width.width, 2);
            entity_count += 1;
        }

    assert_eq!(entity_count, 1);
}

#[test]
fn builder_creates_entity_with_height() {
    let mut world = World::default();
    let mut builder = world_entity_builder_for_world_with_root(&mut world);
    let mut entity_count = 0;
    builder.create_rectangle_entity();
    builder.add_height_component(2);
    builder.complete_entity();

    for height in <&Height>::query()
        .filter(component::<Renderable>())
        .iter(&mut world) {
            assert_eq!(height.height, 2);
            entity_count += 1;
        }

    assert_eq!(entity_count, 1);
}

#[test]
fn builder_creates_entity_with_radius() {
    let mut world = World::default();
    let mut builder = world_entity_builder_for_world_with_root(&mut world);
    let mut entity_count = 0;
    builder.create_circle_entity();
    builder.add_radius_component(1);
    builder.complete_entity();

    for radius in <&Radius>::query()
        .filter(component::<Renderable>())
        .iter(&mut world) {
            assert_eq!(radius.radius, 1);
            entity_count += 1;
        }

    assert_eq!(entity_count, 1);
}

#[test]
fn builder_creates_entity_with_stroke_width() {
    let mut world = World::default();
    let mut builder = world_entity_builder_for_world_with_root(&mut world);
    let mut entity_count = 0;
    builder.create_circle_entity();
    builder.add_stroke_width_component(1);
    builder.complete_entity();

    for stroke_width in <&StrokeWidth>::query()
        .filter(component::<Renderable>())
        .iter(&mut world) {
            assert_eq!(stroke_width.width, 1);
            entity_count += 1;
        }

    assert_eq!(entity_count, 1);
}

#[test]
fn builder_creates_entity_with_colour() {
    let mut world = World::default();
    let mut builder = world_entity_builder_for_world_with_root(&mut world);
    let mut entity_count = 0;
    builder.create_rectangle_entity();
    builder.add_colour_component(1.0, 0.9, 0.8, 0.7);
    builder.complete_entity();

    for colour in <&Colour>::query()
        .filter(component::<Renderable>())
        .iter(&mut world) {
            assert_eq!(colour.r, 1.0);
            assert_eq!(colour.g, 0.9);
            assert_eq!(colour.b, 0.8);
            assert_eq!(colour.a, 0.7);
            entity_count += 1;
        }

    assert_eq!(entity_count, 1);
}

#[test]
fn builder_creates_entity_with_stroke_colour() {
    let mut world = World::default();
    let mut builder = world_entity_builder_for_world_with_root(&mut world);
    let mut entity_count = 0;
    builder.create_circle_entity();
    builder.add_stroke_colour_component(1.0, 0.9, 0.8, 0.7);
    builder.complete_entity();

    for colour in <&StrokeColour>::query()
        .filter(component::<Renderable>())
        .iter(&mut world) {
            assert_eq!(colour.r, 1.0);
            assert_eq!(colour.g, 0.9);
            assert_eq!(colour.b, 0.8);
            assert_eq!(colour.a, 0.7);
            entity_count += 1;
        }

    assert_eq!(entity_count, 1);
}

#[test]
fn builder_creates_entity_with_corner_radii() {
    let mut world = World::default();
    let mut builder = world_entity_builder_for_world_with_root(&mut world);
    let mut entity_count = 0;
    builder.create_rectangle_entity();
    builder.add_corner_radii_component(100, 90, 80, 70);
    builder.complete_entity();

    for colour in <&CornerRadii>::query()
        .filter(component::<Renderable>())
        .iter(&mut world) {
            assert_eq!(colour.left_top, 100);
            assert_eq!(colour.right_top, 90);
            assert_eq!(colour.right_bottom, 80);
            assert_eq!(colour.left_bottom, 70);
            entity_count += 1;
        }

    assert_eq!(entity_count, 1);
}

