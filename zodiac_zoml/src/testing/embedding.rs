use zodiac::*;
use crate::testing::*;
use crate::components::*;

element! {
    <test_renderable>    
    [TestRenderable::default()]
    extra_components {
        [Renderable::default()]
    }
    attributes {
        left(u16)
        top(u16)
        width(u16)
        height(u16)
    }
}