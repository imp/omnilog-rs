use ::{Logger, LogRecord};

/// `ConsoleLogger` is a simple logger that logs everything to the standard output.
/// It is very handy as a last link in the logger chain.
#[derive(Debug, Default)]
pub struct ConsoleLogger<L>
    where L: Logger
{
    next: Option<L>,
    name: String,
}

impl<L> ConsoleLogger<L>
    where L: Logger
{
    pub fn new() -> Self {
        ConsoleLogger {
            next: None,
            name: String::new(),
        }
    }

    pub fn chain(next: L) -> Self {
        ConsoleLogger {
            next: Some(next),
            name: String::new(),
        }
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = name.to_owned();
        self
    }

    fn _log<'a>(&mut self, records: Vec<&'a LogRecord>) -> Vec<&'a LogRecord> {
        records.into_iter().inspect(|r| self._log_single(r)).collect::<Vec<_>>()
    }

    fn _log_single(&self, record: & LogRecord) {
        println!("{} {}::{} {}", self.name, record.get_level(), module_path!(), record.get_event())
    }
}

impl<L> Logger for ConsoleLogger<L>
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
    use super::*;

    #[test]
    fn initial_zero_count_does_nothing() {
        assert!(true);
    }
}
