use legion::*;
use zodiac_rendering_glium::*;
use zodiac::test_helpers::*;
//use zodiac::formatting::*;

#[test]
fn imported_control_gets_output() {
    let big_control = "
<control>
    <circle
        radius=400
        colour=(0.4, 0.4, 0.4, 0.1)
        stroke-colour=(1.0, 1.0, 1.0, 1.0)
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

    
    let mut world = World::default();
    let mut resources = build_zodiac_resources();
    let mut schedule = build_zodiac_systems_schedule();
    
    apply_initial_source(&mut resources, ".\\big_control.zod", big_control);
    apply_initial_source(&mut resources, ".\\root.zod", root);
    
    notify_resize_root_window(&mut world, (100, 100));

    schedule.execute(&mut world, &mut resources);

    let changes: Vec::<RenderPrimitive> = <&RenderPrimitive>::query()
        .iter(&mut world)
        .map(|change| *change)
        .collect();
    
    assert_eq!(changes.len(), 2);
    assert_eq!(changes[0], RenderPrimitive::circle([10, 11], 400, [0.4, 0.4, 0.4, 0.1], [1.0, 1.0, 1.0, 1.0], 3.0));
    assert_eq!(changes[1], RenderPrimitive::circle([10, 11], 400, [0.4, 0.4, 0.4, 0.1], [1.0, 1.0, 1.0, 1.0], 3.0));
}

#[test]
fn changed_imported_control_gets_output() {
    let big_control = "
<control>
    <circle
        radius=400
        colour=(0.4, 0.4, 0.4, 0.1)
        stroke-colour=(1.0, 1.0, 1.0, 1.0)
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
        colour=(0.4, 0.4, 0.4, 0.1)
        stroke-colour=(1.0, 1.0, 1.0, 1.0)
        stroke-width=5
    />
</control>
";  
    let mut world = World::default();
    let mut resources = build_zodiac_resources();
    let mut schedule = build_zodiac_systems_schedule();
    
    apply_initial_source(&mut resources, ".\\big_control.zod", big_control);
    apply_initial_source(&mut resources, ".\\root.zod", root);

    notify_resize_root_window(&mut world, (100, 100));

    schedule.execute(&mut world, &mut resources);

    apply_changed_source(&mut resources, ".\\big_control.zod", changed_big_control);

    schedule.execute(&mut world, &mut resources);
    
    let changes: Vec::<RenderPrimitive> = <&RenderPrimitive>::query()
        .iter(&mut world)
        .map(|change| *change)
        .collect();
    
    assert_eq!(changes.len(), 1);
    assert_eq!(changes[0], RenderPrimitive::circle([10, 11], 400, [0.4, 0.4, 0.4, 0.1], [1.0, 1.0, 1.0, 1.0], 5.0));
}

#[test]
fn created_then_imported_control_gets_output() {
    let new_control = "
<control>
    <circle
        radius=400
        colour=(0.4, 0.4, 0.4, 0.1)
        stroke-colour=(1.0, 1.0, 1.0, 1.0)
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
    let mut world = World::default();
    let mut resources = build_zodiac_resources();
    let mut schedule = build_zodiac_systems_schedule();
    
    apply_initial_source(&mut resources, ".\\root.zod", root);
    notify_resize_root_window(&mut world, (100, 100));
    schedule.execute(&mut world, &mut resources);
    
    apply_created_source(&mut resources, ".\\big_control.zod", new_control);
    schedule.execute(&mut world, &mut resources);
    
    apply_changed_source(&mut resources, ".\\root.zod", changed_root);
    schedule.execute(&mut world, &mut resources);
    
    let changes: Vec::<RenderPrimitive> = <&RenderPrimitive>::query()
        .iter(&mut world)
        .map(|change| *change)
        .collect();
    
    assert_eq!(changes.len(), 1);
    assert_eq!(changes[0], RenderPrimitive::circle([0, 0], 400, [0.4, 0.4, 0.4, 0.1], [1.0, 1.0, 1.0, 1.0], 3.0));
}

