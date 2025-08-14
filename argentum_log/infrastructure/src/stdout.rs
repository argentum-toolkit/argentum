use argentum_log_business::{Level, WriterTrait};
use chrono::{DateTime, Utc};
use yansi::Paint;

pub struct PrettyWriter {}

impl PrettyWriter {
    pub fn new() -> PrettyWriter {
        PrettyWriter {}
    }
}

impl Default for PrettyWriter {
    fn default() -> Self {
        Self::new()
    }
}

impl WriterTrait for PrettyWriter {
    fn write<S: AsRef<str>>(&self, time: DateTime<Utc>, level: Level, msg: S) {
        let l = level.to_string().to_uppercase();
        let colored_level = match level {
            Level::Trace => Paint::blue(&l),
            Level::Debug => Paint::cyan(&l),
            Level::Info => Paint::green(&l),
            Level::Warning => Paint::yellow(&l),
            Level::Error => Paint::red(&l),
            Level::Critical => Paint::rgb(&l, 0xDD, 0, 0),
        };

        println!(
            "{} {}: {}",
            time.format("%Y-%m-%d %H:%M:%S%.3f%:z"),
            colored_level,
            msg.as_ref(),
        );
    }
}
