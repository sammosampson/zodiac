use legion::*;
use mox::mox;
use zodiac::testing::*;
use zodiac_entities::*;
use zodiac_source::*;

#[topo::nested]
fn absolute_positioning_on_screen_app_root() -> RootNode<TestState> {
    mox!(
        <root>
            <rect
                left=10
                top=11
                width=12
                height=13
                colour=(255, 255, 255, 25)
                stroke_colour=(50, 75, 255, 255)
                stroke_width=2
                corner_radii=(50, 0, 50, 50)
            />
            <circle
                left=11
                top=12
                radius=12
                colour=(100, 100, 100, 25)
                stroke_colour=(255, 255, 255, 255)
                stroke_width=3
            />
            <rect
                left=12
                top=13
                width=14
                height=15
                colour=(255, 255, 255, 25)
                stroke_colour=(50, 75, 255, 255)
                stroke_width=4
                corner_radii=(50, 0, 50, 50)
            />
        </root>
    )
}

#[test]
fn absolute_positioning_on_screen() {
    let mut runner = Application::new()
        .with_builders(&mut test_builders(absolute_positioning_on_screen_app_root, Dimensions::new(1024, 768)))
        .with_builder(world_logging())
        .build()
        .unwrap();

    runner.run_once();

    let changes: Vec::<RenderPrimitive> = <&RenderPrimitive>::query()
        .iter(runner.world_mut())
        .map(|change| change.clone())
        .collect();
    
    assert_eq!(changes.len(), 3);
    assert_eq!(changes.iter().any(|change| *change == RenderPrimitive::rectangle([10, 11], [12, 13], [255, 255, 255, 25], [50, 75, 255, 255], 2, [50, 0, 50, 50])), true);
    assert_eq!(changes.iter().any(|change| *change == RenderPrimitive::circle([11, 12], 12, [100, 100, 100, 25], [255, 255, 255, 255], 3)), true);
    assert_eq!(changes.iter().any(|change| *change == RenderPrimitive::rectangle([12, 13], [14, 15], [255, 255, 255, 25], [50, 75, 255, 255], 4, [50, 0, 50, 50])), true);
}

#[topo::nested]
fn absolute_positioning_on_canvas_offset_from_screen_app_root() -> RootNode<TestState> {
    mox!(
        <root>
            <canvas
                left=10
                top=11
            >
                <rect
                    left=10
                    top=11
                    colour=(255, 255, 255, 25)
                    stroke_colour=(50, 75, 255, 255)
                    stroke_width=2
                    corner_radii=(50, 0, 50, 50)
                />
            </canvas>
        </root>
    )
}

#[test]
fn absolute_positioning_on_canvas_offset_from_screen() {
    let mut runner = Application::new()
        .with_builders(&mut test_builders(absolute_positioning_on_canvas_offset_from_screen_app_root, Dimensions::new(100, 110)))
        .with_builder(world_logging())
        .build()
        .unwrap();

    runner.run_once();

    let changes: Vec::<RenderPrimitive> = <&RenderPrimitive>::query()
        .iter(runner.world_mut())
        .map(|change| change.clone())
        .collect();

    assert_eq!(changes.iter().any(|change| *change == RenderPrimitive::rectangle([20, 22], [100, 110], [255, 255, 255, 25], [50, 75, 255, 255], 2, [50, 0, 50, 50])), true);
    assert_eq!(changes.len(), 1);
}

#[topo::nested]
fn dimensions_fit_parent_when_not_specified_app_root() -> RootNode<TestState> {
    mox!(
        <root>
            <horizontal_stack>
                <rect
                    colour=(255, 255, 255, 25)
                    stroke_colour=(50, 75, 255, 255)
                    stroke_width=2
                    corner_radii=(50, 0, 50, 50)
                />
            </horizontal_stack>
        </root>
    )
}

