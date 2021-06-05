use legion::*;
use zodiac::testing::*;
use zodiac_entities::*;
use zodiac::*;
//use zodiac::formatting::*;

#[test]
fn imported_control_gets_output() {
    let big_control = "
<control>
    <circle
        radius=400
        colour=(100, 100, 100, 25)
        stroke-colour=(255, 255, 255, 255)
        stroke-width=3
    />
</control>
";
    let root = "
<import name=\"big-control\" path=\".\\big_control.zod\" />     
<root>
    <canvas
        left=10
        top=11
    >
        <big-control />
        <big-control />
    </canvas>
</root>
";
    
    let mut runner = Application::new()
        .with_builders(&mut test_builders(Dimensions::new(100, 100)))
        .build()
        .unwrap();
    
    apply_initial_source(runner.resources_mut(), ".\\big_control.zod", big_control);
    apply_initial_source(runner.resources_mut(), ".\\root.zod", root);
    
    runner.run_once();

    let changes: Vec::<RenderPrimitive> = <&RenderPrimitive>::query()
        .iter(runner.world_mut())
        .map(|change| change.clone())
        .collect();
    
    assert_eq!(changes.len(), 2);
    assert_eq!(changes[0], RenderPrimitive::circle([10, 11], 400, [100, 100, 100, 25], [255, 255, 255, 255], 3));
    assert_eq!(changes[1], RenderPrimitive::circle([10, 11], 400, [100, 100, 100, 25], [255, 255, 255, 255], 3));
}

#[test]
fn changed_imported_control_gets_output() {
    let big_control = "
<control>
    <circle
        radius=400
        colour=(100, 100, 100, 25)
        stroke-colour=(255, 255, 255, 255)
        stroke-width=3
    />
</control>
";
    let root = "
<import name=\"big-control\" path=\".\\big_control.zod\" />     
<root>
    <canvas
        left=10
        top=11
    >
        <big-control />
    </canvas>
</root>
";

    let changed_big_control = "
<control>
    <circle
        radius=400
        colour=(100, 100, 100, 25)
        stroke-colour=(255, 255, 255, 255)
        stroke-width=5
    />
</control>
";  
    let mut runner = Application::new()
        .with_builders(&mut test_builders(Dimensions::new(1024, 768)))
        .build()
        .unwrap();
    
    apply_initial_source(runner.resources_mut(), ".\\big_control.zod", big_control);
    apply_initial_source(runner.resources_mut(), ".\\root.zod", root);

    runner.run_once();

    apply_changed_source(runner.resources_mut(), ".\\big_control.zod", changed_big_control);

    runner.run_once();
    
    let changes: Vec::<RenderPrimitive> = <&RenderPrimitive>::query()
        .iter(runner.world_mut())
        .map(|change| change.clone())
        .collect();
    
    assert_eq!(changes.len(), 1);
    assert_eq!(changes[0], RenderPrimitive::circle([10, 11], 400, [100, 100, 100, 25], [255, 255, 255, 255], 5));
}

#[test]
fn created_then_imported_control_gets_output() {
    let new_control = "
<control>
    <circle
        radius=400
        colour=(100, 100, 100, 25)
        stroke-colour=(255, 255, 255, 255)
        stroke-width=3
    />
</control>
";
    let root = "
<root>
</root>
";

    let changed_root = "
<import name=\"big-control\" path=\".\\big_control.zod\" />     
<root>
    <big-control/>
</root>    
";  
    let mut runner = Application::new()
        .with_builders(&mut test_builders(Dimensions::new(1024, 768)))
        .build()
        .unwrap();
    
    apply_initial_source(runner.resources_mut(), ".\\root.zod", root);
    
    runner.run_once();
    
    apply_created_source(runner.resources_mut(), ".\\big_control.zod", new_control);
    runner.run_once();
    
    apply_changed_source(runner.resources_mut(), ".\\root.zod", changed_root);
    runner.run_once();
    
    let changes: Vec::<RenderPrimitive> = <&RenderPrimitive>::query()
        .iter(runner.world_mut())
        .map(|change| change.clone())
        .collect();
    
    assert_eq!(changes.len(), 1);
    assert_eq!(changes[0], RenderPrimitive::circle([0, 0], 400, [100, 100, 100, 25], [255, 255, 255, 255], 3));
}

#[test]
fn nonexistent_imported_then_created_control_gets_output() {
    let new_control = "
<control>
    <circle
        radius=400
        colour=(100, 100, 100, 25)
        stroke-colour=(255, 255, 255, 255)
        stroke-width=3
    />
</control>
";
    let root = "
<root>
</root>
";

    let changed_root = "
<import name=\"big-control\" path=\".\\big_control.zod\" />     
<root>
    <big-control/>
</root>    
";  
    let mut runner = Application::new()
        .with_builders(&mut test_builders(Dimensions::new(1024, 768)))
        .build()
        .unwrap();
    
    apply_initial_source(runner.resources_mut(), ".\\root.zod", root);
    
    runner.run_once();
    
    apply_changed_source(runner.resources_mut(), ".\\root.zod", changed_root);
    runner.run_once();

    apply_created_source(runner.resources_mut(), ".\\big_control.zod", new_control);
    runner.run_once();


    let changes: Vec::<RenderPrimitive> = <&RenderPrimitive>::query()
        .iter(runner.world_mut())
        .map(|change| change.clone())
        .collect();
    
    assert_eq!(changes.len(), 1);
    assert_eq!(changes[0], RenderPrimitive::circle([0, 0], 400, [100, 100, 100, 25], [255, 255, 255, 255], 3));
}

#[test]
fn imported_then_deleted_then_recreated_control_gets_output() {
    let new_control = "
<control>
    <circle
        radius=400
        colour=(100, 100, 100, 25)
        stroke-colour=(255, 255, 255, 255)
        stroke-width=3
    />
</control>
";
    let root = "
<import name=\"big-control\" path=\".\\big_control.zod\" />     
<root>
    <big-control/>
</root>
";  
    let mut runner = Application::new()
        .with_builders(&mut test_builders(Dimensions::new(1024, 768)))
        .build()
        .unwrap();
    
    apply_initial_source(runner.resources_mut(), ".\\big_control.zod", new_control);
    apply_initial_source(runner.resources_mut(), ".\\root.zod", root);
    
    runner.run_once();
    
    delete_source(runner.resources_mut(), ".\\big_control.zod");
    runner.run_once();
    
    apply_created_source(runner.resources_mut(), ".\\big_control.zod", new_control);
    runner.run_once();
    
    let changes: Vec::<RenderPrimitive> = <&RenderPrimitive>::query()
        .iter(runner.world_mut())
        .map(|change| change.clone())
        .collect();
    
    assert_eq!(changes.len(), 1);
    assert_eq!(changes[0], RenderPrimitive::circle([0, 0], 400, [100, 100, 100, 25], [255, 255, 255, 255], 3));
}