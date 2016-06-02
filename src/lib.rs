mod logrecord;
mod loggers;
mod processor;

pub use logrecord::LogRecord;
pub use processor::{Processor, NopProcessor};
pub use loggers::{DefaultLogger, ConsoleLogger, SyslogLogger};

pub trait Logger {
    /// Submit log event to logger
    fn log<'a>(&mut self, records: Vec<&'a LogRecord>) -> Vec<&'a LogRecord>;

    // Usual implementation looks like this
    // (assuming there is _log() method that does the actual logging work)
    //
    // fn log<'a>(&mut self, records: Vec<&'a LogRecord>) -> Vec<&'a LogRecord> {
    //     let r = match self.next {
    //         Some(ref mut next) => next.log(records),
    //         None => records,
    //     };
    //     self._log(r)
    // }

    /// Create default logger
    fn default_logger() -> DefaultLogger
    {
        DefaultLogger::new()
    }

    /// Chain console logger
    fn console_logger(self, name: &str) -> ConsoleLogger<Self>
        where Self: Sized + Logger
    {
        ConsoleLogger::chain(self).name(name)
    }
}

/// Create console logger
pub fn console_logger(name: &str) -> ConsoleLogger<DefaultLogger> {
    ConsoleLogger::new().name(name)
}

/// Get an instance of a default logger. The exact kind of the default logger is not defined.
/// Good for a generic logging facilities.
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
        let log = get_logger().console_logger("console");
        println!("{:?}", log);
        assert!(true);
    }

    #[test]
    fn simple_log_entry() {
        let mut log = console_logger("console").console_logger("beta");
        log.log(vec!(&LogRecord::default()));
        assert!(true);
    }
}
