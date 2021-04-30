use zodiac::initialisation::*;

fn main() {
    Application::new()
        .use_logging()
        .with_builders(&mut standard_builders("examples\\assets\\first"))
        .build()
        .unwrap()
        .run_until_closed();
}