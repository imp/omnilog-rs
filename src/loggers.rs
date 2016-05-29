use super::LogRecord;
// use super::Processor;
use super::Logger;

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

/// ConsoleLogger is a simple logger that logs everything to the standard output.
/// It is very handy as a last link in the loger chain.
#[derive(Debug, Default)]
pub struct ConsoleLogger<L>
    where L: Logger
{
    inner: Option<L>,
}

impl<L> ConsoleLogger<L>
    where L: Logger
{
    pub fn new(inner: L) -> Self {
        ConsoleLogger { inner: Some(inner) }
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
    fn initial_zero_count_does_nothing() {
        assert!(true);
    }
}
