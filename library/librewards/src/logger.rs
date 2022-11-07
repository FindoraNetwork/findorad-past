use log::{Level, LevelFilter, Metadata, Record, SetLoggerError};

mod logger {
    extern "C" {
        pub fn log(level: u8, record: *const u8, length: usize);
    }
}

fn level_to_u8(level: Level) -> u8 {
    match level {
        Level::Trace => 1,
        Level::Debug => 2,
        Level::Info => 3,
        Level::Warn => 4,
        Level::Error => 5,
    }
}

struct RewardsLogger;

impl log::Log for RewardsLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let s = record.args().to_string();
            let level = level_to_u8(record.level());

            unsafe {
                logger::log(level, s.as_ptr(), s.len());
            }
        }
    }

    fn flush(&self) {}
}

static LOGGER: RewardsLogger = RewardsLogger;

pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Info))
}
