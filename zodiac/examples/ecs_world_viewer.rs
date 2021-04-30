use zodiac::initialisation::*;

fn main() {
    std::env::set_var("RUST_LOG", "info");
    Application::new()
        .use_logging()
        .with_builders(&mut standard_builders("examples\\assets\\ecs_world_viewer"))
        //.with_builder(world_vision())
        .build()
        .unwrap()
        .run_until_closed();
}