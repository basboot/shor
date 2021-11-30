use colour::{dark_grey_ln, dark_yellow_ln, grey_ln, red_ln};
use log::{Record, Level, Metadata, SetLoggerError, LevelFilter};

// taken from the documentation: https://docs.rs/log/0.4.14/log/trait.Log.html
struct SimpleLogger;

// levels
// error
// warning
// info
// debug
// trace

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= log::max_level()
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            match record.level() {
                Level::Error => { red_ln!("[{}] {}", record.level(), record.args()); }
                Level::Warn => { dark_yellow_ln!("[{}] {}", record.level(), record.args()); }
                Level::Info => { dark_grey_ln!("[{}] {}", record.level(), record.args()); }
                Level::Debug => { grey_ln!("[{}] {}", record.level(), record.args()); }
                Level::Trace => { grey_ln!("[{}] {}", record.level(), record.args()); }
            }

        }
    }

    fn flush(&self) {}
}

static LOGGER: SimpleLogger = SimpleLogger;

pub fn init_logger() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Debug))
}