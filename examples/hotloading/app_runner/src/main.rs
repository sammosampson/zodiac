use app_state::*;
use zodiac_hotloading::*;

fn main() {
    HotLoadableApplication::<TestState>::new("target/debug", "app").run();
}