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
        write!(f, "{self:?}")
    }
}

pub trait LoggerTrait: Send + Sync {
    fn log(&self, level: Level, msg: impl Into<String>);
    fn trace(&self, msg: impl Into<String>);
    fn debug(&self, msg: impl Into<String>);
    fn info(&self, msg: impl Into<String>);
    fn warning(&self, msg: impl Into<String>);
    fn error(&self, msg: impl Into<String>);
    fn critical(&self, msg: impl Into<String>);
}

pub struct DefaultLogger<W>
where
    W: WriterTrait,
{
    level: Level,
    writer: Arc<W>,
}

impl<W> DefaultLogger<W>
where
    W: WriterTrait,
{
    pub fn new(level: Level, writer: Arc<W>) -> DefaultLogger<W> {
        DefaultLogger { level, writer }
    }
}

impl<W> LoggerTrait for DefaultLogger<W>
where
    W: WriterTrait,
{
    fn log(&self, level: Level, msg: impl Into<String>) {
        if level < self.level {
            return;
        }

        self.writer.write(Utc::now(), level, msg.into())
    }

    fn trace(&self, msg: impl Into<String>) {
        self.log(Level::Trace, msg);
    }

    fn debug(&self, msg: impl Into<String>) {
        self.log(Level::Debug, msg);
    }

    fn info(&self, msg: impl Into<String>) {
        self.log(Level::Info, msg);
    }

    fn warning(&self, msg: impl Into<String>) {
        self.log(Level::Warning, msg);
    }

    fn error(&self, msg: impl Into<String>) {
        self.log(Level::Error, msg);
    }

    fn critical(&self, msg: impl Into<String>) {
        self.log(Level::Critical, msg);
    }
}

pub trait WriterTrait: Send + Sync {
    fn write(&self, date_time: DateTime<Utc>, level: Level, msg: impl Into<String>);
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
    fn write(&self, time: DateTime<Utc>, level: Level, msg: impl Into<String>) {
        println!(
            "{} {}: {}",
            time.format("%Y-%m-%d %H:%M:%S%.3f%:z"),
            level.to_string().to_uppercase(),
            msg.into(),
        );
    }
}
