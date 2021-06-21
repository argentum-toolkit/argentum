use ansi_term::Colour;
use argentum_log_business::{Level, WriterTrait};
use chrono::{DateTime, Utc};

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
    fn write(&self, time: DateTime<Utc>, level: Level, msg: String) {
        let colored_level = match level {
            Level::Trace => Colour::Blue.paint(level.to_string().to_uppercase()),
            Level::Debug => Colour::Cyan.paint(level.to_string().to_uppercase()),
            Level::Info => Colour::Green.paint(level.to_string().to_uppercase()),
            Level::Warning => Colour::Yellow.paint(level.to_string().to_uppercase()),
            Level::Error => Colour::Red.paint(level.to_string().to_uppercase()),
            Level::Critical => Colour::RGB(0xDD, 0, 0).paint(level.to_string().to_uppercase()),
        };

        println!(
            "{} {}: {}",
            time.format("%Y-%m-%d %H:%M:%S%.3f%:z"),
            colored_level,
            msg
        );
    }
}
