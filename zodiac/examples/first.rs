use zodiac::initialisation::Application;

fn main() {
    Application::build().initialise("C:/work/other/zodiac/zodiac/examples/assets/test_zods")
        .unwrap()
        .run()
        .unwrap();
}