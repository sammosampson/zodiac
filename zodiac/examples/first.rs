use zodiac::initialisation::*;

fn main() {
    pretty_env_logger::init();

    Application::new()
        .with_builders(&mut standard_builders("examples\\assets\\test_zods"))
        .build()
        .unwrap()
        .run_until_closed();
}