#[test]
fn dimensions_fit_parent_when_not_specified() {
    let mut runner = Application::new()
        .with_builders(&mut test_builders(dimensions_fit_parent_when_not_specified_app_root, Dimensions::new(100, 110)))
        .with_builder(world_logging())
        .build()
        .unwrap();

    runner.run_once();

    let changes: Vec::<RenderPrimitive> = <&RenderPrimitive>::query()
        .iter(runner.world_mut())
        .map(|change| change.clone())
        .collect();
    
    assert_eq!(changes.iter().any(|change| *change == RenderPrimitive::rectangle([0, 0], [100, 110], [255, 255, 255, 25], [50, 75, 255, 255], 2, [50, 0, 50, 50])), true);
    assert_eq!(changes.len(), 1);
}

#[topo::nested]
fn horizontal_layout_for_none_sized_children_app_root() -> RootNode<TestState> {
    mox!(
        <root>
            <horizontal_stack>
                <rect
                    colour=(255, 255, 255, 25)
                    stroke_colour=(50, 75, 255, 255)
                    stroke_width=2
                    corner_radii=(50, 0, 50, 50)
                />
                <rect
                    colour=(255, 255, 255, 25)
                    stroke_colour=(50, 75, 255, 255)
                    stroke_width=2
                    corner_radii=(50, 0, 50, 50)
                />
            </horizontal_stack>
        </root>
    )
}

#[test]
fn horizontal_layout_for_none_sized_children() {
    let mut runner = Application::new()
        .with_builders(&mut test_builders(horizontal_layout_for_none_sized_children_app_root, Dimensions::new(100, 100)))
        .with_builder(world_logging())
        .build()
        .unwrap();

    runner.run_once();

    let changes: Vec::<RenderPrimitive> = <&RenderPrimitive>::query()
        .iter(runner.world_mut())
        .map(|change| change.clone())
        .collect(); 
        
    assert_eq!(changes.iter().any(|change| *change == RenderPrimitive::rectangle([0, 0], [50, 100], [255, 255, 255, 25], [50, 75, 255, 255], 2, [50, 0, 50, 50])), true);
    assert_eq!(changes.iter().any(|change| *change == RenderPrimitive::rectangle([50, 0], [50, 100], [255, 255, 255, 25], [50, 75, 255, 255], 2, [50, 0, 50, 50])), true);
    assert_eq!(changes.len(), 2);
}

#[topo::nested]
fn horizontal_layout_for_sized_children_app_root() -> RootNode<TestState> {
    mox!(
        <root>
            <horizontal_stack>
                <rect
                    width=25
                    colour=(255, 255, 255, 25)
                    stroke_colour=(50, 75, 255, 255)
                    stroke_width=2
                    corner_radii=(50, 0, 50, 50)
                />
                <rect
                    colour=(255, 255, 255, 25)
                    stroke_colour=(50, 75, 255, 255)
                    stroke_width=2
                    corner_radii=(50, 0, 50, 50)
                />
                <rect
                    width=35
                    colour=(255, 255, 255, 25)
                    stroke_colour=(50, 75, 255, 255)
                    stroke_width=2
                    corner_radii=(50, 0, 50, 50)
                />
            </horizontal_stack>
        </root>
    )
}

#[test]
fn horizontal_layout_for_sized_children() {
    let mut runner = Application::new()
        .with_builders(&mut test_builders(horizontal_layout_for_sized_children_app_root, Dimensions::new(100, 100)))
        .with_builder(world_logging())
        .build()
        .unwrap();

    runner.run_once();

    let changes: Vec::<RenderPrimitive> = <&RenderPrimitive>::query()
        .iter(runner.world_mut())
        .map(|change| change.clone())
        .collect();

    assert_eq!(changes.iter().any(|change| *change == RenderPrimitive::rectangle([0, 0], [25, 100], [255, 255, 255, 25], [50, 75, 255, 255], 2, [50, 0, 50, 50])), true);
    assert_eq!(changes.iter().any(|change| *change == RenderPrimitive::rectangle([25, 0], [40, 100], [255, 255, 255, 25], [50, 75, 255, 255], 2, [50, 0, 50, 50])), true);
    assert_eq!(changes.iter().any(|change| *change == RenderPrimitive::rectangle([65, 0], [35, 100], [255, 255, 255, 25], [50, 75, 255, 255], 2, [50, 0, 50, 50])), true);
    assert_eq!(changes.len(), 3);
}

