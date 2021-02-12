use legion::*;
use zodiac_parsing::tokenization::abstract_syntax::*;
use zodiac_parsing::tokenization::source::*;
use zodiac::world_building::abstract_syntax::*;
use zodiac_entities::components::*;

#[test]
fn parse_horizontal_layoutcontainer_produces_container_components_on_entity() {
    let mut world = World::default();
    build_world(&mut world, "<horizontal-layout-content />").unwrap();
    
    let entity_count = <&HorizontalLayoutContent>::query()
        .iter(&mut world)
        .count();
    
    assert_eq!(entity_count, 1);
}

#[test]
fn parse_canvas_layout_container_produces_container_components_on_entity() {
    let mut world = World::default();
    build_world(&mut world, "<canvas-layout-content />").unwrap();
    
    let entity_count = <&CanvasLayoutContent>::query()
        .iter(&mut world)
        .count();
    
    assert_eq!(entity_count, 2);
}

#[test]
fn parse_circle_produces_circle_components_on_entity() {
    let mut world = World::default();
    build_world(&mut world, "<circle
        offset=(200, 100)
        radius=50
        colour=(1.0, 2.0, 3.0, 4.0)
        stroke-colour=(0.0, 1.0, 2.0, 3.0)
        stroke-width=3
    />").unwrap();
    let mut entities = 0;
    let mut query = <(&Offset, &Radius, &Colour, &StrokeColour, &StrokeWidth)>::query()
        .filter(component::<Circle>());

    for (offset, radius, colour, stroke_colour, stroke_width) in query.iter(&mut world) {
        entities += 1;
        assert_eq!(offset.x, 200);
        assert_eq!(offset.y, 100);
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
        offset=(200, 100)
        dimensions=(200, 300)
        colour=(1.0, 2.0, 3.0, 4.0)
        stroke-colour=(0.0, 1.0, 2.0, 3.0)
        stroke-width=3
        corner-radii=(0.5, 1.5, 2.5, 3.5)
    />").unwrap();
    let mut entities = 0;
    let mut query = <(&Offset, &Dimensions, &Colour, &StrokeColour, &StrokeWidth, &CornerRadii)>::query()
        .filter(component::<Rectangle>());

    for (offset, dimensions, colour, stroke_colour, stroke_width, corner_radii) in query.iter(&mut world) {
        entities += 1;
        assert_eq!(offset.x, 200);
        assert_eq!(offset.y, 100);
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
        offset=(200, 100)
        dimensions=(200, 300)
        glyph-index=32
        colour=(1.0, 2.0, 3.0, 4.0)
    />").unwrap();
    let mut entities = 0;
    let mut query = <(&Offset, &Dimensions, &GlyphIndex, &Colour)>::query()
        .filter(component::<Text>());

    for (offset, dimensions, glyph_index, colour) in query.iter(&mut world) {
        entities += 1;
        assert_eq!(offset.x, 200);
        assert_eq!(offset.y, 100);
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
    build_world(
        &mut world, 
        "<circle offset=(200, 100) /><rect offset=(200, 100) /><text offset=(200, 100) />")
        .unwrap();
    
    let entity_count = <&Offset>::query()
        .iter(&mut world)
        .count();
        
    assert_eq!(entity_count, 3);
}

#[test]
fn parse_hierarchical_controls_produces_relationships() {
    let mut world = World::default();
    build_world(
        &mut world, 
        "<horizontal-layout-content><rect offset=(200, 100) /></horizontal-layout-content>")
        .unwrap();
    
    let relationships:Vec::<&Relationship> = <&Relationship>::query()
        .iter(&mut world)
        .collect();

    assert_eq!(relationships[0].parent, None);
    assert_eq!(relationships[0].next_sibling, None);
    assert_ne!(relationships[0].first_child, None);
    assert_ne!(relationships[0].last_child, None);

    assert_ne!(relationships[1].parent, None);
    assert_eq!(relationships[1].next_sibling, None);
    assert_ne!(relationships[1].first_child, None);
    assert_ne!(relationships[1].last_child, None);

    assert_ne!(relationships[2].parent, None);
    assert_eq!(relationships[2].next_sibling, None);
    assert_eq!(relationships[2].first_child, None);
    assert_eq!(relationships[2].last_child, None);
        
    assert_eq!(relationships.len(), 3);
}

#[test]
fn parse_malformed_property_produces_error() {
    let mut world = World::default();
    let result = build_world(
        &mut world,
        "<circle offset=(200, 100) dimensions=(200, 30x) /><rect offset=(200, 100) dimensions=(200, 300) /><text offset=(200, 100) />");
    
    assert_eq!(result, Err(AbstractSyntaxTokenError::BadDimensionsValue));
    
    let entity_count = <&Offset>::query()
        .iter(&mut world)
        .count();
        
    assert_eq!(entity_count, 1);
}

fn build_world(world: &mut World, source: &str) -> Result<(), AbstractSyntaxTokenError> {
    let mut tokenizer = AbstractSyntaxTokenizer::from_source(SourceTokenizer::from_string(source));
    tokenizer.build_world(world)
}

