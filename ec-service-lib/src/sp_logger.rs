use log::{Metadata, Record};

pub struct SpLogger;

impl log::Log for SpLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let module_path = record.module_path().unwrap_or("unknown");
            odp_ffa::println!("{:<5} - {} - {}", record.level(), module_path, record.args());
        }
    }

    fn flush(&self) {}
}
