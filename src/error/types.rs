use std::path::PathBuf;
use std::error::Error as StdError;
use std::fmt;
use std::io;
use pbo_tools_rs::error::PboError;

#[derive(Debug)]
pub enum Error {
    Parser(ParserError),
    Scanner(ScannerError),
    Pbo(PboError),
    Io(io::Error),
}

#[derive(Debug)]
pub enum ParserError {
    TokenizationError(String),
    PropertyError(PropertyParserError),
    ClassError(ClassParserError),
    InvalidClass(String),
    InvalidValue(String),
    NestedTooDeep,
}

#[derive(Debug)]
pub enum PropertyParserError {
    InvalidProperty(String),
    InvalidValue(String),
    UnterminatedString(usize),
    UnexpectedToken {
        found: String,
        expected: String,
        pos: usize,
    },
}

#[derive(Debug)]
pub enum ClassParserError {
    BlockTooDeep(usize),
    UnmatchedBlock(usize),
    InvalidClassName(String),
    NoCodeFiles(PathBuf),
    FileTooLarge { path: PathBuf },
}

#[derive(Debug)]
pub enum ScannerError {
    NoCodeFiles(PathBuf),
    FileTooLarge { path: PathBuf },
    FileProcessing {
        path: PathBuf,
        source: std::io::Error,
    },
    Timeout(PathBuf),
    FileReadError(std::io::Error),
    PboScanError {
        path: PathBuf,
        source: Box<dyn std::error::Error + Send + Sync>,
    },
    ParseError(String),
}

// Error conversion implementations
impl From<ParserError> for Error {
    fn from(err: ParserError) -> Self {
        Error::Parser(err)
    }
}

impl From<ScannerError> for Error {
    fn from(err: ScannerError) -> Self {
        Error::Scanner(err)
    }
}

impl From<PboError> for Error {
    fn from(err: PboError) -> Self {
        Error::Pbo(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<PropertyParserError> for ParserError {
    fn from(err: PropertyParserError) -> Self {
        ParserError::PropertyError(err)
    }
}

impl From<ClassParserError> for ParserError {
    fn from(err: ClassParserError) -> Self {
        ParserError::ClassError(err)
    }
}

// Display implementations
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Parser(e) => write!(f, "Parser error: {}", e),
            Error::Scanner(e) => write!(f, "Scanner error: {}", e),
            Error::Pbo(e) => write!(f, "PBO error: {}", e),
            Error::Io(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserError::TokenizationError(e) => write!(f, "Tokenization error: {}", e),
            ParserError::PropertyError(e) => write!(f, "Property error: {}", e),
            ParserError::ClassError(e) => write!(f, "Class error: {}", e),
            ParserError::InvalidClass(e) => write!(f, "Invalid class: {}", e),
            ParserError::InvalidValue(e) => write!(f, "Invalid value: {}", e),
            ParserError::NestedTooDeep => write!(f, "Class nesting too deep"),
        }
    }
}

impl fmt::Display for PropertyParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PropertyParserError::InvalidProperty(e) => write!(f, "Invalid property: {}", e),
            PropertyParserError::InvalidValue(e) => write!(f, "Invalid value: {}", e),
            PropertyParserError::UnterminatedString(pos) => write!(f, "Unterminated string at position {}", pos),
            PropertyParserError::UnexpectedToken { found, expected, pos } => {
                write!(f, "Unexpected token {} at position {}, expected {}", found, pos, expected)
            }
        }
    }
}

impl fmt::Display for ClassParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ClassParserError::BlockTooDeep(depth) => write!(f, "Block nesting too deep ({})", depth),
            ClassParserError::UnmatchedBlock(pos) => write!(f, "Unmatched block at position {}", pos),
            ClassParserError::InvalidClassName(name) => write!(f, "Invalid class name: {}", name),
            ClassParserError::NoCodeFiles(path) => write!(f, "No code files found in {}", path.display()),
            ClassParserError::FileTooLarge { path } => write!(f, "File exceeds maximum size: {}", path.display()),
        }
    }
}

impl fmt::Display for ScannerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScannerError::NoCodeFiles(path) => write!(f, "No code files found in {}", path.display()),
            ScannerError::FileTooLarge { path } => write!(f, "File exceeds maximum size: {}", path.display()),
            ScannerError::FileProcessing { path, source } => {
                write!(f, "Failed to process file {}: {}", path.display(), source)
            }
            ScannerError::Timeout(path) => write!(f, "Parsing timeout exceeded for {}", path.display()),
            ScannerError::FileReadError(e) => write!(f, "File read error: {}", e),
            ScannerError::PboScanError { path, source } => {
                write!(f, "Failed to scan PBO {}: {}", path.display(), source)
            }
            ScannerError::ParseError(msg) => write!(f, "Parse error: {}", msg),
        }
    }
}

// Standard error trait implementations
impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::Parser(_) => None,
            Error::Scanner(ScannerError::FileProcessing { source, .. }) => Some(source),
            Error::Scanner(_) => None,
            Error::Pbo(e) => Some(e),
            Error::Io(e) => Some(e),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;