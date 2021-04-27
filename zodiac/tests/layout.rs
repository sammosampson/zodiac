use legion::*;
use zodiac_rendering_glium::*;
use zodiac::testing::*;
use zodiac_entities::*;
use zodiac::*;
//use zodiac::formatting::*;

#[test]
fn absolute_positioning_on_screen() {
    let source = "
<root>
    <rect
        left=10
        top=11
        width=12
        height=13
        colour=(1.0, 1.0, 1.0, 0.1)
        stroke-colour=(0.2, 0.3, 1.0, 1.0)
        stroke-width=2
        corner-radii=(50, 0, 50, 50)
    />
    <circle
        left=11
        top=12
        radius=12
        colour=(0.4, 0.4, 0.4, 0.1)
        stroke-colour=(1.0, 1.0, 1.0, 1.0)
        stroke-width=3
    />
    <rect
        left=12
        top=13
        width=14
        height=15
        colour=(1.0, 1.0, 1.0, 0.1)
        stroke-colour=(0.2, 0.3, 1.0, 1.0)
        stroke-width=4
        corner-radii=(50, 0, 50, 50)
    />
</root>
";
    let mut runner = Application::new()
        .with_builders(&mut test_builders(Dimensions::new(1024, 768)))
        .build()
        .unwrap();

    apply_initial_source(runner.resources_mut(), ".\\root.zod", source);

    runner.run_once();

    let changes: Vec::<RenderPrimitive> = <&RenderPrimitive>::query()
        .iter(runner.world_mut())
        .map(|change| *change)
        .collect();
    
    assert_eq!(changes.len(), 3);
    assert_eq!(changes.iter().any(|change| *change == RenderPrimitive::rectangle([10, 11], [12, 13], [1.0, 1.0, 1.0, 0.1], [0.2, 0.3, 1.0, 1.0], 2.0, [50, 0, 50, 50])), true);
    assert_eq!(changes.iter().any(|change| *change == RenderPrimitive::circle([11, 12], 12, [0.4, 0.4, 0.4, 0.1], [1.0, 1.0, 1.0, 1.0], 3.0)), true);
    assert_eq!(changes.iter().any(|change| *change == RenderPrimitive::rectangle([12, 13], [14, 15], [1.0, 1.0, 1.0, 0.1], [0.2, 0.3, 1.0, 1.0], 4.0, [50, 0, 50, 50])), true);
}

#[test]
fn absolute_positioning_on_canvas_offset_from_screen() {
    let source = "
<root>
    <canvas
        left=10
        top=11
    >
        <rect
            left=10
            top=11
            colour=(1.0, 1.0, 1.0, 0.1)
            stroke-colour=(0.2, 0.3, 1.0, 1.0)
            stroke-width=2
            corner-radii=(50, 0, 50, 50)
        />
    </canvas>
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

    assert_eq!(changes.iter().any(|change| *change == RenderPrimitive::rectangle([20, 22], [100, 110], [1.0, 1.0, 1.0, 0.1], [0.2, 0.3, 1.0, 1.0], 2.0, [50, 0, 50, 50])), true);
    assert_eq!(changes.len(), 1);
}

#[test]
fn dimensions_fit_parent_when_not_specified() {
    let source = "
<root>
    <horizontal-stack>
        <rect
            colour=(1.0, 1.0, 1.0, 0.1)
            stroke-colour=(0.2, 0.3, 1.0, 1.0)
            stroke-width=2
            corner-radii=(50, 0, 50, 50)
        />
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
    
    assert_eq!(changes.iter().any(|change| *change == RenderPrimitive::rectangle([0, 0], [100, 110], [1.0, 1.0, 1.0, 0.1], [0.2, 0.3, 1.0, 1.0], 2.0, [50, 0, 50, 50])), true);
    assert_eq!(changes.len(), 1);
}

#[test]
fn horizontal_layout_for_none_sized_children() {
    let source = "
<root>
    <horizontal-stack>
        <rect
            colour=(1.0, 1.0, 1.0, 0.1)
            stroke-colour=(0.2, 0.3, 1.0, 1.0)
            stroke-width=2
            corner-radii=(50, 0, 50, 50)
        />
        <rect
            colour=(1.0, 1.0, 1.0, 0.1)
            stroke-colour=(0.2, 0.3, 1.0, 1.0)
            stroke-width=2
            corner-radii=(50, 0, 50, 50)
        />
    </horizontal-stack>
</root>
";
    let mut runner = Application::new()
        .with_builders(&mut test_builders(Dimensions::new(100, 100)))
        .build()
        .unwrap();

    apply_initial_source(runner.resources_mut(), ".\\root.zod", source);

    runner.run_once();

    let changes: Vec::<RenderPrimitive> = <&RenderPrimitive>::query()
        .iter(runner.world_mut())
        .map(|change| *change)
        .collect(); 
        
    assert_eq!(changes.iter().any(|change| *change == RenderPrimitive::rectangle([0, 0], [50, 100], [1.0, 1.0, 1.0, 0.1], [0.2, 0.3, 1.0, 1.0], 2.0, [50, 0, 50, 50])), true);
    assert_eq!(changes.iter().any(|change| *change == RenderPrimitive::rectangle([50, 0], [50, 100], [1.0, 1.0, 1.0, 0.1], [0.2, 0.3, 1.0, 1.0], 2.0, [50, 0, 50, 50])), true);
    assert_eq!(changes.len(), 2);
}

