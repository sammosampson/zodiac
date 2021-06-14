use legion::*;
use mox::mox;
use zodiac::testing::*;
use zodiac::*;

#[topo::nested]
fn absolute_positioning_on_screen_app_root() -> RootNode<TestState> {
    mox!(
        <root>
            <test_renderable
                left=10
                top=11
                width=12
                height=13
            />
            <test_renderable
                left=12
                top=13
                width=14
                height=15
            />
        </root>
    )
}

#[test]
fn absolute_positioning_on_screen() {
    let mut runner = Application::new(TestState::default(), absolute_positioning_on_screen_app_root)
        .with_builders(&mut test_builders(Dimensions::new(1024, 768)))
        .with_builder(world_logging())
        .build()
        .unwrap();

    runner.run_once();

    let changes: Vec::<RenderPrimitive> = <&RenderPrimitive>::query()
        .iter(runner.world_mut())
        .map(|change| change.clone())
        .collect();
    
    assert_eq!(changes.len(), 2);
    assert_eq!(changes.iter().any(|change| *change == RenderPrimitive::test_renderable([10, 11], [12, 13])), true);
    assert_eq!(changes.iter().any(|change| *change == RenderPrimitive::test_renderable([12, 13], [14, 15])), true);
}

#[topo::nested]
fn absolute_positioning_on_canvas_offset_from_screen_app_root() -> RootNode<TestState> {
    mox!(
        <root>
            <canvas
                left=10
                top=11
            >
                <test_renderable
                    left=10
                    top=11
                />
            </canvas>
        </root>
    )
}

#[test]
fn absolute_positioning_on_canvas_offset_from_screen() {
    let mut runner = Application::new(TestState::default(), absolute_positioning_on_canvas_offset_from_screen_app_root)
        .with_builders(&mut test_builders(Dimensions::new(100, 110)))
        .with_builder(world_logging())
        .build()
        .unwrap();

    runner.run_once();

    let changes: Vec::<RenderPrimitive> = <&RenderPrimitive>::query()
        .iter(runner.world_mut())
        .map(|change| change.clone())
        .collect();

    assert_eq!(changes.iter().any(|change| *change == RenderPrimitive::test_renderable([20, 22], [100, 110])), true);
    assert_eq!(changes.len(), 1);
}

#[topo::nested]
fn dimensions_fit_parent_when_not_specified_app_root() -> RootNode<TestState> {
    mox!(
        <root>
            <horizontal_stack>
                <test_renderable
                />
            </horizontal_stack>
        </root>
    )
}

#[test]
fn dimensions_fit_parent_when_not_specified() {
    let mut runner = Application::new(TestState::default(), dimensions_fit_parent_when_not_specified_app_root)
        .with_builders(&mut test_builders(Dimensions::new(100, 110)))
        .with_builder(world_logging())
        .build()
        .unwrap();

    runner.run_once();

    let changes: Vec::<RenderPrimitive> = <&RenderPrimitive>::query()
        .iter(runner.world_mut())
        .map(|change| change.clone())
        .collect();
    
    assert_eq!(changes.iter().any(|change| *change == RenderPrimitive::test_renderable([0, 0], [100, 110])), true);
    assert_eq!(changes.len(), 1);
}

#[topo::nested]
fn horizontal_layout_for_none_sized_children_app_root() -> RootNode<TestState> {
    mox!(
        <root>
            <horizontal_stack>
                <test_renderable />
                <test_renderable />
            </horizontal_stack>
        </root>
    )
}

#[test]
fn horizontal_layout_for_none_sized_children() {
    let mut runner = Application::new(TestState::default(), horizontal_layout_for_none_sized_children_app_root)
        .with_builders(&mut test_builders(Dimensions::new(100, 100)))
        .with_builder(world_logging())
        .build()
        .unwrap();

    runner.run_once();

    let changes: Vec::<RenderPrimitive> = <&RenderPrimitive>::query()
        .iter(runner.world_mut())
        .map(|change| change.clone())
        .collect(); 
        
    assert_eq!(changes.iter().any(|change| *change == RenderPrimitive::test_renderable([0, 0], [50, 100])), true);
    assert_eq!(changes.iter().any(|change| *change == RenderPrimitive::test_renderable([50, 0], [50, 100])), true);
    assert_eq!(changes.len(), 2);
}

