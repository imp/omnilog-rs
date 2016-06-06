use super::LogRecord;
use super::{DefaultLogger, ConsoleLogger, SyslogLogger};

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
    fn default_logger() -> DefaultLogger {
        DefaultLogger::new()
    }

    /// Chain console logger
    fn console_logger(self, name: &str) -> ConsoleLogger<Self>
        where Self: Sized + Logger
    {
        ConsoleLogger::chain(self).name(name)
    }

    /// Chain syslog logger
    fn syslog_logger(self) -> SyslogLogger<Self>
        where Self: Sized + Logger
    {
        SyslogLogger::chain(self)
    }
}

/// Create console logger
pub fn console_logger(name: &str) -> ConsoleLogger<DefaultLogger> {
    ConsoleLogger::new().name(name)
}

/// Create new syslog logger
pub fn syslog_logger() -> SyslogLogger<DefaultLogger> {
    SyslogLogger::new()
}

/// Get an instance of a default logger.
/// The exact kind of the default logger is not defined
/// and happens to be a DefaultLogger at the moment.
/// Good for a generic logging facilities and as an API demonstrator.
pub fn get_logger() -> DefaultLogger {
    DefaultLogger::new()
}
