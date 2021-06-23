use mox::mox;
use zodiac_html::*;
use zodiac_html::testing::*;
use zodiac::*;

#[topo::nested]
pub fn element_border_style() -> Node {
    mox!(
        <style
            border=(px(5), BorderStyles::Dashed, rgb(50, 100, 200)).into()
        />
    )
}

#[topo::nested]
fn element_border_root() -> RootNode<TestState> {
    mox!(
        <root>
            <div style=element_border_style() />
        </root>
    )
}

#[test]
fn element_border() {
    let changes = test_app(element_border_root)
        .build()
        .run_once()
        .get_changes();

    assert_eq!(changes.len(), 1);
    assert_eq!(changes[0].has_border_top_of((px(5), BorderStyles::Dashed, rgb(50, 100, 200)).into()), true);
    assert_eq!(changes[0].has_border_bottom_of((px(5),BorderStyles::Dashed, rgb(50, 100, 200)).into()), true);
    assert_eq!(changes[0].has_border_left_of((px(5), BorderStyles::Dashed, rgb(50, 100, 200)).into()), true);
    assert_eq!(changes[0].has_border_right_of((px(5), BorderStyles::Dashed, rgb(50, 100, 200)).into()), true);
}

#[topo::nested]
pub fn element_border_props_style() -> Node {
    mox!(
        <style
            border_width=px(5)
            border_style=BorderStyles::Dashed
            border_colour=rgb(50, 100, 200)
            border_radius=px(50)
        />
    )
}

#[topo::nested]
fn element_border_props_root() -> RootNode<TestState> {
    mox!(
        <root>
            <div style=element_border_props_style() />
        </root>
    )
}

#[test]
fn element_border_props() {
    let changes = test_app(element_border_props_root)
        .build()
        .run_once()
        .get_changes();

    assert_eq!(changes.len(), 1);
    assert_eq!(changes[0].has_border_top_of((px(5), BorderStyles::Dashed, rgb(50, 100, 200)).into()), true);
    assert_eq!(changes[0].has_border_bottom_of((px(5),BorderStyles::Dashed, rgb(50, 100, 200)).into()), true);
    assert_eq!(changes[0].has_border_left_of((px(5), BorderStyles::Dashed, rgb(50, 100, 200)).into()), true);
    assert_eq!(changes[0].has_border_right_of((px(5), BorderStyles::Dashed, rgb(50, 100, 200)).into()), true);
    assert_eq!(changes[0].has_border_radius_of(px(50)), true);
}

#[topo::nested]
pub fn element_border_sides_style() -> Node {
    mox!(
        <style
            border_top=(px(5), BorderStyles::Dashed, rgb(50, 100, 200)).into()
            border_bottom=(px(6), BorderStyles::Solid, rgb(60, 110, 210)).into()
            border_left=(px(7), BorderStyles::Dotted, rgb(70, 120, 220)).into()
            border_right=(px(8), BorderStyles::Double, rgb(80, 130, 230)).into()
        />
    )
}

#[topo::nested]
fn element_border_sides_root() -> RootNode<TestState> {
    mox!(
        <root>
            <div style=element_border_sides_style() />
        </root>
    )
}

#[test]
fn element_border_sides() {
    let changes = test_app(element_border_sides_root)
        .build()
        .run_once()
        .get_changes();

    assert_eq!(changes.len(), 1);
    assert_eq!(changes[0].has_border_top_of((px(5), BorderStyles::Dashed, rgb(50, 100, 200)).into()), true);
    assert_eq!(changes[0].has_border_bottom_of((px(6),BorderStyles::Solid, rgb(60, 110, 210)).into()), true);
    assert_eq!(changes[0].has_border_left_of((px(7), BorderStyles::Dotted, rgb(70, 120, 220)).into()), true);
    assert_eq!(changes[0].has_border_right_of((px(8), BorderStyles::Double, rgb(80, 130, 230)).into()), true);
}

#[topo::nested]
pub fn element_border_sides_props_style() -> Node {
    mox!(
        <style
            border_top_width=px(5)
            border_top_style=BorderStyles::Dashed
            border_top_colour=rgb(50, 100, 200)
            border_bottom_width=px(6)
            border_bottom_style=BorderStyles::Solid
            border_bottom_colour=rgb(60, 110, 210)
            border_left_width=px(7)
            border_left_style=BorderStyles::Dotted
            border_left_colour=rgb(70, 120, 220)
            border_right_width=px(8)
            border_right_style=BorderStyles::Double
            border_right_colour=rgb(80, 130, 230)
        />
    )
}

#[topo::nested]
fn element_border_sides_props_root() -> RootNode<TestState> {
    mox!(
        <root>
            <div style=element_border_sides_props_style() />
        </root>
    )
}

#[test]
fn element_border_props_sides() {
    let changes = test_app(element_border_sides_props_root)
        .build()
        .run_once()
        .get_changes();

    assert_eq!(changes.len(), 1);
    assert_eq!(changes[0].has_border_top_of((px(5), BorderStyles::Dashed, rgb(50, 100, 200)).into()), true);
    assert_eq!(changes[0].has_border_bottom_of((px(6),BorderStyles::Solid, rgb(60, 110, 210)).into()), true);
    assert_eq!(changes[0].has_border_left_of((px(7), BorderStyles::Dotted, rgb(70, 120, 220)).into()), true);
    assert_eq!(changes[0].has_border_right_of((px(8), BorderStyles::Double, rgb(80, 130, 230)).into()), true);
}