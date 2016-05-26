#[derive(Debug, Default, PartialEq)]
pub struct LogRecordId(u64);

#[derive(Debug, Default, PartialEq)]
pub struct LogItem {
    key: String,
    value: String,
}

#[derive(Debug, Default, PartialEq)]
pub struct LogRecord {
    id: LogRecordId,
    items: Vec<LogItem>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_log_record() {
        let record = LogRecord::default();
        assert_eq!(record.id.0, 0);
    }
}
