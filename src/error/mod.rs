use thiserror::Error;
use std::{io, path::PathBuf};

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("Lexer error at {location}: {message}")]
    LexerError {
        message: String,
        location: SourceLocation,
    },

    #[error("Parser error at {location}: {message}")]
    ParseError {
        message: String,
        location: SourceLocation,
    },

    #[error("Type error: {0}")]
    TypeError(String),

    #[error("Inheritance error: {0}")]
    InheritanceError(String),

    #[error("Include error: Could not include file '{0}' from '{1}'")]
    IncludeError(String, String),

    #[error("Macro error: {0}")]
    MacroError(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct SourceLocation {
    pub file: Option<PathBuf>,
    pub line: usize,
    pub column: usize,
}

impl SourceLocation {
    pub fn new(file: Option<PathBuf>, line: usize, column: usize) -> Self {
        Self { file, line, column }
    }

    pub fn unknown() -> Self {
        Self {
            file: None,
            line: 0,
            column: 0,
        }
    }

    pub fn with_file(mut self, file: PathBuf) -> Self {
        self.file = Some(file);
        self
    }

    pub fn with_context(&self) -> String {
        match (&self.file, self.line) {
            (Some(path), line) if line > 0 => format!(" at {}:{}", path.display(), line),
            (None, line) if line > 0 => format!(" at line {}", line),
            _ => String::new()
        }
    }
}

impl std::fmt::Display for SourceLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.file {
            Some(path) => write!(f, "{}:{}:{}", path.display(), self.line, self.column),
            None => write!(f, "line {}:{}", self.line, self.column),
        }
    }
}

