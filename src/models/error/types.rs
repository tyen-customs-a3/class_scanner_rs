use std::path::PathBuf;
use std::error::Error as StdError;
use std::fmt;
use std::io;
use pbo_tools_rs::error::PboError;

#[derive(Debug)]
pub enum Error {
    /// Validation errors during parsing
    Validation(String),
    /// Parsing errors with context
    Parse {
        message: String,
        line: Option<usize>,
        column: Option<usize>,
    },
    /// Structural errors in class definitions
    Structure(String),
    /// I/O errors with context
    IO(std::io::Error),
    /// Configuration errors
    Config(String),
    PropertyMissing(String),
    TypeMismatch(PropertyTypeMismatchError),
    PboScanError { path: PathBuf, source: Box<dyn StdError + Send + Sync> },
    FileTooLarge { path: PathBuf },
}

#[derive(Debug)]
pub struct PropertyTypeMismatchError {
    pub expected_type: String,
    pub actual_type: String,
    pub property_name: String,
}

impl PropertyTypeMismatchError {
    pub fn new(expected: &str, actual: &str) -> Self {
        Self {
            expected_type: expected.to_string(),
            actual_type: actual.to_string(),
            property_name: "unknown".to_string(),
        }
    }

    pub fn with_property_name(mut self, name: &str) -> Self {
        self.property_name = name.to_string();
        self
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Validation(msg) => write!(f, "Validation error: {}", msg),
            Error::Parse { message, line, column } => {
                if let (Some(l), Some(c)) = (line, column) {
                    write!(f, "Parse error at {}:{}: {}", l, c, message)
                } else {
                    write!(f, "Parse error: {}", message)
                }
            },
            Error::Structure(msg) => write!(f, "Structure error: {}", msg),
            Error::IO(err) => write!(f, "I/O error: {}", err),
            Error::Config(msg) => write!(f, "Configuration error: {}", msg),
            Error::PropertyMissing(name) => write!(f, "Property '{}' not found", name),
            Error::TypeMismatch(e) => write!(
                f,
                "Type mismatch for property '{}': expected {}, got {}",
                e.property_name, e.expected_type, e.actual_type
            ),
            Error::PboScanError { path, source } => {
                write!(f, "PBO scan error for {:?}: {}", path, source)
            }
            Error::FileTooLarge { path } => {
                write!(f, "File too large: {:?}", path)
            }
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::IO(err) => Some(err),
            Error::PboScanError { source, .. } => Some(source.as_ref()),
            _ => None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IO(err)
    }
}

pub type Result<T> = std::result::Result<T, Error>;

pub type ParserError = Error;
pub type ScannerError = Error;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = Error::Parse {
            message: "Invalid token".to_string(),
            line: Some(10),
            column: Some(5),
        };
        assert_eq!(
            err.to_string(),
            "Parse error at 10:5: Invalid token"
        );
    }

    #[test]
    fn test_io_error_conversion() {
        let io_err = std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "File not found"
        );
        let err: Error = io_err.into();
        match err {
            Error::IO(_) => (),
            _ => panic!("Expected IO error variant"),
        }
    }
}