#[topo::nested]
fn vertical_layout_for_none_sized_children_app_root() -> RootNode<TestState> {
    mox!(
        <root>
            <vertical_stack>
                <rect
                    colour=(255, 255, 255, 25)
                    stroke_colour=(50, 75, 255, 255)
                    stroke_width=2
                    corner_radii=(50, 0, 50, 50)
                />
                <rect
                    colour=(255, 255, 255, 25)
                    stroke_colour=(50, 75, 255, 255)
                    stroke_width=2
                    corner_radii=(50, 0, 50, 50)
                />
            </vertical_stack>
        </root>
    )
}

#[test]
fn vertical_layout_for_none_sized_children() {
    let mut runner = Application::new()
        .with_builders(&mut test_builders(vertical_layout_for_none_sized_children_app_root, Dimensions::new(100, 100)))
        .with_builder(world_logging())
        .build()
        .unwrap();

    runner.run_once();

    let changes: Vec::<RenderPrimitive> = <&RenderPrimitive>::query()
        .iter(runner.world_mut())
        .map(|change| change.clone())
        .collect();
        
    assert_eq!(changes.iter().any(|change| *change == RenderPrimitive::rectangle([0, 0], [100, 50], [255, 255, 255, 25], [50, 75, 255, 255], 2, [50, 0, 50, 50])), true);
    assert_eq!(changes.iter().any(|change| *change == RenderPrimitive::rectangle([0, 50], [100, 50], [255, 255, 255, 25], [50, 75, 255, 255], 2, [50, 0, 50, 50])), true);
    assert_eq!(changes.len(), 2);
}


#[topo::nested]
fn vertical_layout_for_sized_children_app_root() -> RootNode<TestState> {
    mox!(
        <root>
            <vertical_stack>
                <rect
                    height=25
                    colour=(255, 255, 255, 25)
                    stroke_colour=(50, 75, 255, 255)
                    stroke_width=2
                    corner_radii=(50, 0, 50, 50)
                />
                <rect
                    colour=(255, 255, 255, 25)
                    stroke_colour=(50, 75, 255, 255)
                    stroke_width=2
                    corner_radii=(50, 0, 50, 50)
                />
                <rect
                    height=35
                    colour=(255, 255, 255, 25)
                    stroke_colour=(50, 75, 255, 255)
                    stroke_width=2
                    corner_radii=(50, 0, 50, 50)
                />
            </vertical_stack>
        </root>
    )
}

#[test]
fn vertical_layout_for_sized_children() {
    let mut runner = Application::new()
        .with_builders(&mut test_builders(vertical_layout_for_sized_children_app_root, Dimensions::new(100, 100)))
        .with_builder(world_logging())
        .build()
        .unwrap();

    runner.run_once();

    let changes: Vec::<RenderPrimitive> = <&RenderPrimitive>::query()
        .iter(runner.world_mut())
        .map(|change| change.clone())
        .collect();

    assert_eq!(changes.iter().any(|change| *change == RenderPrimitive::rectangle([0, 0], [100, 25], [255, 255, 255, 25], [50, 75, 255, 255], 2, [50, 0, 50, 50])), true);
    assert_eq!(changes.iter().any(|change| *change == RenderPrimitive::rectangle([0, 25], [100, 40], [255, 255, 255, 25], [50, 75, 255, 255], 2, [50, 0, 50, 50])), true);
    assert_eq!(changes.iter().any(|change| *change == RenderPrimitive::rectangle([0, 65], [100, 35], [255, 255, 255, 25], [50, 75, 255, 255], 2, [50, 0, 50, 50])), true);
    assert_eq!(changes.len(), 3);
}