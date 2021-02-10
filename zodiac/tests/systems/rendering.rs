use rand::*;
use legion::*;
use zodiac::systems::rendering::*;
use zodiac_rendering::rendering::*;
use zodiac_entities::components::*;

#[derive(PartialEq)]
enum TestRendererPrimitive {
    Rectangle(usize, [u16; 2], [u16; 2], [f32; 4], [f32; 4], f32, [f32; 4]),
    Circle(usize, [u16; 2], u16, [f32; 4], [f32; 4], f32),
    Text(usize, [u16; 2], [u16; 2], [f32; 4], u16),
}

struct TestRenderer {
    queued_primitives: Vec::<TestRendererPrimitive>,
    rendered: bool
}

impl TestRenderer {
    fn new() -> Self {
        Self {
            queued_primitives: vec!(),
            rendered: false
        }
    }

    fn rectangle_was_queued(
        &self,
        index: usize,
        position: &Position,
        dimensions: &Dimensions, 
        colour: &Colour, 
        stroke_colour: &StrokeColour,
        stroke_width: &StrokeWidth, 
        corner_radii: &CornerRadii) -> bool {
        self.was_queued(
            TestRendererPrimitive::Rectangle(
                index,
                [position.x, position.y],
                [dimensions.x, dimensions.y], 
                [colour.r, colour.g, colour.b, colour.a],
                [stroke_colour.r, stroke_colour.g, stroke_colour.b, stroke_colour.a],
                stroke_width.width as f32,
                [corner_radii.left_top, corner_radii.right_top, corner_radii.right_bottom, corner_radii.left_bottom],
            )
        )
    }
    
    fn circle_was_queued(
        &self,
        index: usize,
        position: &Position,
        radius: &Radius, 
        colour: &Colour, 
        stroke_colour: &StrokeColour,
        stroke_width: &StrokeWidth) -> bool {
        self.was_queued(
            TestRendererPrimitive::Circle(
                index,
                [position.x, position.y],
                radius.radius, 
                [colour.r, colour.g, colour.b, colour.a],
                [stroke_colour.r, stroke_colour.g, stroke_colour.b, stroke_colour.a],
                stroke_width.width as f32,
            )
        )
    }

    fn text_was_queued(
        &self,
        index: usize,
        position: &Position,
        dimensions: &Dimensions, 
        colour: &Colour, 
        glyph_index: &GlyphIndex) -> bool {
        self.was_queued(
            TestRendererPrimitive::Text(
                index,
                [position.x, position.y],
                [dimensions.x, dimensions.y], 
                [colour.r, colour.g, colour.b, colour.a],
                glyph_index.index
            )
        )
    }

    fn was_queued(&self, to_check: TestRendererPrimitive) -> bool {
        self.queued_primitives.contains(&to_check)
    }
}

impl Renderer for TestRenderer {
    fn queue_rectangle_for_render(
        &mut self,
        index: usize,
        position: [u16; 2],
        dimensions: [u16; 2],
        inner_colour: [f32; 4],
        outer_colour: [f32; 4],
        stroke_width: f32,
        corner_radii: [f32; 4]) {
            self.queued_primitives.push(TestRendererPrimitive::Rectangle(
                index,
                position,
                dimensions,
                inner_colour,
                outer_colour,
                stroke_width,
                corner_radii
            ));
        }

    fn queue_circle_for_render(
        &mut self,
        index: usize,
        position: [u16; 2],
        radius: u16,
        inner_colour: [f32; 4],
        outer_colour: [f32; 4],
        stroke_width: f32) {
            self.queued_primitives.push(TestRendererPrimitive::Circle(
                index,
                position,
                radius,
                inner_colour,
                outer_colour,
                stroke_width
            ));
        }

    fn queue_text_for_render(
        &mut self,
        index: usize,
        position: [u16; 2],
        dimensions: [u16; 2],
        colour: [f32; 4],
        glyph_index: u16) {
            self.queued_primitives.push(TestRendererPrimitive::Text(
                index,
                position,
                dimensions,
                colour,
                glyph_index
            ));
        }

