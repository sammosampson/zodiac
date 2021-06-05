/* use legion::*;
use zodiac::testing::*;
use zodiac_entities::*;
use zodiac_source::embedding::*;
use zodiac::*;
use mox::mox;
//use zodiac::formatting::*;

#[topo::nested]
pub fn app_root() -> RootNode<TestState> {
    mox!(
        <root>
            <circle radius=12 />
        </root>
    )
}

#[test]
fn invalid_source_causes_top_level_error_circle_renderable_control_output() {
    let mut runner = Application::new()
        .with_builders(&mut test_builders(Dimensions::new(100, 100)))
        .build()
        .unwrap();
    
    runner.run_once();
    
    let changes: Vec::<RenderPrimitive> = <&RenderPrimitive>::query()
        .iter(runner.world_mut())
        .map(|change| change.clone())
        .collect();
    
    assert_eq!(changes.len(), 1);
    assert_eq!(changes[0], RenderPrimitive::circle([0, 0], 100, [255, 0, 0, 255], [0, 0, 0, 0], 0));
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
        .with_builders(&mut test_builders(Dimensions::new(100, 100)))
        .build()
        .unwrap();

    apply_initial_source(runner.resources_mut(), ".\\root.zod", root);

    runner.run_once();

    let changes: Vec::<RenderPrimitive> = <&RenderPrimitive>::query()
        .iter(runner.world_mut())
        .map(|change| change.clone())
        .collect();
    
    assert_eq!(changes.len(), 1);
    assert_eq!(changes[0], RenderPrimitive::rectangle([0, 0], [100, 100], [255, 0, 0, 255], [0, 0, 0, 0], 0, [0, 0, 0, 0]));
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
        .with_builders(&mut test_builders(Dimensions::new(100, 100)))
        .build()
        .unwrap();

    apply_initial_source(runner.resources_mut(), ".\\root.zod", root);

    runner.run_once();

    let changes: Vec::<RenderPrimitive> = <&RenderPrimitive>::query()
        .iter(runner.world_mut())
        .map(|change| change.clone())
        .collect();

    assert_eq!(changes.len(), 1);
    assert_eq!(changes[0], RenderPrimitive::rectangle([0, 0], [100, 100], [255, 0, 0, 255], [0, 0, 0, 0], 0, [0, 0, 0, 0]));
}

#[test]
fn invalid_control_causes_top_level_error_control_output() {
    let root = "
<root>
    <big-control/>
</root>
";  
    let mut runner = Application::new()
        .with_builders(&mut test_builders(Dimensions::new(100, 100)))
        .build()
        .unwrap();

    apply_initial_source(runner.resources_mut(), ".\\root.zod", root);

    runner.run_once();

    let changes: Vec::<RenderPrimitive> = <&RenderPrimitive>::query()
        .iter(runner.world_mut())
        .map(|change| change.clone())
        .collect();
    
    assert_eq!(changes.len(), 1);
    assert_eq!(changes[0], RenderPrimitive::rectangle([0, 0], [100, 100], [255, 0, 0, 255], [0, 0, 0, 0], 0, [0, 0, 0, 0]));
}

#[test]
fn error_control_does_not_apply_twice() {
    let root = "
<root>
    <big-control/>
</root>
";  
    let mut runner = Application::new()
        .with_builders(&mut test_builders(Dimensions::new(100, 100)))
        .build()
        .unwrap();

    apply_initial_source(runner.resources_mut(), ".\\root.zod", root);

    runner.run_once();
    runner.run_once();

    let changes: Vec::<RenderPrimitive> = <&RenderPrimitive>::query()
        .iter(runner.world_mut())
        .map(|change| change.clone())
        .collect();
    
    assert_eq!(changes.len(), 1);
}
 */