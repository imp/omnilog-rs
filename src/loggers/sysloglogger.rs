use Logger;
use LogRecord;

/// `SyslogLogger` is a simple logger that logs everything to the UNIX syslog facility.
/// It is very handy as a last link in the logger chain.
#[derive(Debug, Default)]
pub struct SyslogLogger<L>
    where L: Logger
{
    next: Option<L>,
    name: String,
}

impl<L> SyslogLogger<L>
    where L: Logger
{
    pub fn new() -> Self {
        SyslogLogger {
            next: None,
            name: String::new(),
        }
    }

    pub fn chain(next: L) -> Self {
        SyslogLogger {
            next: Some(next),
            name: String::new(),
        }
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = name.to_owned();
        self
    }

    fn _log<'a>(&mut self, records: Vec<&'a LogRecord>) -> Vec<&'a LogRecord> {
        // XXX Replace with real implementation
        records
    }
}

impl<L> Logger for SyslogLogger<L>
    where L: Logger
{
    fn log<'a>(&mut self, records: Vec<&'a LogRecord>) -> Vec<&'a LogRecord> {
        let r = match self.next {
            Some(ref mut next) => next.log(records),
            None => records,
        };
        self._log(r)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn initial_zero_count_does_nothing() {
        assert!(true);
    }
}
