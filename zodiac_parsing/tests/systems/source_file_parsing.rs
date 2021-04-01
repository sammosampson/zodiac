use legion::*;
use zodiac_entities::*;
use zodiac_parsing::*;

#[test]
fn parse_horizontal_layout_container_produces_container_components_on_entity() {
    let source = "<horizontal-stack />";
    let mut world = World::default();

    run_source_file_parse_system(source, &mut world);

        let horizontal_contents = <&LayoutContent>::query()
            .iter(&mut world)
            .filter(|layout|layout.layout_type == LayoutType::Horizontal)
            .count();

        assert_eq!(horizontal_contents, 1);

}

#[test]
fn parse_canvas_layout_container_produces_container_components_on_entity() {
    let source = "<root><canvas /></root>";
    let mut world = World::default();
    
    run_source_file_parse_system(source, &mut world);

    let canvases = <&LayoutContent>::query()
        .iter(&mut world)
        .filter(|layout|layout.layout_type == LayoutType::Canvas)
        .count();

        assert_eq!(canvases, 2);
}

#[test]
fn parse_circle_produces_circle_components_on_entity() {
    let source = "<circle
        left=200
        top=100
        radius=50
        colour=(1.0, 2.0, 3.0, 4.0)
        stroke-colour=(0.0, 1.0, 2.0, 3.0)
        stroke-width=3
    />";
    
    let mut world = World::default();

    run_source_file_parse_system(source, &mut world);
    
    let circles = <(&Left, &Top, &Radius, &Colour, &StrokeColour, &StrokeWidth)>::query()
        .filter(component::<Renderable>())
        .iter(&mut world)
        .filter(|(left, top, radius, colour, stroke_colour, stroke_width)| {
            left.left == 200 
            && top.top == 100
            && radius.radius == 50
            && colour.r == 1.0
            && colour.g == 2.0
            && colour.b == 3.0
            && colour.a == 4.0
            && stroke_colour.r == 0.0
            && stroke_colour.g == 1.0
            && stroke_colour.b == 2.0
            && stroke_colour.a == 3.0
            && stroke_width.width == 3
        })
        .count();

    assert_eq!(circles, 1);
}

#[test]
fn parse_rect_produces_rectangle_components_on_entity() {
    let source =  "<rect
        left=200
        top=100
        height=200
        width=300
        colour=(1.0, 2.0, 3.0, 4.0)
        stroke-colour=(0.0, 1.0, 2.0, 3.0)
        stroke-width=3
        corner-radii=(5, 15, 25, 35)
    />";

    let mut world = World::default();
    
    run_source_file_parse_system(source, &mut world);
    
    let rectangles = <(&Top, &Width, &Height, &Colour, &StrokeColour, &StrokeWidth, &CornerRadii)>::query()
        .filter(component::<Renderable>())
        .iter(&mut world)
        .filter(|(top, width, height, colour, stroke_colour, stroke_width, corner_radii)| {
             top.top == 100
            && width.width == 300
            && height.height == 200
            && colour.r == 1.0
            && colour.g == 2.0
            && colour.b == 3.0
            && colour.a == 4.0
            && stroke_colour.r == 0.0
            && stroke_colour.g == 1.0
            && stroke_colour.b == 2.0
            && stroke_colour.a == 3.0
            && stroke_width.width == 3
            && corner_radii.left_top == 5
            && corner_radii.right_top == 15
            && corner_radii.right_bottom == 25
            && corner_radii.left_bottom == 35
        })
        .count();
    
    assert_eq!(rectangles, 1);
}

#[test]
fn parse_text_produces_text_components_on_entity() {
    let source = "<text
        left=200
        top=100
        height=200
        width=300
        colour=(1.0, 2.0, 3.0, 4.0)
        content=\"test\"
    />";
    
    let mut world = World::default();
    
    run_source_file_parse_system(source, &mut world);
        
    let text = <(&LayoutContent, &Left, &Top, &Height, &Width, &Colour)>::query()
        .iter(&mut world)
        .filter(|(layout, left, top, height, width, colour)| {
            layout.layout_type == LayoutType::Canvas
            && left.left == 200 
            && top.top == 100 
            && height.height == 200 
            && width.width == 300 
            && colour.r == 1.0 
            && colour.g == 2.0 
            && colour.b == 3.0 
            && colour.a == 4.0 
        })
        .count();
        
    assert_eq!(text, 1);

    let mut characters = 0;
    let mut query = <(&Renderable, &Character)>::query();

    for (renderable, character) in query.iter(&mut world) {
        assert_eq!(renderable.render_type, RenderType::Glyph);
        assert_eq!(character.character, "test".chars().nth(characters).unwrap());
        assert_eq!(character.position, characters);
        characters += 1;
    }
    assert_eq!(characters, 4);
}

