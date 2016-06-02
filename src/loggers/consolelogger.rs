use ::Logger;
use logrecord::LogRecord;

/// ConsoleLogger is a simple logger that logs everything to the standard output.
/// It is very handy as a last link in the logger chain.
#[derive(Debug, Default)]
pub struct ConsoleLogger<L>
    where L: Logger
{
    next: L,
}

impl<L> ConsoleLogger<L>
    where L: Logger
{
    pub fn new(next: L) -> Self {
        ConsoleLogger { next: next }
    }

    fn _log<'a>(&mut self, records: Vec<&'a LogRecord>) -> Vec<&'a LogRecord> {
        // println!("{:?}", records);
        records.into_iter().inspect(|r| println!("{:?}", r)).collect::<Vec<_>>()
    }
}

impl<L> Logger for ConsoleLogger<L>
    where L: Logger
{
    fn log<'a>(&mut self, records: Vec<&'a LogRecord>) -> Vec<&'a LogRecord> {
        let r = self.next.log(records);
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
