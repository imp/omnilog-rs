mod logger;
mod loggers;
mod logrecord;
mod processor;
mod loglevel;

pub use logger::{Logger, console_logger, syslog_logger, get_logger};
pub use loggers::{DefaultLogger, ConsoleLogger, SyslogLogger};
pub use loglevel::LogLevel;
pub use logrecord::LogRecord;
pub use processor::{Processor, NopProcessor};

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
        let mut log = console_logger("alpha").console_logger("delta");
        println!("");
        log.log(vec![&LogRecord::new(LogLevel::Debug, "simple_log_entry")]);
        println!("");
        assert!(true);
    }
}