#[test]
fn nonexistent_imported_then_created_control_gets_output() {
    let new_control = "
<control>
    <circle
        radius=400
        colour=(0.4, 0.4, 0.4, 0.1)
        stroke-colour=(1.0, 1.0, 1.0, 1.0)
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
    let mut world = World::default();
    let mut resources = build_zodiac_resources();
    let mut schedule = build_zodiac_systems_schedule();
    
    apply_initial_source(&mut resources, ".\\root.zod", root);
    notify_resize_root_window(&mut world, (100, 100));
    schedule.execute(&mut world, &mut resources);
    
    apply_changed_source(&mut resources, ".\\root.zod", changed_root);
    schedule.execute(&mut world, &mut resources);
    
    apply_created_source(&mut resources, ".\\big_control.zod", new_control);
    schedule.execute(&mut world, &mut resources);
    
    let changes: Vec::<RenderPrimitive> = <&RenderPrimitive>::query()
        .iter(&mut world)
        .map(|change| *change)
        .collect();
    
    assert_eq!(changes.len(), 1);
    assert_eq!(changes[0], RenderPrimitive::circle([0, 0], 400, [0.4, 0.4, 0.4, 0.1], [1.0, 1.0, 1.0, 1.0], 3.0));
}

#[test]
fn imported_then_deleted_then_recreated_control_gets_output() {
    let new_control = "
<control>
    <circle
        radius=400
        colour=(0.4, 0.4, 0.4, 0.1)
        stroke-colour=(1.0, 1.0, 1.0, 1.0)
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
    let mut world = World::default();
    let mut resources = build_zodiac_resources();
    let mut schedule = build_zodiac_systems_schedule();
    
    apply_initial_source(&mut resources, ".\\big_control.zod", new_control);
    apply_initial_source(&mut resources, ".\\root.zod", root);
    notify_resize_root_window(&mut world, (100, 100));
    schedule.execute(&mut world, &mut resources);
    
    delete_source(&mut resources, ".\\big_control.zod");
    schedule.execute(&mut world, &mut resources);
    
    apply_created_source(&mut resources, ".\\big_control.zod", new_control);
    schedule.execute(&mut world, &mut resources);
    
    let changes: Vec::<RenderPrimitive> = <&RenderPrimitive>::query()
        .iter(&mut world)
        .map(|change| *change)
        .collect();
    
    assert_eq!(changes.len(), 1);
    assert_eq!(changes[0], RenderPrimitive::circle([0, 0], 400, [0.4, 0.4, 0.4, 0.1], [1.0, 1.0, 1.0, 1.0], 3.0));
}

#[test]
fn invalid_source_causes_top_level_error_control_output() {
    let root = "
<root>
    <circle
        radius=x
    />
</root>
";  
    let mut world = World::default();
    let mut resources = build_zodiac_resources();
    let mut schedule = build_zodiac_systems_schedule();
    
    apply_initial_source(&mut resources, ".\\root.zod", root);
    notify_resize_root_window(&mut world, (100, 100));
    schedule.execute(&mut world, &mut resources);
    
    let changes: Vec::<RenderPrimitive> = <&RenderPrimitive>::query()
        .iter(&mut world)
        .map(|change| *change)
        .collect();
    
    assert_eq!(changes.len(), 1);
    assert_eq!(changes[0], RenderPrimitive::rectangle([0, 0], [100, 100], [1.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0], 0.0, [0, 0, 0, 0]));
}

#[test]
fn invalid_import_causes_top_level_error_control_output() {
    let root = "
<import name=\"big-control\" path=\".\\big_control.zod\" />     
<root>
    <big-control/>
</root>
";  
    let mut world = World::default();
    let mut resources = build_zodiac_resources();
    let mut schedule = build_zodiac_systems_schedule();
    
    apply_initial_source(&mut resources, ".\\root.zod", root);
    notify_resize_root_window(&mut world, (100, 100));
    schedule.execute(&mut world, &mut resources);
        
    let changes: Vec::<RenderPrimitive> = <&RenderPrimitive>::query()
        .iter(&mut world)
        .map(|change| *change)
        .collect();
    
    assert_eq!(changes.len(), 1);
    assert_eq!(changes[0], RenderPrimitive::rectangle([0, 0], [100, 100], [1.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0], 0.0, [0, 0, 0, 0]));
}

#[test]
fn invalid_control_causes_top_level_error_control_output() {
    let root = "
<root>
    <big-control/>
</root>
";  
    let mut world = World::default();
    let mut resources = build_zodiac_resources();
    let mut schedule = build_zodiac_systems_schedule();
    
    apply_initial_source(&mut resources, ".\\root.zod", root);
    notify_resize_root_window(&mut world, (100, 100));
    schedule.execute(&mut world, &mut resources);
        
    let changes: Vec::<RenderPrimitive> = <&RenderPrimitive>::query()
        .iter(&mut world)
        .map(|change| *change)
        .collect();
    
    assert_eq!(changes.len(), 1);
    assert_eq!(changes[0], RenderPrimitive::rectangle([0, 0], [100, 100], [1.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0], 0.0, [0, 0, 0, 0]));
}