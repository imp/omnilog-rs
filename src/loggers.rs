use super::LogRecord;
use super::Processor;
use super::Logger;

#[derive(Debug, Default)]
pub struct DefaultLogger<L>
    where L: Logger
{
    inner: Option<L>,
}

impl<L> DefaultLogger<L>
    where L: Logger
{
    pub fn new(inner: Option<L>) -> Option<Self> {
        Some(DefaultLogger { inner: inner })
    }

    fn _log<'a>(&mut self, records: Vec<&'a LogRecord>) -> Vec<&'a LogRecord> {
        records
    }
}

impl<L> Logger for DefaultLogger<L>
    where L: Logger
{
    fn log<'a>(&mut self, records: Vec<&'a LogRecord>) -> Vec<&'a LogRecord> {
        let _records = match self.inner.as_mut() {
            Some(inner) => inner.log(records),
            None => records,
        };
        self._log(_records)
    }
}

#[derive(Debug, Default)]
pub struct ConsoleLogger<L>
    where L: Logger
{
    inner: Option<L>,
}

impl<L> ConsoleLogger<L>
    where L: Logger
{
    pub fn new(inner: Option<L>) -> Option<Self> {
        Some(ConsoleLogger { inner: inner })
    }

    fn _log<'a>(&mut self, records: Vec<&'a LogRecord>) -> Vec<&'a LogRecord> {
        println!("{:?}", records);
        records
    }
}

impl<L> Logger for ConsoleLogger<L>
    where L: Logger
{
    fn log<'a>(&mut self, records: Vec<&'a LogRecord>) -> Vec<&'a LogRecord> {
        let _records = match self.inner.as_mut() {
            Some(inner) => inner.log(records),
            None => records,
        };
        self._log(_records)
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