#[topo::nested]
fn horizontal_layout_for_sized_children_app_root() -> RootNode<TestState> {
    mox!(
        <root>
            <horizontal_stack>
                <test_renderable width=25 />
                <test_renderable />
                <test_renderable width=35 />
            </horizontal_stack>
        </root>
    )
}

#[test]
fn horizontal_layout_for_sized_children() {
    let mut runner = Application::new(TestState::default(), horizontal_layout_for_sized_children_app_root)
        .with_builders(&mut test_builders(Dimensions::new(100, 100)))
        .with_builder(world_logging())
        .build()
        .unwrap();

    runner.run_once();

    let changes: Vec::<RenderPrimitive> = <&RenderPrimitive>::query()
        .iter(runner.world_mut())
        .map(|change| change.clone())
        .collect();

    assert_eq!(changes.iter().any(|change| *change == RenderPrimitive::test_renderable([0, 0], [25, 100])), true);
    assert_eq!(changes.iter().any(|change| *change == RenderPrimitive::test_renderable([25, 0], [40, 100])), true);
    assert_eq!(changes.iter().any(|change| *change == RenderPrimitive::test_renderable([65, 0], [35, 100])), true);
    assert_eq!(changes.len(), 3);
}

#[topo::nested]
fn vertical_layout_for_none_sized_children_app_root() -> RootNode<TestState> {
    mox!(
        <root>
            <vertical_stack>
                <test_renderable />
                <test_renderable />
            </vertical_stack>
        </root>
    )
}

#[test]
fn vertical_layout_for_none_sized_children() {
    let mut runner = Application::new(TestState::default(), vertical_layout_for_none_sized_children_app_root)
        .with_builders(&mut test_builders(Dimensions::new(100, 100)))
        .with_builder(world_logging())
        .build()
        .unwrap();

    runner.run_once();

    let changes: Vec::<RenderPrimitive> = <&RenderPrimitive>::query()
        .iter(runner.world_mut())
        .map(|change| change.clone())
        .collect();
        
    assert_eq!(changes.iter().any(|change| *change == RenderPrimitive::test_renderable([0, 0], [100, 50])), true);
    assert_eq!(changes.iter().any(|change| *change == RenderPrimitive::test_renderable([0, 50], [100, 50])), true);
    assert_eq!(changes.len(), 2);
}


#[topo::nested]
fn vertical_layout_for_sized_children_app_root() -> RootNode<TestState> {
    mox!(
        <root>
            <vertical_stack>
                <test_renderable height=25 />
                <test_renderable />
                <test_renderable height=35 />
            </vertical_stack>
        </root>
    )
}

#[test]
fn vertical_layout_for_sized_children() {
    let mut runner = Application::new(TestState::default(), vertical_layout_for_sized_children_app_root)
        .with_builders(&mut test_builders(Dimensions::new(100, 100)))
        .with_builder(world_logging())
        .build()
        .unwrap();

    runner.run_once();

    let changes: Vec::<RenderPrimitive> = <&RenderPrimitive>::query()
        .iter(runner.world_mut())
        .map(|change| change.clone())
        .collect();

    assert_eq!(changes.iter().any(|change| *change == RenderPrimitive::test_renderable([0, 0], [100, 25])), true);
    assert_eq!(changes.iter().any(|change| *change == RenderPrimitive::test_renderable([0, 25], [100, 40])), true);
    assert_eq!(changes.iter().any(|change| *change == RenderPrimitive::test_renderable([0, 65], [100, 35])), true);
    assert_eq!(changes.len(), 3);
}