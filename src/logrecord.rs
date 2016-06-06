use super::loglevel::LogLevel;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
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

    pub fn item(&mut self, item: LogItem) -> &Self {
        self.items.push(item);
        self
    }

    pub fn items(&mut self, items: &mut Vec<LogItem>) -> &Self {
        self.items.append(items);
        self
    }

    pub fn get_level(&self) -> LogLevel {
        self.level
    }

    pub fn get_id(&self) -> LogRecordId {
        self.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use LogLevel::*;

    #[test]
    fn default_log_record() {
        let record = LogRecord::default();
        assert_eq!(record.get_id(), LogRecordId(0));
    }

    #[test]
    fn simple_log_record() {
        let record = LogRecord::new(Debug);
        assert_eq!(record.get_level(), Debug);
    }

    #[test]
    fn add_one_item() {
        let mut record = LogRecord::new(Info);
        record.item(LogItem::default());
        assert_eq!(record.items.len(), 1);
    }

    #[test]
    fn add_many_items() {
        let mut record = LogRecord::new(Warning);
        let mut items = vec!(LogItem::default(), LogItem::default());
        record.items(&mut items);
        assert_eq!(record.items.len(), 2);
    }
}
