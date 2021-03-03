use legion::*;
use zodiac_entities::world_building::*;
use zodiac_entities::components::*;
use zodiac_layout::text::*;

#[test]
fn format_glyphs_system_format_glyphs() {
    let mut world = World::default();
    let mut resources = Resources::default();
    let mut schedule = Schedule::builder()
        .add_system(build_text_colour_map_system())
        .add_system(format_glyphs_system())
        .build();

    let mut builder = WorldEntityBuilder::for_world(&mut world);
    
    builder.create_canvas_layout_content_entity();
    builder.add_colour_component(1.0, 2.0, 3.0, 4.0);

    builder.create_glyph_entity();
    builder.add_character_component('a', 0);
    builder.complete_entity();
    
    builder.create_glyph_entity();
    builder.add_character_component('b', 1);
    builder.complete_entity();
    
    builder.create_glyph_entity();
    builder.add_character_component('c', 2);
    builder.complete_entity();
    
    builder.complete_entity();

    resources.insert(create_text_colour_map());

    schedule.execute(&mut world, &mut resources);
    let mut query = <(&Left, &Top, &Width, &Height, &GlyphIndex, &Colour)>::query();
    let mut iter = query.iter(&mut world);

    let (left, top, width, height, glyph_index, colour) = iter.next().unwrap();
    assert_eq!(left.left, 0);
    assert_eq!(top.top, 0);
    assert_eq!(width.width, 16);
    assert_eq!(height.height, 16);
    assert_eq!(glyph_index.index, 35);
    assert_eq!(colour.r, 1.0);
    assert_eq!(colour.g, 2.0);
    assert_eq!(colour.b, 3.0);
    assert_eq!(colour.a, 4.0);

    let (left, top, width, height, glyph_index, colour) = iter.next().unwrap();
    assert_eq!(left.left, 16);
    assert_eq!(top.top, 0);
    assert_eq!(width.width, 16);
    assert_eq!(height.height, 16);
    assert_eq!(glyph_index.index, 36);
    assert_eq!(colour.r, 1.0);
    assert_eq!(colour.g, 2.0);
    assert_eq!(colour.b, 3.0);
    assert_eq!(colour.a, 4.0);

    let (left, top, width, height, glyph_index, colour) = iter.next().unwrap();
    assert_eq!(left.left, 32);
    assert_eq!(top.top, 0);
    assert_eq!(width.width, 16);
    assert_eq!(height.height, 16);
    assert_eq!(glyph_index.index, 37);
    assert_eq!(colour.r, 1.0);
    assert_eq!(colour.g, 2.0);
    assert_eq!(colour.b, 3.0);
    assert_eq!(colour.a, 4.0);
    
    assert_eq!(iter.next(), None);
}
