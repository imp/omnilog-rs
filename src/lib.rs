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
    //     if self.inner.is_some() {
    //         self._log(self.inner.log(records))
    //     } else {
    //         self._log(records)
    //     }
    //
    //     // or alternatively
    //
    //     self._log(if self.inner.is_some() { self.inner.log(records) } else { records })
    // }

    /// Create default logger
    fn default_logger() -> Option<DefaultLogger>
    {
        DefaultLogger::new()
    }

    /// Create default logger
    fn console_logger(self) -> Option<ConsoleLogger<Self>>
        where Self: Sized + Logger
    {
        ConsoleLogger::new(Some(self))
    }
}

pub fn get_logger() -> Option<DefaultLogger> {
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
}
