extern crate zodiac;
use zodiac::initialisation::Application;

fn main() {
    Application::build().initialise("assets/test_zods")
        .unwrap()
        .run()
        .unwrap();
}