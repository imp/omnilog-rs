use ::Logger;
use logrecord::LogRecord;

/// ConsoleLogger is a simple logger that logs everything to the standard output.
/// It is very handy as a last link in the loger chain.
#[derive(Debug, Default)]
pub struct ConsoleLogger<L>
    where L: Logger
{
    inner: L,
}

impl<L> ConsoleLogger<L>
    where L: Logger
{
    pub fn new(inner: L) -> Self {
        ConsoleLogger { inner: inner }
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
        //let records1 = self.inner.map_or(records, |mut l| l.log(records));
        // let _records = match self.inner.as_mut() {
        //     Some(inner) => inner.log(records),
        //     None => records,
        // };
        // Some(records).map(self.inner.log).map(self._log).unwrap()
        let r = self.inner.log(records);
        self._log(r)
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