    fn render(&mut self) -> Result<(), RendererError> {
        self.rendered = true;
        Ok(())
    }
}

fn setup_world() -> World{
    World::default()
}

fn setup_resources() -> Resources {
    let mut resources = Resources::default();
    resources.insert(TestRenderer::new());
    resources
}

fn setup_schedule() -> Schedule {
    let schedule = Schedule::builder()
        .add_thread_local(render_primitives_system::<TestRenderer>())
        .build();
    schedule
}

fn create_dirty_component() -> Dirty {
    Dirty {}
}

fn create_rectangle_component() -> Rectangle {
    Rectangle {}
}

fn create_circle_component() -> Circle {
    Circle {}
}

fn create_text_component() -> Text {
    Text {}
}

fn create_radius_component() -> Radius {
    Radius { radius: random() }
}

fn create_glyph_index_component() -> GlyphIndex {
    GlyphIndex { index: random() }
}

fn create_position_component() -> Position {
    Position { x: random(), y: random() }
}

fn create_dimensions_component() -> Dimensions {
    Dimensions { x: random(), y: random() }
}

fn create_colour_component() -> Colour {
    Colour { r: random(), g: random(), b: random(), a: random() }
}

fn create_stroke_colour_component() -> StrokeColour {
    StrokeColour { r: random(), g: random(), b: random(), a: random() }
}

fn create_stroke_width_component() -> StrokeWidth {
    StrokeWidth { width: random() }
}

fn create_corner_radii_component() -> CornerRadii {
    CornerRadii { left_top: random(), right_top: random(), right_bottom: random(), left_bottom: random() }
}

#[test]
fn rectangles_marked_dirty_get_rendered() {
    let mut world = setup_world();
    let mut resources = setup_resources();
    let mut schedule = setup_schedule();

    let dirty1 = create_dirty_component();
    let dirty2 = create_dirty_component();
    let rectangle1 = create_rectangle_component();
    let rectangle2 = create_rectangle_component();
    let position1 = create_position_component();
    let position2 = create_position_component();
    let dimensions1 = create_dimensions_component();
    let dimensions2 = create_dimensions_component();
    let colour1 = create_colour_component();
    let colour2 = create_colour_component();
    let stroke_colour1 = create_stroke_colour_component();
    let stroke_colour2 = create_stroke_colour_component();
    let stroke_width1 = create_stroke_width_component();
    let stroke_width2 = create_stroke_width_component();
    let corner_radii1 = create_corner_radii_component();
    let corner_radii2 = create_corner_radii_component();

    world.push((dirty1, rectangle1, position1, dimensions1, colour1, stroke_colour1, stroke_width1, corner_radii1));
    world.push((dirty2, rectangle2, position2, dimensions2, colour2, stroke_colour2, stroke_width2, corner_radii2));
    schedule.execute(&mut world, &mut resources);

    let test_renderer = resources.get::<TestRenderer>().unwrap();

    assert_eq!(
        test_renderer.rectangle_was_queued(0, &position1, &dimensions1, &colour1, &stroke_colour1, &stroke_width1, &corner_radii1),
        true);

    assert_eq!(
        test_renderer.rectangle_was_queued(1, &position2, &dimensions2, &colour2, &stroke_colour2, &stroke_width2, &corner_radii2),
        true);

    assert_eq!(test_renderer.rendered, true); 
}


