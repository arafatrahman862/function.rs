use log::Log;
use std::io::{Result, Write};
use std::path::Path;
use std::{fs::File, sync::Mutex};

pub struct CodegenLogger {
    file: Mutex<File>,
}

impl CodegenLogger {
    pub fn new(path: impl AsRef<Path>) -> Result<Self> {
        let file = File::options()
            .create(true)
            .write(true)
            .append(true)
            .open(path)?;

        Ok(Self {
            file: Mutex::new(file),
        })
    }
}

impl Log for CodegenLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::max_level()
    }
    fn log(&self, record: &log::Record) {
        let mut file = self.file.lock().unwrap();
        if self.enabled(record.metadata()) {
            let _ = writeln!(&mut file, "[{}] - {}", record.level(), record.args());
        }
    }
    fn flush(&self) {}
}
