use legion::*;
use zodiac_rendering_glium::*;
use zodiac::testing::*;
use zodiac_entities::*;
use zodiac::*;
use zodiac_layout::*;
//use zodiac::formatting::*;

#[test]
fn invalid_source_causes_top_level_error_circle_renderable_control_output() {
    let root = "
<root>
    <circle
        radius=
    />
</root>
";  
    let mut runner = Application::new()
        .with_builder(test_source_file_building())
        .with_builder(test_source_building())
        .with_builder(standard_layout())
        .with_builder(standard_test_rendering())
        .with_builder(test_renderer(Dimensions::new(100, 100)))
        .build()
        .unwrap();
    
    apply_initial_source(runner.resources_mut(), ".\\root.zod", root);
    
    runner.run_once();
    
    let changes: Vec::<RenderPrimitive> = <&RenderPrimitive>::query()
        .iter(runner.world_mut())
        .map(|change| *change)
        .collect();
    
    assert_eq!(changes.len(), 1);
    assert_eq!(changes[0], RenderPrimitive::circle([0, 0], 100, [1.0, 0.0, 0.0, 1.0], [0.0, 0.0, 0.0, 0.0], 0.0));
}

#[test]
fn invalid_source_causes_top_level_error_rect_renderable_control_output() {
    let root = "
<root>
    <rect
        left=
    />
</root>
";  
    let mut runner = Application::new()
        .with_builder(test_source_file_building())
        .with_builder(test_source_building())
        .with_builder(standard_layout())
        .with_builder(standard_test_rendering())
        .with_builder(test_renderer(Dimensions::new(100, 100)))
        .build()
        .unwrap();

    apply_initial_source(runner.resources_mut(), ".\\root.zod", root);

    runner.run_once();

    let changes: Vec::<RenderPrimitive> = <&RenderPrimitive>::query()
        .iter(runner.world_mut())
        .map(|change| *change)
        .collect();
    
    assert_eq!(changes.len(), 1);
    assert_eq!(changes[0], RenderPrimitive::rectangle([0, 0], [100, 100], [1.0, 0.0, 0.0, 1.0], [0.0, 0.0, 0.0, 0.0], 0.0, [0, 0, 0, 0]));
}

#[test]
fn invalid_import_causes_top_level_error_control_output() {
    let root = "
<import name=\"big-control\" path=\".\\big_control.zod\" />         
<root>
    <big-control/>
</root>
";  
    let mut runner = Application::new()
        .with_builder(test_source_file_building())
        .with_builder(test_source_building())
        .with_builder(standard_layout())
        .with_builder(standard_test_rendering())
        .with_builder(test_renderer(Dimensions::new(100, 100)))
        .build()
        .unwrap();

    apply_initial_source(runner.resources_mut(), ".\\root.zod", root);

    runner.run_once();

    let changes: Vec::<RenderPrimitive> = <&RenderPrimitive>::query()
        .iter(runner.world_mut())
        .map(|change| *change)
        .collect();

    assert_eq!(changes.len(), 1);
    assert_eq!(changes[0], RenderPrimitive::rectangle([0, 0], [100, 100], [1.0, 0.0, 0.0, 1.0], [0.0, 0.0, 0.0, 0.0], 0.0, [0, 0, 0, 0]));
}

#[test]
fn invalid_control_causes_top_level_error_control_output() {
    let root = "
<root>
    <big-control/>
</root>
";  
    let mut runner = Application::new()
        .with_builder(test_source_file_building())
        .with_builder(test_source_building())
        .with_builder(standard_layout())
        .with_builder(standard_test_rendering())
        .with_builder(test_renderer(Dimensions::new(100, 100)))
        .build()
        .unwrap();

    apply_initial_source(runner.resources_mut(), ".\\root.zod", root);

    runner.run_once();

    let changes: Vec::<RenderPrimitive> = <&RenderPrimitive>::query()
        .iter(runner.world_mut())
        .map(|change| *change)
        .collect();
    
    assert_eq!(changes.len(), 1);
    assert_eq!(changes[0], RenderPrimitive::rectangle([0, 0], [100, 100], [1.0, 0.0, 0.0, 1.0], [0.0, 0.0, 0.0, 0.0], 0.0, [0, 0, 0, 0]));
}

#[test]
fn error_control_does_not_apply_twice() {
    let root = "
<root>
    <big-control/>
</root>
";  
    let mut runner = Application::new()
        .with_builder(test_source_file_building())
        .with_builder(test_source_building())
        .with_builder(standard_layout())
        .with_builder(standard_test_rendering())
        .with_builder(test_renderer(Dimensions::new(100, 100)))
        .build()
        .unwrap();

    apply_initial_source(runner.resources_mut(), ".\\root.zod", root);

    runner.run_once();
    runner.run_once();

    let changes: Vec::<RenderPrimitive> = <&RenderPrimitive>::query()
        .iter(runner.world_mut())
        .map(|change| *change)
        .collect();
    
    assert_eq!(changes.len(), 1);
}
