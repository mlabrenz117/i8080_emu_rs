use i8080_emulator::Emulator;
use simplelog::{Config, LevelFilter, SimpleLogger};

mod rom;
use self::rom::rom;

fn main() {
    let mut config = Config::default();
    config.time = None;
    config.level = None;
    SimpleLogger::init(LevelFilter::Info, config).unwrap();

    let mut emulator = Emulator::new(rom());
    emulator.run();
}
