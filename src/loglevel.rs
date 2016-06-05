use std::fmt;

#[derive(Debug, PartialEq, PartialOrd)]
pub enum LogLevel {
    Panic,
    Error,
    Warning,
    Info,
    Debug,
    Trace,
}

impl Default for LogLevel {
    fn default() -> Self {
        LogLevel::Trace
    }
}

impl fmt::Display for LogLevel {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let text = match *self {
            LogLevel::Panic => "PANIC",
            LogLevel::Error => "ERROR",
            LogLevel::Warning => "WARNING",
            LogLevel::Info => "INFO",
            LogLevel::Debug => "DEBUG",
            LogLevel::Trace => "TRACE",
        };
        write!(fmt, "{}", text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loglevel() {
        let panic = LogLevel::Panic;
        let error = LogLevel::Error;

        assert!(panic < error);
    }
}
