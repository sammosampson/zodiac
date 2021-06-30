use mox::mox;
use zodiac_html::*;
use zodiac_html::testing::*;
use zodiac::*;

#[topo::nested]
pub fn block_layout_with_children_with_margin_style() -> Node {
    mox!(
        <style display=DisplayTypes::Block margin=px(5).into() />
    )
}

#[topo::nested]
fn block_layout_with_children_with_margin_root() -> RootNode<TestState> {
    mox!(
        <root>
            <div style=block_layout_with_children_with_margin_style() />
            <div style=block_layout_with_children_with_margin_style() />
        </root>
    )
}

#[test]
fn div_performs_block_layout() {

    let changes = test_app(block_layout_with_children_with_margin_root)
        .with_screen_dimensions(1024, 768)
        .build()
        .run_once()
        .get_changes();

    assert_eq!(changes.len(), 2);
    assert_eq!(changes[0].is_positioned_at(0, 0), true, "changes: {:?}", changes[0]);
    assert_eq!(changes[0].has_dimensions_of(1024, 10), true, "changes: {:?}", changes[0]);
    assert_eq!(changes[1].is_positioned_at(0, 10), true, "changes: {:?}", changes[1]);
    assert_eq!(changes[1].has_dimensions_of(1024, 10), true, "changes: {:?}", changes[1]);
}