//! Compiler diagnostics.

use std::fmt;

#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub message: String,
    pub line: Option<usize>,
    pub column: Option<usize>,
}

impl Diagnostic {
    pub fn error(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            line: None,
            column: None,
        }
    }

    pub fn at(mut self, line: usize, column: usize) -> Self {
        self.line = Some(line);
        self.column = Some(column);
        self
    }
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.line, self.column) {
            (Some(l), Some(c)) => write!(f, "error:{}:{}: {}", l, c, self.message),
            (Some(l), None) => write!(f, "error:{}: {}", l, self.message),
            _ => write!(f, "error: {}", self.message),
        }
    }
}

pub type Result<T> = std::result::Result<T, Diagnostic>;
