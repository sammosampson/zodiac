use legion::*;
use zodiac::world_building::entities::*;
use zodiac_entities::components::*;

#[test]
fn builder_creates_screen_entity() {
    let mut world = World::default();
    WorldEntityBuilder::for_world(&mut world);
    
    let relationships: Vec::<&Relationship> = <&Relationship>::query()
        .filter(component::<CanvasLayoutContent>())
        .iter(&mut world)
        .collect();

    assert_eq!(relationships[0].parent, None);
    assert_eq!(relationships[0].next_sibling, None);
    assert_eq!(relationships[0].first_child, None);
    assert_eq!(relationships[0].last_child, None);
    assert_eq!(relationships.len(), 1);
}

#[test]
fn builder_creates_canvas_layout_content_entity() {
    let mut world = World::default();
    let mut builder = WorldEntityBuilder::for_world(&mut world);
    builder.create_canvas_layout_content_entity();

    let entity_count = <&CanvasLayoutContent>::query().iter(&mut world).count();
    assert_eq!(entity_count, 2);
}

#[test]
fn builder_creates_horizontal_layout_content_entity() {
    let mut world = World::default();
    let mut builder = WorldEntityBuilder::for_world(&mut world);
    builder.create_horizontal_layout_content_entity();

    let entity_count = <&HorizontalLayoutContent>::query().iter(&mut world).count();
    assert_eq!(entity_count, 1);
}

#[test]
fn builder_creates_rectangle_entity() {
    let mut world = World::default();
    let mut builder = WorldEntityBuilder::for_world(&mut world);
    builder.create_rectangle_entity();

    let entity_count = <&Rectangle>::query().iter(&mut world).count();
    assert_eq!(entity_count, 1);
}

#[test]
fn builder_creates_circle_entity() {
    let mut world = World::default();
    let mut builder = WorldEntityBuilder::for_world(&mut world);
    builder.create_circle_entity();

    let entity_count = <&Circle>::query().iter(&mut world).count();
    assert_eq!(entity_count, 1);
}

#[test]
fn builder_creates_text_entity() {
    let mut world = World::default();
    let mut builder = WorldEntityBuilder::for_world(&mut world);
    builder.create_text_entity();

    let entity_count = <&Text>::query().iter(&mut world).count();
    assert_eq!(entity_count, 1);
}

#[test]
fn builder_closes_entities() {
    let mut world = World::default();
    let mut builder = WorldEntityBuilder::for_world(&mut world);
    
    let screen = builder.get_current_entity();
    builder.create_rectangle_entity();
    
    assert_ne!(builder.get_current_entity(), screen);

    builder.complete_entity();

    assert_eq!(builder.get_current_entity(), screen);
}

#[test]
fn builder_creates_hierarchical_relationships_to_one_level() {
    let mut world = World::default();
    let mut builder = WorldEntityBuilder::for_world(&mut world);
    let screen = builder.get_current_entity();
    
    builder.create_rectangle_entity();
    let rectangle = builder.get_current_entity();
    builder.complete_entity();
    
    builder.create_circle_entity();
    let circle = builder.get_current_entity();
    builder.complete_entity();

    builder.create_text_entity();
    let text = builder.get_current_entity();
    builder.complete_entity();

    let relationships: Vec::<&Relationship> = <&Relationship>::query()
        .iter(&mut world)
        .collect();

    assert_eq!(relationships[0].parent, None);
    assert_eq!(relationships[0].next_sibling, None);
    assert_eq!(relationships[0].first_child, Some(rectangle));
    assert_eq!(relationships[0].last_child, Some(text));

    assert_eq!(relationships[1].parent, Some(screen));
    assert_eq!(relationships[1].next_sibling, Some(circle));
    assert_eq!(relationships[1].first_child, None);
    assert_eq!(relationships[1].last_child, None);

    assert_eq!(relationships[2].parent, Some(screen));
    assert_eq!(relationships[2].next_sibling, Some(text));
    assert_eq!(relationships[2].first_child, None);
    assert_eq!(relationships[2].last_child, None);

    assert_eq!(relationships[3].parent, Some(screen));
    assert_eq!(relationships[3].next_sibling, None);
    assert_eq!(relationships[3].first_child, None);
    assert_eq!(relationships[3].last_child, None);
    
    assert_eq!(relationships.len(), 4);
}


