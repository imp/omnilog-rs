use Logger;
use LogRecord;

/// `DefaultLogger` is a simpliest "do nothing" logger, that always terminates the logger chain.
/// It can be used for testing and debugging, or as a starting point for creating your own logger.
#[derive(Debug, Default)]
pub struct DefaultLogger {
}

impl DefaultLogger {
    pub fn new() -> Self {
        DefaultLogger::default()
    }

    fn _log<'a>(&mut self, records: Vec<&'a LogRecord>) -> Vec<&'a LogRecord> {
        records
    }
}

impl Logger for DefaultLogger {
    fn log<'a>(&mut self, records: Vec<&'a LogRecord>) -> Vec<&'a LogRecord> {
        self._log(records)
    }
}
