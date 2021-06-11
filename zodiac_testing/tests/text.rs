use legion::*;
use mox::mox;
use zodiac_testing::*;
use zodiac::*;

#[topo::nested]
fn text_gets_output_app_root() -> RootNode<TestState> {
    mox!(
        <root>
            <text content="abc".to_string() font_size=32 colour=(255, 255, 255, 25) />
        </root>
    )
}

#[test]
fn text_gets_output() {
    let mut runner = Application::new(TestState::default(), text_gets_output_app_root)
        .with_builders(&mut test_builders(Dimensions::new(100, 110)))
        .with_builder(world_logging())
        .build()
        .unwrap();

    runner.run_once();

    let changes: Vec::<RenderPrimitive> = <&RenderPrimitive>::query()
        .iter(runner.world_mut())
        .map(|change| change.clone())
        .collect();  
    
    assert_eq!(changes.len(), 1);
    assert_eq!(changes.iter().any(|change| *change == RenderPrimitive::text([0, 0], [100, 110], [255, 255, 255, 25], "abc".to_string(), 32)), true);
}

#[topo::nested]
fn text_gets_output_in_stack_app_root() -> RootNode<TestState> {
    mox!(
        <root>
            <horizontal_stack>
                <text content="abc".to_string() font_size=32 colour=(255, 255, 255, 25) />
            </horizontal_stack>
        </root>
    )
}

#[test]
fn text_gets_output_in_stack() {
    let mut runner = Application::new(TestState::default(), text_gets_output_in_stack_app_root)
        .with_builders(&mut test_builders(Dimensions::new(100, 110)))
        .with_builder(world_logging())
        .build()
        .unwrap();

    runner.run_once();

    let changes: Vec::<RenderPrimitive> = <&RenderPrimitive>::query()
        .iter(runner.world_mut())
        .map(|change| change.clone())
        .collect();  
    
    assert_eq!(changes.len(), 1);
    assert_eq!(changes.iter().any(|change| *change == RenderPrimitive::text([0, 0], [100, 110], [255, 255, 255, 25], "abc".to_string(), 32)), true);
}