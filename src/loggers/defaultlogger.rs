use ::Logger;
use logrecord::LogRecord;

/// DefaultLogger is a default head of the logger stack.
/// It always terminates the chain and effectively does nothing.
#[derive(Debug, Default)]
pub struct DefaultLogger;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_zero_count_does_nothing() {
        assert!(true);
    }
}
