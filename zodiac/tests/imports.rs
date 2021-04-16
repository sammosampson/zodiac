use legion::*;
use zodiac_rendering_glium::*;
use zodiac::test_helpers::*;

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
#[ignore]
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
    assert_eq!(changes[0], RenderPrimitive::circle([0, 0], 400, [0.4, 0.4, 0.4, 0.1], [1.0, 1.0, 1.0, 1.0], 5.0));
}
