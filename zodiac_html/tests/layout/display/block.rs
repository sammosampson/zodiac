use mox::mox;
use zodiac_html::*;
use zodiac_html::testing::*;
use zodiac::*;

#[topo::nested]
pub fn block_layout_with_margin_border_border_and_padding_style() -> Node {
    mox!(
        <style
            display=DisplayTypes::Block
            margin=px(5).into()
            border_width=px(7).into()
            padding=px(10).into()
        />
    )
}

#[topo::nested]
fn block_layout_with_margin_border_border_and_padding_root() -> RootNode<TestState> {
    mox!(
        <root>
            <window>
                <div style=block_layout_with_margin_border_border_and_padding_style() />
                <div style=block_layout_with_margin_border_border_and_padding_style() />
            </window>
        </root>
    )
}

#[test]
fn block_layout_with_margin_border_border_and_padding() {
    let changes = test_app(block_layout_with_margin_border_border_and_padding_root)
        .with_screen_dimensions(1024, 768)
        .build()
        .run_once()
        .get_changes();

    assert_eq!(changes.len(), 2);
    assert_eq!(changes[0].is_positioned_at(0, 0), true);
    assert_eq!(changes[0].content_is_positioned_at(22, 22), true);
    assert_eq!(changes[0].has_dimensions_of(1024, 44), true);
    assert_eq!(changes[0].content_has_dimensions_of(980, 0), true);
    assert_eq!(changes[1].is_positioned_at(0, 44), true);
    assert_eq!(changes[1].content_is_positioned_at(22, 66), true);
    assert_eq!(changes[1].has_dimensions_of(1024, 44), true);
    assert_eq!(changes[1].content_has_dimensions_of(980, 0), true);
}