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
    fn log<S: AsRef<str>>(&self, level: Level, msg: S);

    fn trace<S: AsRef<str>>(&self, msg: S) {
        self.log(Level::Trace, msg);
    }

    fn debug<S: AsRef<str>>(&self, msg: S) {
        self.log(Level::Debug, msg);
    }

    fn info<S: AsRef<str>>(&self, msg: S) {
        self.log(Level::Info, msg);
    }

    fn warning<S: AsRef<str>>(&self, msg: S) {
        self.log(Level::Warning, msg);
    }

    fn error<S: AsRef<str>>(&self, msg: S) {
        self.log(Level::Error, msg);
    }

    fn critical<S: AsRef<str>>(&self, msg: S) {
        self.log(Level::Critical, msg);
    }
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
    fn log<S: AsRef<str>>(&self, level: Level, msg: S) {
        if level < self.level {
            return;
        }

        self.writer.write(Utc::now(), level, msg.as_ref())
    }
}

pub trait WriterTrait: Send + Sync {
    fn write<S: AsRef<str>>(&self, date_time: DateTime<Utc>, level: Level, msg: S);
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
    fn write<S: AsRef<str>>(&self, time: DateTime<Utc>, level: Level, msg: S) {
        println!(
            "{} {}: {}",
            time.format("%Y-%m-%d %H:%M:%S%.3f%:z"),
            level.to_string().to_uppercase(),
            msg.as_ref(),
        );
    }
}
