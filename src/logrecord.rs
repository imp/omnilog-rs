use super::loglevel::LogLevel;

#[derive(Debug, Default, PartialEq)]
pub struct LogRecordId(u64);

#[derive(Debug, Default, PartialEq)]
pub struct LogItem {
    key: String,
    value: String,
}

#[derive(Debug, Default, PartialEq)]
pub struct LogRecord {
    level: LogLevel,
    id: LogRecordId,
    items: Vec<LogItem>,
}

impl LogRecord {
    pub fn new(lvl: LogLevel) -> Self {
        LogRecord { level: lvl, .. LogRecord::default() }
    }

    pub fn level(record: &LogRecord) -> LogLevel {
        record.level
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use LogLevel::*;

    #[test]
    fn default_log_record() {
        let record = LogRecord::default();
        assert_eq!(record.id.0, 0);
    }

    #[test]
    fn simple_log_record() {
        let record = LogRecord::new(Debug);
        assert_eq!(record.level, Debug);
    }
}
