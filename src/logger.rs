use simplelog::*;
use std::fs::File;

pub fn init_logger() {
    let _ = WriteLogger::init(
        LevelFilter::Info,
        Config::default(),
        File::create("gambleguard.log").unwrap(),
    );
}
