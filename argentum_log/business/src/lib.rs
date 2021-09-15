use chrono::{DateTime, Utc};
use std::fmt;
use std::sync::Arc;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Level {
    Trace,
    Debug,
    Info,
    Warning,
    Error,
    Critical,
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub trait LoggerTrait: Send + Sync {
    fn log(&self, level: Level, msg: String);
    fn trace(&self, msg: String);
    fn debug(&self, msg: String);
    fn info(&self, msg: String);
    fn warning(&self, msg: String);
    fn error(&self, msg: String);
    fn critical(&self, msg: String);
}

pub struct DefaultLogger {
    level: Level,
    writer: Arc<dyn WriterTrait>,
}

impl DefaultLogger {
    pub fn new(level: Level, writer: Arc<dyn WriterTrait>) -> DefaultLogger {
        DefaultLogger { level, writer }
    }
}

impl LoggerTrait for DefaultLogger {
    fn log(&self, level: Level, msg: String) {
        if level < self.level {
            return;
        }

        self.writer.write(Utc::now(), level, msg)
    }

    fn trace(&self, msg: String) {
        self.log(Level::Trace, msg);
    }

    fn debug(&self, msg: String) {
        self.log(Level::Debug, msg);
    }

    fn info(&self, msg: String) {
        self.log(Level::Info, msg);
    }

    fn warning(&self, msg: String) {
        self.log(Level::Warning, msg);
    }

    fn error(&self, msg: String) {
        self.log(Level::Error, msg);
    }

    fn critical(&self, msg: String) {
        self.log(Level::Critical, msg);
    }
}

pub trait WriterTrait: Send + Sync {
    fn write(&self, date_time: DateTime<Utc>, level: Level, msg: String);
}

pub struct StdoutWriter {}

impl StdoutWriter {
    pub fn new() -> StdoutWriter {
        StdoutWriter {}
    }
}

impl Default for StdoutWriter {
    fn default() -> Self {
        Self::new()
    }
}

impl WriterTrait for StdoutWriter {
    fn write(&self, time: DateTime<Utc>, level: Level, msg: String) {
        println!(
            "{} {}: {}",
            time.format("%Y-%m-%d %H:%M:%S%.3f%:z"),
            level.to_string().to_uppercase(),
            msg,
        );
    }
}
