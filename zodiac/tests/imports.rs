use legion::*;
use zodiac_rendering_glium::*;
use zodiac::test_helpers::*;
use zodiac_parsing::*;

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
    
    let mut source_lookup = SourceCodeLookup::new();
    source_lookup.insert(SourceLocation::from(".\\big_control.zod"), String::from(big_control));
    source_lookup.insert(SourceLocation::from(".\\root.zod"), String::from(root));

    let mut world = World::default();
    let mut resources = build_zodiac_resources(source_lookup);
    let mut schedule = build_zodiac_systems_schedule();
    
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
