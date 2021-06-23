use mox::mox;
use zodiac_html::*;
use zodiac_html::testing::*;
use zodiac::*;

#[topo::nested]
pub fn block_layout_with_children_with_margin_style() -> Node {
    mox!(
        <style margin=px(5).into() />
    )
}

#[topo::nested]
fn block_layout_with_children_with_margin_root() -> RootNode<TestState> {
    mox!(
        <root>
            <div>
                <span style=block_layout_with_children_with_margin_style()/>
                <span style=block_layout_with_children_with_margin_style()/>
            </div>    
        </root>
    )
}

#[test]
fn div_performs_block_layout() {
    let changes = test_app(block_layout_with_children_with_margin_root)
        .build()
        .run_once()
        .get_changes();

    assert_eq!(changes.len(), 2);
    assert_eq!(changes[0].is_positioned_at(0, 0), true);
    assert_eq!(changes[0].has_dimensions_of(10, 10), true);
    assert_eq!(changes[1].is_positioned_at(10, 0), true);
    assert_eq!(changes[0].has_dimensions_of(10, 10), true);
}