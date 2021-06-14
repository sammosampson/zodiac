use app_state::*;
use zodiac::*;
use zodiac_hotloading::*;
use zodiac_rendering_html_webrender::*;
use crate::root::app_root;

zod_hotload_client!(
    [initialise_application] 
    [TestState]
);

fn initialise_application(state: TestState) -> ApplicationRunner<TestState> {
    std::env::set_var("RUST_LOG", "info");
    Application::new(state, app_root)
        .use_logging()
        .with_builder(html_webrender_renderer())
        //.with_builder(world_logging())
        .build()
        .unwrap()
}