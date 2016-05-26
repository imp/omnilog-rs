mod logrecord;
mod logger;
mod processor;

pub use logrecord::LogRecord;
pub use processor::{Processor, NopProcessor};
pub use logger::{Logger, GenericLogger};

pub fn get_logger<T: Processor>() -> GenericLogger<T> {
    GenericLogger::<T>::new()
}

pub fn get_default_logger() -> GenericLogger<NopProcessor> {
    get_logger::<NopProcessor>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_zero_count() {
        let log = get_default_logger();
        assert_eq!(log.count(), 0);
    }
}
