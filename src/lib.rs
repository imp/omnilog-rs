#[derive(Debug, Default)]
pub struct LogRecord;

#[derive(Debug, Default)]
pub struct Sink;

#[derive(Debug, Default)]
pub struct Filter;

#[derive(Debug, Default)]
pub struct Logger {
    count: usize,
    filters: Vec<Filter>,
    sinks: Vec<Sink>,
}

impl Logger {
    pub fn new() -> Self {
        Logger {
            count: 0,
            filters: vec![],
            sinks: vec![],
        }
    }

    pub fn log(&mut self, msg: &str) {
        let len = msg.len();

        if len > 0 {
            self.count += len;
        }
    }

    pub fn count(&self) -> usize {
        self.count
    }
}

impl Sink {
    pub fn sink<'a>(&'a mut self, rec: &'a LogRecord) -> &LogRecord {
        rec
    }
}

pub fn get_logger() -> Logger {
    Logger::new()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn initial_zero_count() {
        let log = get_logger();
        assert_eq!(log.count(), 0);
    }
    #[test]
    fn log_five_chars() {
        let mut log = get_logger();
        log.log("12345");
        assert_eq!(log.count(), 5);
    }
}
