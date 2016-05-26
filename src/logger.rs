use super::LogRecord;
use super::Processor;

pub trait Logger {
    fn log<'a>(&mut self, Vec<&'a LogRecord>) -> Vec<&'a LogRecord>;
}

#[derive(Debug, Default)]
pub struct GenericLogger<T> where T: Processor {
    count: usize,
    pre: Vec<T>,
    post: Vec<T>,
}

impl<T> Logger for GenericLogger<T> where T: Processor {
    fn log<'a>(&mut self, records: Vec<&'a LogRecord>) -> Vec<&'a LogRecord> {
        records
    }
}

impl<T> GenericLogger<T> where T: Processor {
    pub fn new() -> Self {
        GenericLogger {
            count: 0,
            pre: vec![],
            post: vec![],
        }
    }

    pub fn log1(&mut self, msg: &str) {
        let len = msg.len();

        if len > 0 {
            self.count += len;
        }
    }

    pub fn count(&self) -> usize {
        self.count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::NopProcessor;

    #[test]
    fn initial_zero_count() {
        let log = GenericLogger::<NopProcessor>::new();
        assert_eq!(log.count(), 0);
    }
}
