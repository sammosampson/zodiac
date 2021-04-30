use legion::*;
use zodiac_rendering_glium::*;
use zodiac::testing::*;
use zodiac_entities::*;
use zodiac::*;
//use zodiac::formatting::*;

#[test]
fn text_gets_output() {
    let source = "
<root>
    <text content=\"abc\" colour=(1.0, 1.0, 1.0, 0.1) />
</root>
";
    let mut runner = Application::new()
        .with_builders(&mut test_builders(Dimensions::new(100, 110)))
        .build()
        .unwrap();

    apply_initial_source(runner.resources_mut(), ".\\root.zod", source);

    runner.run_once();

    let changes: Vec::<RenderPrimitive> = <&RenderPrimitive>::query()
        .iter(runner.world_mut())
        .map(|change| *change)
        .collect();  
    
    assert_eq!(changes.len(), 1);
    assert_eq!(changes.iter().any(|change| *change == RenderPrimitive::text([0, 0], [100, 110], [1.0, 1.0, 1.0, 0.1], "abc".to_string())), true);
}

#[test]
fn text_gets_output_in_stack() {
    let source = "
<root>
    <horizontal-stack>
        <text content=\"abc\" colour=(1.0, 1.0, 1.0, 0.1) />
    </horizontal-stack>
</root>
";
    let mut runner = Application::new()
        .with_builders(&mut test_builders(Dimensions::new(100, 110)))
        .build()
        .unwrap();

    apply_initial_source(runner.resources_mut(), ".\\root.zod", source);

    runner.run_once();

    let changes: Vec::<RenderPrimitive> = <&RenderPrimitive>::query()
        .iter(runner.world_mut())
        .map(|change| *change)
        .collect();  
    
    assert_eq!(changes.len(), 1);
    assert_eq!(changes.iter().any(|change| *change == RenderPrimitive::text([0, 0], [100, 110], [1.0, 1.0, 1.0, 0.1], "abc".to_string())), true);
}