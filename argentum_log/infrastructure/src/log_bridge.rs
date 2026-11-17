use argentum_log_business::{Level, LoggerTrait};
use std::sync::Arc;

/// Bridges the standard `log` facade into [`LoggerTrait`].
pub struct LogBridge<L>
where
    L: LoggerTrait + 'static,
{
    logger: Arc<L>,
}

impl<L> LogBridge<L>
where
    L: LoggerTrait + 'static,
{
    pub fn install(logger: Arc<L>) {
        let _ = log::set_boxed_logger(Box::new(Self { logger }));
        log::set_max_level(log::LevelFilter::Debug);
    }
}

impl<L> log::Log for LogBridge<L>
where
    L: LoggerTrait + 'static,
{
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::Level::Debug
    }

    fn log(&self, record: &log::Record) {
        let level = match record.level() {
            log::Level::Error => Level::Error,
            log::Level::Warn => Level::Warning,
            log::Level::Info => Level::Info,
            log::Level::Debug => Level::Debug,
            log::Level::Trace => Level::Trace,
        };

        self.logger.log(level, record.args().to_string());
    }

    fn flush(&self) {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    struct CapturingLogger {
        records: Mutex<Vec<String>>,
    }

    impl LoggerTrait for CapturingLogger {
        fn log<S: AsRef<str>>(&self, _level: Level, msg: S) {
            self.records.lock().unwrap().push(msg.as_ref().to_string());
        }
    }

    #[test]
    fn bridge_forwards_sqlx_records_to_logger_trait() {
        let logger = Arc::new(CapturingLogger {
            records: Mutex::new(vec![]),
        });
        LogBridge::install(logger.clone());

        log::debug!(target: "sqlx::query", "SELECT * FROM users");

        let records = logger.records.lock().unwrap();
        assert!(
            records.iter().any(|m| m.contains("SELECT * FROM users")),
            "sqlx::query record should reach LoggerTrait"
        );
    }
}
