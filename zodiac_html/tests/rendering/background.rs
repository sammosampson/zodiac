use mox::mox;
use zodiac_html::*;
use zodiac_html::testing::*;
use zodiac::*;

#[topo::nested]
pub fn element_background_colour_style() -> Node {
    mox!(
        <style
            background_colour=rgb(50, 100, 200)
        />
    )
}

#[topo::nested]
fn element_background_colour_root() -> RootNode<TestState> {
    mox!(
        <root>
            <window>    
                <div style=element_background_colour_style() />
            </window>
        </root>
    )
}

#[test]
fn element_background_colour() {
    let changes = test_app(element_background_colour_root)
        .build()
        .run_once()
        .get_changes();

    assert_eq!(changes.len(), 1);
    assert_eq!(changes[0].has_background_colour_of(rgb(50, 100, 200)), true);
}
