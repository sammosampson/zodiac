use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Root, Logger};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::{Config, Handle};
use zodiac::WorldSerializer;

pub fn configure_console_logging() -> Handle {
    let standard = "standard";
    let world = "world";

    let standard_appender = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} {l} {t} - {m}{n}")))
        .append(false)
        .build("/logs/zodiac/tests/log.txt")
        .unwrap();
    
    let world_appender = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{m}\n")))
        .append(false)
        .build("/logs/zodiac/tests/world.json")
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build(world, Box::new(world_appender)))
        .appender(Appender::builder().build(standard, Box::new(standard_appender)))
        .logger(Logger::builder()
            .appender(world)
            .additive(false)
            .build(WorldSerializer::logging_target(), LevelFilter::Trace))
        .build(Root::builder().appender(standard).build(LevelFilter::Trace))
        .unwrap();

    log4rs::init_config(config).unwrap()
}