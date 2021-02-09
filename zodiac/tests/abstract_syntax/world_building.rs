use legion::*;
use zodiac_parsing::tokenization::abstract_syntax::*;
use zodiac_parsing::tokenization::source::*;
use zodiac::abstract_syntax::world_building::*;
use zodiac_entities::components::*;

#[test]
fn parse_circle_produces_circle_components_on_entity() {
    let mut world = World::default();
    build_world(&mut world, "<circle
        position=(200, 100)
        radius=50
        colour=(1.0, 2.0, 3.0, 4.0)
        stroke-colour=(0.0, 1.0, 2.0, 3.0)
        stroke-width=3
    />").unwrap();
    let mut entities = 0;
    let mut query = <(&Position, &Radius, &Colour, &StrokeColour, &StrokeWidth)>::query()
        .filter(component::<Dirty>() & component::<Circle>());

    for (position, radius, colour, stroke_colour, stroke_width) in query.iter(&mut world) {
        entities += 1;
        assert_eq!(position.x, 200);
        assert_eq!(position.y, 100);
        assert_eq!(radius.radius, 50);
        assert_eq!(colour.r, 1.0);
        assert_eq!(colour.g, 2.0);
        assert_eq!(colour.b, 3.0);
        assert_eq!(colour.a, 4.0);
        assert_eq!(stroke_colour.r, 0.0);
        assert_eq!(stroke_colour.g, 1.0);
        assert_eq!(stroke_colour.b, 2.0);
        assert_eq!(stroke_colour.a, 3.0);
        assert_eq!(stroke_width.width, 3);
    }
    assert_eq!(entities, 1);
}

#[test]
fn parse_rect_produces_rectangle_components_on_entity() {
    let mut world = World::default();
    build_world(&mut world, "<rect
        position=(200, 100)
        dimensions=(200, 300)
        colour=(1.0, 2.0, 3.0, 4.0)
        stroke-colour=(0.0, 1.0, 2.0, 3.0)
        stroke-width=3
        corner-radii=(0.5, 1.5, 2.5, 3.5)
    />").unwrap();
    let mut entities = 0;
    let mut query = <(&Position, &Dimensions, &Colour, &StrokeColour, &StrokeWidth, &CornerRadii)>::query()
        .filter(component::<Dirty>() & component::<Rectangle>());

    for (position, dimensions, colour, stroke_colour, stroke_width, corner_radii) in query.iter(&mut world) {
        entities += 1;
        assert_eq!(position.x, 200);
        assert_eq!(position.y, 100);
        assert_eq!(dimensions.x, 200);
        assert_eq!(dimensions.y, 300);
        assert_eq!(colour.r, 1.0);
        assert_eq!(colour.g, 2.0);
        assert_eq!(colour.b, 3.0);
        assert_eq!(colour.a, 4.0);
        assert_eq!(stroke_colour.r, 0.0);
        assert_eq!(stroke_colour.g, 1.0);
        assert_eq!(stroke_colour.b, 2.0);
        assert_eq!(stroke_colour.a, 3.0);
        assert_eq!(stroke_width.width, 3);
        assert_eq!(corner_radii.left_top, 0.5);
        assert_eq!(corner_radii.right_top, 1.5);
        assert_eq!(corner_radii.right_bottom, 2.5);
        assert_eq!(corner_radii.left_bottom, 3.5);
    }
    assert_eq!(entities, 1);
}

#[test]
fn parse_text_produces_text_components_on_entity() {
    let mut world = World::default();
    build_world(&mut world, "<text
        position=(200, 100)
        dimensions=(200, 300)
        glyph-index=32
        colour=(1.0, 2.0, 3.0, 4.0)
    />").unwrap();
    let mut entities = 0;
    let mut query = <(&Position, &Dimensions, &GlyphIndex, &Colour)>::query()
        .filter(component::<Dirty>() & component::<Text>());

    for (position, dimensions, glyph_index, colour) in query.iter(&mut world) {
        entities += 1;
        assert_eq!(position.x, 200);
        assert_eq!(position.y, 100);
        assert_eq!(dimensions.x, 200);
        assert_eq!(dimensions.y, 300);
        assert_eq!(glyph_index.index, 32);
        assert_eq!(colour.r, 1.0);
        assert_eq!(colour.g, 2.0);
        assert_eq!(colour.b, 3.0);
        assert_eq!(colour.a, 4.0);
    }
    assert_eq!(entities, 1);
}

#[test]
fn parse_multiple_controls_produces_entities() {
    let mut world = World::default();
    build_world(&mut world, "<circle position=(200, 100) /><rect position=(200, 100) /><text position=(200, 100) />").unwrap();
    let mut entities = 0;
    let mut query = <&Position>::query();
    for _position in query.iter(&mut world) {
        entities += 1;
    }
    assert_eq!(entities, 3);
}

#[test]
fn parse_malformed_property_produces_error() {
    let mut world = World::default();
    let result = build_world(&mut world, "<circle position=(200, 100) dimensions=(200, 30x) /><rect position=(200, 100) dimensions=(200, 300) /><text position=(200, 100) />");
    assert_eq!(result, Err(AbstractSyntaxTokenError::BadDimensionsValue));
    let mut entities = 0;
    let mut query = <&Position>::query();
    for _position in query.iter(&mut world) {
        entities += 1;
    }
    assert_eq!(entities, 1);
}

fn build_world(world: &mut World, source: &str) -> Result<(), AbstractSyntaxTokenError> {
    let mut tokenizer = AbstractSyntaxTokenizer::from_source(SourceTokenizer::from_string(source));
    tokenizer.build_world(world)
}

