use std::fmt::{Result, Write};

pub struct CodeFormatter {
    pub buf: String,
    pub spaces: &'static str,
    pub indent_lvl: usize,
}

impl Default for CodeFormatter {
    fn default() -> Self {
        Self {
            buf: String::new(),
            spaces: "\t",
            indent_lvl: 0,
        }
    }
}

impl Write for CodeFormatter {
    fn write_str(&mut self, s: &str) -> Result {
        self.buf += s;
        Ok(())
    }
}
