use mox::mox;
use zodiac_html::*;
use zodiac_html::testing::*;
use zodiac::*;

#[topo::nested]
fn default_layout_for_elements_root() -> RootNode<TestState> {
    mox!(
        <root>
            <window>
                <div/>
                <span/>
            </window>
        </root>
    )
}

#[test]
fn default_layout_for_elements() {
    let changes = test_app(default_layout_for_elements_root)
        .with_screen_dimensions(1024, 768)
        .build()
        .run_once()
        .get_changes();

    assert_eq!(changes.len(), 3);
}