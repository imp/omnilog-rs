use super::LogRecord;

pub trait Processor {
    fn process<'a>(&self, Vec<&'a LogRecord>) -> Vec<&'a LogRecord>;
}

#[derive(Debug, Default)]
pub struct NopProcessor;

impl Processor for NopProcessor {
    fn process<'a>(&self, records: Vec<&'a LogRecord>) -> Vec<&'a LogRecord> {
        records
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use logrecord::LogRecord;

    #[test]
    fn null_processor() {
        let p = NopProcessor::default();
        let r = &LogRecord::default();
        assert_eq!(vec![r], p.process(vec![r]));
    }
}
