use legion::*;
use zodiac::world_building::entities::*;
use zodiac_entities::components::*;

#[test]
fn builder_creates_screen_entity() {
    let mut world = World::default();
    WorldEntityBuilder::for_world(&mut world);

    let entity_count = <&CanvasLayoutContent>::query().iter(&mut world).count();
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
fn builder_creates_entity_with_position() {
    let mut world = World::default();
    let mut builder = WorldEntityBuilder::for_world(&mut world);
    let mut entity_count = 0;
    builder.create_rectangle_entity();
    builder.add_position_component(1, 2);

    for position in <&Position>::query()
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

