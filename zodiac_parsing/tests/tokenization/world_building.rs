use legion::*;
use zodiac_parsing::tokenization::abstract_syntax::*;
use zodiac_parsing::tokenization::source::*;
use zodiac_entities::components::*;
use zodiac_parsing::tokenization::world_building::*;

#[test]
fn parse_horizontal_layoutcontainer_produces_container_components_on_entity() {
    let mut world = World::default();
    build_world(&mut world, "<horizontal-layout-content />").unwrap();
    
    let entities:Vec::<&LayoutContent> = <&LayoutContent>::query()
        .iter(&mut world)
        .collect();

    assert_eq!(entities[0].layout_type, LayoutType::Canvas);
    assert_eq!(entities[1].layout_type, LayoutType::Horizontal);
    assert_eq!(entities.len(), 2);
}

#[test]
fn parse_canvas_layout_container_produces_container_components_on_entity() {
    let mut world = World::default();
    build_world(&mut world, "<canvas />").unwrap();
    
    let entities:Vec::<&LayoutContent> = <&LayoutContent>::query()
        .iter(&mut world)
        .collect();
    
    assert_eq!(entities[0].layout_type, LayoutType::Canvas);
    assert_eq!(entities[1].layout_type, LayoutType::Canvas);
    assert_eq!(entities.len(), 2);
}

#[test]
fn parse_circle_produces_circle_components_on_entity() {
    let mut world = World::default();
    build_world(&mut world, "<circle
        left=200
        top=100
        radius=50
        colour=(1.0, 2.0, 3.0, 4.0)
        stroke-colour=(0.0, 1.0, 2.0, 3.0)
        stroke-width=3
    />").unwrap();
    let mut entities = 0;
    let mut query = <(&Left, &Top, &Radius, &Colour, &StrokeColour, &StrokeWidth)>::query()
        .filter(component::<Renderable>());

    for (left, top, radius, colour, stroke_colour, stroke_width) in query.iter(&mut world) {
        entities += 1;
        assert_eq!(left.left, 200);
        assert_eq!(top.top, 100);
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
        left=200
        top=100
        height=200
        width=300
        colour=(1.0, 2.0, 3.0, 4.0)
        stroke-colour=(0.0, 1.0, 2.0, 3.0)
        stroke-width=3
        corner-radii=(5, 15, 25, 35)
    />").unwrap();
    let mut entities = 0;
    let mut query = <(&Top, &Width, &Height, &Colour, &StrokeColour, &StrokeWidth, &CornerRadii)>::query()
        .filter(component::<Renderable>());

    for (top, width, height, colour, stroke_colour, stroke_width, corner_radii) in query.iter(&mut world) {
        entities += 1;
        assert_eq!(top.top, 100);
        assert_eq!(width.width, 300);
        assert_eq!(height.height, 200);
        assert_eq!(colour.r, 1.0);
        assert_eq!(colour.g, 2.0);
        assert_eq!(colour.b, 3.0);
        assert_eq!(colour.a, 4.0);
        assert_eq!(stroke_colour.r, 0.0);
        assert_eq!(stroke_colour.g, 1.0);
        assert_eq!(stroke_colour.b, 2.0);
        assert_eq!(stroke_colour.a, 3.0);
        assert_eq!(stroke_width.width, 3);
        assert_eq!(corner_radii.left_top, 5);
        assert_eq!(corner_radii.right_top, 15);
        assert_eq!(corner_radii.right_bottom, 25);
        assert_eq!(corner_radii.left_bottom, 35);
    }
    assert_eq!(entities, 1);
}

#[test]
fn parse_text_produces_text_components_on_entity() {
    let mut world = World::default();
    build_world(&mut world, "<text
        left=200
        top=100
        height=200
        width=300
        glyph-index=32
        colour=(1.0, 2.0, 3.0, 4.0)
    />").unwrap();
    let mut entities = 0;
    let mut query = <(&Left, &Top, &Height, &Width, &GlyphIndex, &Colour)>::query()
        .filter(component::<Renderable>());

    for (left, top, height, width, glyph_index, colour) in query.iter(&mut world) {
        entities += 1;
        assert_eq!(left.left, 200);
        assert_eq!(top.top, 100);
        assert_eq!(height.height, 200);
        assert_eq!(width.width, 300);
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
        "<circle top=200 /><rect top=200 /><text top=200 />")
        .unwrap();
    
    let entity_count = <&Top>::query()
        .iter(&mut world)
        .count();
        
    assert_eq!(entity_count, 3);
}

#[test]
fn parse_hierarchical_controls_produces_relationships() {
    let mut world = World::default();
    build_world(
        &mut world, 
        "<horizontal-layout-content><rect top=200 /></horizontal-layout-content>")
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
        "<circle width=100 colour=(1.0, 10x) /><rect Left=200 /><text Left=200/>");
    
    assert_eq!(result, Err(AbstractSyntaxTokenError::BadColourValue));
    
    let entity_count = <&Width>::query()
        .iter(&mut world)
        .count();
        
    assert_eq!(entity_count, 1);
}

fn build_world(world: &mut World, source: &str) -> Result<(), AbstractSyntaxTokenError> {
    let mut tokenizer = AbstractSyntaxTokenizer::from_source(SourceTokenizer::from_string(source));
    tokenizer.build_world(world)
}