#[test]
fn parse_multiple_controls_produces_entities() {
    let source = "<circle top=200 /><rect top=200 /><text top=200 />";
    let mut world = World::default();
    
    run_source_file_parse_system(source, &mut world);
            
    let entity_count = <&Top>::query()
        .iter(&mut world)
        .count();
        
    assert_eq!(entity_count, 3);
}

#[test]
fn parse_hierarchical_controls_produces_relationships() {
    let source = "<root><horizontal-stack><rect top=200 /></horizontal-stack></root>";
    let mut world = World::default();
    
    run_source_file_parse_system(source, &mut world);
    
    let root = <(&LayoutContent, &Relationship)>::query()
        .filter(component::<Root>())
        .iter(&mut world)
        .filter(|(layout, relationship)| {
            layout.layout_type == LayoutType::Canvas
            && relationship.parent == None
            && relationship.next_sibling == None
            && relationship.first_child != None
            && relationship.last_child != None 
        })
        .count();

    assert_eq!(root, 1);

    let stack = <(&LayoutContent, &Relationship)>::query()
        .iter(&mut world)
        .filter(|(layout, relationship)| {
            layout.layout_type == LayoutType::Horizontal
            && relationship.parent != None
            && relationship.next_sibling == None
            && relationship.first_child != None
            && relationship.last_child != None 
        })
        .count();

    assert_eq!(stack, 1);

    let rect = <&Relationship>::query()
        .filter(component::<Renderable>())
        .iter(&mut world)
        .filter(|relationship| {
            relationship.parent != None
            && relationship.next_sibling == None
            && relationship.first_child == None
            && relationship.last_child == None 
        })
        .count();

    assert_eq!(rect, 1);
}

#[test]
fn parse_malformed_property_produces_error() {
    let source = "<circle width=100 colour=(1.0, 10x) /><rect Left=200 /><text Left=200/>";
    let mut world = World::default();
    
    run_source_file_parse_system(source, &mut world);
    
    //TODO: test this when its possible
    //assert_eq!(result, Err(AbstractSyntaxTokenError::BadColourValue));
    
    let entity_count = <&Width>::query()
        .iter(&mut world)
        .count();
        
    assert_eq!(entity_count, 1);
}

fn run_source_file_parse_system(source: &str, world: &mut World){
    let mut resources = Resources::default();

    let mut schedule = Schedule::builder()
        .add_system(source_file_parse_system::<TestSourceReader>())
        .build();

    let source_entity = world.push((
        SourceFile::default(),
        Root::default(),
        Relationship::default(),
        LayoutContent::canvas()));

    let source_location = "test";

    resources.insert(create_test_source_reader(source, source_location));
    resources.insert(create_test_source_location_lookup(source_entity, source_location));

    schedule.execute(world, &mut resources);
}

fn create_test_source_reader(source: &str, source_location: &str) -> TestSourceReader {
    TestSourceReader {
        source: source.to_string(),
        source_location: SourceLocation::from(source_location)
    }   
}

fn create_test_source_location_lookup(entity: Entity, source_location: &str) -> SourceLocationLookup {
    let mut lookup = create_source_location_lookup();
    lookup.insert(entity, SourceLocation::from(source_location));
    lookup
}

pub struct TestSourceReader {
    source: String,
    pub source_location: SourceLocation
}

impl SourceReader for TestSourceReader {
    fn read_source_at_location(&self, location: &SourceLocation) -> Result<String, SourceReaderError> {
        if *location != self.source_location {
            return Err(SourceReaderError::SourceNotFound);
        }
        Ok(self.source.clone())
    }       
}