mod logrecord;
mod loggers;
mod processor;

pub use logrecord::LogRecord;
pub use processor::{Processor, NopProcessor};
pub use loggers::{DefaultLogger, ConsoleLogger};

pub trait Logger {
    /// Submit log event to logger
    fn log<'a>(&mut self, records: Vec<&'a LogRecord>) -> Vec<&'a LogRecord>;

    // Usual implementation looks like this
    // (assuming there is _log() method that does the actual logging work)
    // fn log<'a>(&mut self, records: Vec<&'a LogRecord>) -> Vec<&'a LogRecord> {
    //     let _records = match self.inner.as_mut() {
    //         Some(inner) => inner.log(records),
    //         None => records,
    //     };
    //     self._log(_records)
    // }

    /// Create default logger
    fn default_logger() -> DefaultLogger
    {
        DefaultLogger::new()
    }

    /// Create default logger
    fn console_logger(self) -> ConsoleLogger<Self>
        where Self: Sized + Logger
    {
        ConsoleLogger::new(self)
    }
}

pub fn get_logger() -> DefaultLogger {
    DefaultLogger::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_zero_count() {
        let log = get_logger();
        println!("{:?}", log);
        assert!(true);
    }

    #[test]
    fn chained_loggers() {
        let log = get_logger().console_logger();
        println!("{:?}", log);
        assert!(true);
    }

    #[test]
    fn simple_log_entry() {
        let mut log = get_logger().console_logger();
        log.log(vec!(&LogRecord::default()));
        assert!(true);
    }
}