#[test]
fn builder_creates_hierarchical_relationships_to_multiple_levels() {
    let mut world = World::default();
    let mut builder = WorldEntityBuilder::for_world(&mut world);
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
fn builder_creates_entity_with_offset() {
    let mut world = World::default();
    let mut builder = WorldEntityBuilder::for_world(&mut world);
    let mut entity_count = 0;
    builder.create_rectangle_entity();
    builder.add_offset_component(1, 2);
    builder.complete_entity();

    for position in <&Offset>::query()
        .filter(component::<Rectangle>())
        .iter(&mut world) {
            assert_eq!(position.x, 1);
            assert_eq!(position.y, 2);
            entity_count += 1;
        }

    assert_eq!(entity_count, 1);
}

#[test]
fn builder_creates_entity_with_dimensions() {
    let mut world = World::default();
    let mut builder = WorldEntityBuilder::for_world(&mut world);
    let mut entity_count = 0;
    builder.create_rectangle_entity();
    builder.add_dimensions_component(1, 2);
    builder.complete_entity();

    for dimensions in <&Dimensions>::query()
        .filter(component::<Rectangle>())
        .iter(&mut world) {
            assert_eq!(dimensions.x, 1);
            assert_eq!(dimensions.y, 2);
            entity_count += 1;
        }

    assert_eq!(entity_count, 1);
}

#[test]
fn builder_creates_entity_with_radius() {
    let mut world = World::default();
    let mut builder = WorldEntityBuilder::for_world(&mut world);
    let mut entity_count = 0;
    builder.create_circle_entity();
    builder.add_radius_component(1);
    builder.complete_entity();

    for radius in <&Radius>::query()
        .filter(component::<Circle>())
        .iter(&mut world) {
            assert_eq!(radius.radius, 1);
            entity_count += 1;
        }

    assert_eq!(entity_count, 1);
}

#[test]
fn builder_creates_entity_with_stroke_width() {
    let mut world = World::default();
    let mut builder = WorldEntityBuilder::for_world(&mut world);
    let mut entity_count = 0;
    builder.create_circle_entity();
    builder.add_stroke_width_component(1);
    builder.complete_entity();

    for stroke_width in <&StrokeWidth>::query()
        .filter(component::<Circle>())
        .iter(&mut world) {
            assert_eq!(stroke_width.width, 1);
            entity_count += 1;
        }

    assert_eq!(entity_count, 1);
}

#[test]
fn builder_creates_entity_with_glyph_index() {
    let mut world = World::default();
    let mut builder = WorldEntityBuilder::for_world(&mut world);
    let mut entity_count = 0;
    builder.create_text_entity();
    builder.add_glyph_index_component(1);
    builder.complete_entity();

    for glyph_index in <&GlyphIndex>::query()
        .filter(component::<Text>())
        .iter(&mut world) {
            assert_eq!(glyph_index.index, 1);
            entity_count += 1;
        }

    assert_eq!(entity_count, 1);
}

#[test]
fn builder_creates_entity_with_colour() {
    let mut world = World::default();
    let mut builder = WorldEntityBuilder::for_world(&mut world);
    let mut entity_count = 0;
    builder.create_text_entity();
    builder.add_colour_component(1.0, 0.9, 0.8, 0.7);
    builder.complete_entity();

    for colour in <&Colour>::query()
        .filter(component::<Text>())
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
    let mut builder = WorldEntityBuilder::for_world(&mut world);
    let mut entity_count = 0;
    builder.create_circle_entity();
    builder.add_stroke_colour_component(1.0, 0.9, 0.8, 0.7);
    builder.complete_entity();

    for colour in <&StrokeColour>::query()
        .filter(component::<Circle>())
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
    let mut builder = WorldEntityBuilder::for_world(&mut world);
    let mut entity_count = 0;
    builder.create_rectangle_entity();
    builder.add_corner_radii_component(1.0, 0.9, 0.8, 0.7);
    builder.complete_entity();

    for colour in <&CornerRadii>::query()
        .filter(component::<Rectangle>())
        .iter(&mut world) {
            assert_eq!(colour.left_top, 1.0);
            assert_eq!(colour.right_top, 0.9);
            assert_eq!(colour.right_bottom, 0.8);
            assert_eq!(colour.left_bottom, 0.7);
            entity_count += 1;
        }

    assert_eq!(entity_count, 1);
}

