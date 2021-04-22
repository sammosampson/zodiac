use legion::*;
use zodiac_rendering_glium::*;
use zodiac::test_helpers::*;
use zodiac::formatting::*;
use zodiac_entities::*;

#[test]
fn invalid_source_causes_top_level_error_circle_renderable_control_output() {
    let root = "
<root>
    <circle
        radius=
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
    let mut world = World::default();
    let mut resources = build_zodiac_resources();
    let mut schedule = build_zodiac_systems_schedule();
    
    apply_initial_source(&mut resources, ".\\root.zod", root);
    notify_resize_root_window(&mut world, (100, 100));
    schedule.execute(&mut world, &mut resources);

    
    world.to_pretty();

    let changes: Vec::<RenderPrimitive> = <&RenderPrimitive>::query()
        .iter(&mut world)
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
    let mut world = World::default();
    let mut resources = build_zodiac_resources();
    let mut schedule = build_zodiac_systems_schedule();
    
    apply_initial_source(&mut resources, ".\\root.zod", root);
    notify_resize_root_window(&mut world, (100, 100));
    schedule.execute(&mut world, &mut resources);
        
    world.to_pretty();
        
    let changes: Vec::<RenderPrimitive> = <&RenderPrimitive>::query()
        .iter(&mut world)
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
    let mut world = World::default();
    let mut resources = build_zodiac_resources();
    let mut schedule = build_zodiac_systems_schedule();
    
    apply_initial_source(&mut resources, ".\\root.zod", root);
    notify_resize_root_window(&mut world, (100, 100));
    
    schedule.execute(&mut world, &mut resources);    
    schedule.execute(&mut world, &mut resources);

    let changes: Vec::<Renderable> = <&Renderable>::query()
        .iter(&mut world)
        .map(|change| *change)
        .collect();
    
    assert_eq!(changes.len(), 1);
}
