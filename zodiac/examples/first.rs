use zodiac::initialisation::Application;

fn main() {
    Application::build().initialise("examples/assets/test_zods")
        .unwrap()
        .run()
        .unwrap();
}