use legion::*;
use zodiac_rendering_glium::*;
use zodiac::test_helpers::*;

#[test]
fn text_gets_output_as_glyphs() {
    let source = "
<root>
    <text content=\"abc\" colour=(1.0, 1.0, 1.0, 0.1) />
</root>
";
    let mut world = World::default();
    let mut resources = build_zodiac_resources();
    let mut schedule = build_zodiac_systems_schedule();

    apply_initial_source(&mut resources, ".\\root.zod", source);
    
    notify_resize_root_window(&mut world, (100, 100));

    schedule.execute(&mut world, &mut resources);

    let changes: Vec::<RenderPrimitive> = <&RenderPrimitive>::query()
        .iter(&mut world)
        .map(|change| *change)
        .collect();   
    
    assert_eq!(changes.iter().any(|change| *change == RenderPrimitive::glyph([0, 0], [16, 16], [1.0, 1.0, 1.0, 0.1], 35)), true);
    assert_eq!(changes.iter().any(|change| *change == RenderPrimitive::glyph([16, 0], [16, 16], [1.0, 1.0, 1.0, 0.1], 36)), true);
    assert_eq!(changes.iter().any(|change| *change == RenderPrimitive::glyph([32, 0], [16, 16], [1.0, 1.0, 1.0, 0.1], 37)), true);
    assert_eq!(changes.len(), 3);
}
