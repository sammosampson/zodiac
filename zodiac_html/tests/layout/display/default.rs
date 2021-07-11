use mox::mox;
use zodiac_html::*;
use zodiac_html::testing::*;
use zodiac::*;

#[topo::nested]
fn default_layout_for_elements_root() -> RootNode<TestState> {
    mox!(
        <root>
            <window>
                <div />
                <span />
            </window>
        </root>
    )
}

#[test]
fn default_layout_for_elements() {
    configure_console_logging();

    let changes = test_app(default_layout_for_elements_root)
        .with_screen_dimensions(1024, 768)
        .build()
        .run_once()
        .get_changes();

    assert_eq!(changes.len(), 2);
    assert_eq!(changes[0].is_positioned_at(0, 0), true);
    assert_eq!(changes[0].content_is_positioned_at(0, 0), true);
    assert_eq!(changes[0].has_dimensions_of(1024, 0), true);
    assert_eq!(changes[0].content_has_dimensions_of(1024, 0), true);
    assert_eq!(changes[1].is_positioned_at(1024, 0), true);
    assert_eq!(changes[1].content_is_positioned_at(1024, 0), true);
    assert_eq!(changes[1].has_dimensions_of(0, 0), true);
    assert_eq!(changes[1].content_has_dimensions_of(0, 0), true);
}