#[test]
fn rectangles_not_marked_dirty_do_not_get_rendered() {
    let mut world = setup_world();
    let mut resources = setup_resources();
    let mut schedule = setup_schedule();

    let rectangle1 = create_rectangle_component();
    let rectangle2 = create_rectangle_component();
    let position1 = create_position_component();
    let position2 = create_position_component();
    let dimensions1 = create_dimensions_component();
    let dimensions2 = create_dimensions_component();
    let colour1 = create_colour_component();
    let colour2 = create_colour_component();
    let stroke_colour1 = create_stroke_colour_component();
    let stroke_colour2 = create_stroke_colour_component();
    let stroke_width1 = create_stroke_width_component();
    let stroke_width2 = create_stroke_width_component();
    let corner_radii1 = create_corner_radii_component();
    let corner_radii2 = create_corner_radii_component();

    world.push((rectangle1, position1, dimensions1, colour1, stroke_colour1, stroke_width1, corner_radii1));
    world.push((rectangle2, position2, dimensions2, colour2, stroke_colour2, stroke_width2, corner_radii2));
    schedule.execute(&mut world, &mut resources);

    let test_renderer = resources.get::<TestRenderer>().unwrap();
    assert_eq!(test_renderer.queued_primitives.len(), 0); 
    assert_eq!(test_renderer.rendered, false); 
}

#[test]
fn circles_marked_dirty_get_rendered() {
    let mut world = setup_world();
    let mut resources = setup_resources();
    let mut schedule = setup_schedule();

    let dirty1 = create_dirty_component();
    let dirty2 = create_dirty_component();
    let circle1 = create_circle_component();
    let circle2 = create_circle_component();
    let position1 = create_position_component();
    let position2 = create_position_component();
    let radius1 = create_radius_component();
    let radius2 = create_radius_component();
    let colour1 = create_colour_component();
    let colour2 = create_colour_component();
    let stroke_colour1 = create_stroke_colour_component();
    let stroke_colour2 = create_stroke_colour_component();
    let stroke_width1 = create_stroke_width_component();
    let stroke_width2 = create_stroke_width_component();

    world.push((dirty1, circle1, position1, radius1, colour1, stroke_colour1, stroke_width1));
    world.push((dirty2, circle2, position2, radius2, colour2, stroke_colour2, stroke_width2));
    schedule.execute(&mut world, &mut resources);

    let test_renderer = resources.get::<TestRenderer>().unwrap();

    assert_eq!(
        test_renderer.circle_was_queued(0, &position1, &radius1, &colour1, &stroke_colour1, &stroke_width1),
        true);

    assert_eq!(
        test_renderer.circle_was_queued(1, &position2, &radius2, &colour2, &stroke_colour2, &stroke_width2),
        true);

    assert_eq!(test_renderer.rendered, true); 
}

#[test]
fn circles_not_marked_dirty_do_not_get_rendered() {
    let mut world = setup_world();
    let mut resources = setup_resources();
    let mut schedule = setup_schedule();

    let circle1 = create_circle_component();
    let circle2 = create_circle_component();
    let position1 = create_position_component();
    let position2 = create_position_component();
    let radius1 = create_radius_component();
    let radius2 = create_radius_component();
    let colour1 = create_colour_component();
    let colour2 = create_colour_component();
    let stroke_colour1 = create_stroke_colour_component();
    let stroke_colour2 = create_stroke_colour_component();
    let stroke_width1 = create_stroke_width_component();
    let stroke_width2 = create_stroke_width_component();

    world.push((circle1, position1, radius1, colour1, stroke_colour1, stroke_width1));
    world.push((circle2, position2, radius2, colour2, stroke_colour2, stroke_width2));
    schedule.execute(&mut world, &mut resources);

    let test_renderer = resources.get::<TestRenderer>().unwrap();
    assert_eq!(test_renderer.queued_primitives.len(), 0);
    assert_eq!(test_renderer.rendered, false); 
}

