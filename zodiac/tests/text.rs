use legion::*;
use zodiac_rendering_glium::*;
use zodiac::testing::*;
use zodiac_entities::*;
use zodiac::*;
use zodiac_layout::*;
//use zodiac::formatting::*;

#[test]
fn text_gets_output_as_glyphs() {
    let source = "
<root>
    <text content=\"abc\" colour=(1.0, 1.0, 1.0, 0.1) />
</root>
";
    let mut runner = Application::new()
        .with_builder(test_source_file_building())
        .with_builder(test_source_building())
        .with_builder(standard_layout())
        .with_builder(standard_test_rendering())
        .with_builder(test_renderer(Dimensions::new(100, 110)))
        .build()
        .unwrap();

    apply_initial_source(runner.resources_mut(), ".\\root.zod", source);

    runner.run_once();

    let changes: Vec::<RenderPrimitive> = <&RenderPrimitive>::query()
        .iter(runner.world_mut())
        .map(|change| *change)
        .collect();  
    
    assert_eq!(changes.len(), 3);
    assert_eq!(changes.iter().any(|change| *change == RenderPrimitive::glyph([0, 0], [16, 16], [1.0, 1.0, 1.0, 0.1], 35)), true);
    assert_eq!(changes.iter().any(|change| *change == RenderPrimitive::glyph([16, 0], [16, 16], [1.0, 1.0, 1.0, 0.1], 36)), true);
    assert_eq!(changes.iter().any(|change| *change == RenderPrimitive::glyph([32, 0], [16, 16], [1.0, 1.0, 1.0, 0.1], 37)), true);
}