#[test]
fn horizontal_layout_for_sized_children() {
    let source = "
<root>
    <horizontal-stack>
        <rect
            width=25
            colour=(1.0, 1.0, 1.0, 0.1)
            stroke-colour=(0.2, 0.3, 1.0, 1.0)
            stroke-width=2
            corner-radii=(50, 0, 50, 50)
        />
        <rect
            colour=(1.0, 1.0, 1.0, 0.1)
            stroke-colour=(0.2, 0.3, 1.0, 1.0)
            stroke-width=2
            corner-radii=(50, 0, 50, 50)
        />
        <rect
            width=35
            colour=(1.0, 1.0, 1.0, 0.1)
            stroke-colour=(0.2, 0.3, 1.0, 1.0)
            stroke-width=2
            corner-radii=(50, 0, 50, 50)
        />
    </horizontal-stack>
</root>
";
    let mut runner = Application::new()
        .with_builders(&mut test_builders(Dimensions::new(100, 100)))
        .build()
        .unwrap();

    apply_initial_source(runner.resources_mut(), ".\\root.zod", source);

    runner.run_once();

    let changes: Vec::<RenderPrimitive> = <&RenderPrimitive>::query()
        .iter(runner.world_mut())
        .map(|change| *change)
        .collect();
    
    assert_eq!(changes.iter().any(|change| *change == RenderPrimitive::rectangle([0, 0], [25, 100], [1.0, 1.0, 1.0, 0.1], [0.2, 0.3, 1.0, 1.0], 2.0, [50, 0, 50, 50])), true);
    assert_eq!(changes.iter().any(|change| *change == RenderPrimitive::rectangle([25, 0], [40, 100], [1.0, 1.0, 1.0, 0.1], [0.2, 0.3, 1.0, 1.0], 2.0, [50, 0, 50, 50])), true);
    assert_eq!(changes.iter().any(|change| *change == RenderPrimitive::rectangle([65, 0], [35, 100], [1.0, 1.0, 1.0, 0.1], [0.2, 0.3, 1.0, 1.0], 2.0, [50, 0, 50, 50])), true);
    assert_eq!(changes.len(), 3);
}

#[test]
fn vertical_layout_for_none_sized_children() {
    let source = "
<root>
    <vertical-stack>
        <rect
            colour=(1.0, 1.0, 1.0, 0.1)
            stroke-colour=(0.2, 0.3, 1.0, 1.0)
            stroke-width=2
            corner-radii=(50, 0, 50, 50)
        />
        <rect
            colour=(1.0, 1.0, 1.0, 0.1)
            stroke-colour=(0.2, 0.3, 1.0, 1.0)
            stroke-width=2
            corner-radii=(50, 0, 50, 50)
        />
    </vertical-stack>
</root>
";
    let mut runner = Application::new()
        .with_builders(&mut test_builders(Dimensions::new(100, 100)))
        .build()
        .unwrap();

    apply_initial_source(runner.resources_mut(), ".\\root.zod", source);

    runner.run_once();

    let changes: Vec::<RenderPrimitive> = <&RenderPrimitive>::query()
        .iter(runner.world_mut())
        .map(|change| *change)
        .collect();
        
    assert_eq!(changes.iter().any(|change| *change == RenderPrimitive::rectangle([0, 0], [100, 50], [1.0, 1.0, 1.0, 0.1], [0.2, 0.3, 1.0, 1.0], 2.0, [50, 0, 50, 50])), true);
    assert_eq!(changes.iter().any(|change| *change == RenderPrimitive::rectangle([0, 50], [100, 50], [1.0, 1.0, 1.0, 0.1], [0.2, 0.3, 1.0, 1.0], 2.0, [50, 0, 50, 50])), true);
    assert_eq!(changes.len(), 2);
}

#[test]
fn vertical_layout_for_sized_children() {
    let source = "
<root>
    <vertical-stack>
        <rect
            height=25
            colour=(1.0, 1.0, 1.0, 0.1)
            stroke-colour=(0.2, 0.3, 1.0, 1.0)
            stroke-width=2
            corner-radii=(50, 0, 50, 50)
        />
        <rect
            colour=(1.0, 1.0, 1.0, 0.1)
            stroke-colour=(0.2, 0.3, 1.0, 1.0)
            stroke-width=2
            corner-radii=(50, 0, 50, 50)
        />
        <rect
            height=35
            colour=(1.0, 1.0, 1.0, 0.1)
            stroke-colour=(0.2, 0.3, 1.0, 1.0)
            stroke-width=2
            corner-radii=(50, 0, 50, 50)
        />
    </vertical-stack>
</root>
";
    let mut runner = Application::new()
        .with_builders(&mut test_builders(Dimensions::new(100, 100)))
        .build()
        .unwrap();

    apply_initial_source(runner.resources_mut(), ".\\root.zod", source);

    runner.run_once();

    let changes: Vec::<RenderPrimitive> = <&RenderPrimitive>::query()
        .iter(runner.world_mut())
        .map(|change| *change)
        .collect();
    
    assert_eq!(changes.iter().any(|change| *change == RenderPrimitive::rectangle([0, 0], [100, 25], [1.0, 1.0, 1.0, 0.1], [0.2, 0.3, 1.0, 1.0], 2.0, [50, 0, 50, 50])), true);
    assert_eq!(changes.iter().any(|change| *change == RenderPrimitive::rectangle([0, 25], [100, 40], [1.0, 1.0, 1.0, 0.1], [0.2, 0.3, 1.0, 1.0], 2.0, [50, 0, 50, 50])), true);
    assert_eq!(changes.iter().any(|change| *change == RenderPrimitive::rectangle([0, 65], [100, 35], [1.0, 1.0, 1.0, 0.1], [0.2, 0.3, 1.0, 1.0], 2.0, [50, 0, 50, 50])), true);
    assert_eq!(changes.len(), 3);
}