#[test]
fn texts_marked_dirty_get_rendered() {
    let mut world = setup_world();
    let mut resources = setup_resources();
    let mut schedule = setup_schedule();

    let dirty1 = create_dirty_component();
    let dirty2 = create_dirty_component();
    let text1 = create_text_component();
    let text2 = create_text_component();
    let position1 = create_position_component();
    let position2 = create_position_component();
    let dimensions1 = create_dimensions_component();
    let dimensions2 = create_dimensions_component();
    let colour1 = create_colour_component();
    let colour2 = create_colour_component();
    let glyph_index1 = create_glyph_index_component();
    let glyph_index2 = create_glyph_index_component();

    world.push((dirty1, text1, position1, dimensions1, colour1, glyph_index1));
    world.push((dirty2, text2, position2, dimensions2, colour2, glyph_index2));
    schedule.execute(&mut world, &mut resources);

    let test_renderer = resources.get::<TestRenderer>().unwrap();

    assert_eq!(
        test_renderer.text_was_queued(0, &position1, &dimensions1, &colour1, &glyph_index1),
        true);

    assert_eq!(
        test_renderer.text_was_queued(1, &position2, &dimensions2, &colour2, &glyph_index2),
        true);

    assert_eq!(test_renderer.rendered, true); 
}


#[test]
fn texts_not_marked_dirty_do_not_get_rendered() {
    let mut world = setup_world();
    let mut resources = setup_resources();
    let mut schedule = setup_schedule();

    let text1 = create_text_component();
    let text2 = create_text_component();
    let position1 = create_position_component();
    let position2 = create_position_component();
    let dimensions1 = create_dimensions_component();
    let dimensions2 = create_dimensions_component();
    let colour1 = create_colour_component();
    let colour2 = create_colour_component();
    let glyph_index1 = create_glyph_index_component();
    let glyph_index2 = create_glyph_index_component();

    world.push((text1, position1, dimensions1, colour1, glyph_index1));
    world.push((text2, position2, dimensions2, colour2, glyph_index2));
    schedule.execute(&mut world, &mut resources);

    let test_renderer = resources.get::<TestRenderer>().unwrap();
    assert_eq!(test_renderer.queued_primitives.len(), 0);
    assert_eq!(test_renderer.rendered, false); 
}

#[test]
fn differing_primitives_marked_dirty_get_rendered() {
    let mut world = setup_world();
    let mut resources = setup_resources();
    let mut schedule = setup_schedule();

    let dirty1 = create_dirty_component();
    let dirty2 = create_dirty_component();
    let dirty3 = create_dirty_component();
    let rectangle = create_rectangle_component();
    let circle = create_circle_component();
    let text = create_text_component();
    let position1 = create_position_component();
    let position2 = create_position_component();
    let position3 = create_position_component();
    let dimensions1 = create_dimensions_component();
    let dimensions2 = create_dimensions_component();
    let radius = create_radius_component();
    let colour1 = create_colour_component();
    let colour2 = create_colour_component();
    let colour3 = create_colour_component();
    let stroke_colour1 = create_stroke_colour_component();
    let stroke_colour2 = create_stroke_colour_component();
    let stroke_width1 = create_stroke_width_component();
    let stroke_width2 = create_stroke_width_component();
    let glyph_index = create_glyph_index_component();
    let corner_radii = create_corner_radii_component();

    world.push((dirty1, rectangle, position1, dimensions1, colour1, stroke_colour1, stroke_width1, corner_radii));
    world.push((dirty2, circle, position2, radius, colour2, stroke_colour2, stroke_width2));
    world.push((dirty3, text, position3, dimensions2, colour3, glyph_index));
    schedule.execute(&mut world, &mut resources);

    let test_renderer = resources.get::<TestRenderer>().unwrap();

    assert_eq!(
        test_renderer.rectangle_was_queued(0, &position1, &dimensions1, &colour1, &stroke_colour1, &stroke_width1, &corner_radii),
        true);

    assert_eq!(
        test_renderer.circle_was_queued(1, &position2, &radius, &colour2, &stroke_colour2, &stroke_width2),
        true);

    assert_eq!(
        test_renderer.text_was_queued(2, &position3, &dimensions2, &colour3, &glyph_index),
        true);

    assert_eq!(test_renderer.rendered, true); 
}