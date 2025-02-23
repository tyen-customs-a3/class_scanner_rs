use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    // Class Structure Patterns
    /// Matches any class declaration including inheritance
    pub static ref CLASS_PATTERN: Regex = Regex::new(
        r"\bclass\s+(\w+)(?:\s*:\s*(\w+))?\s*"
    ).unwrap();

    /// Matches specifically class declarations that open a block
    pub static ref CLASS_BLOCK_START: Regex = Regex::new(
        r"class\s+(\w+)(?:\s*:\s*(\w+))?\s*\{"
    ).unwrap();

    /// Matches nested class blocks for structural analysis
    pub static ref NESTED_CLASS: Regex = Regex::new(
        r"class\s+\w+[^;]*\{[^\}]*\}"
    ).unwrap();

    /// Matches class blocks for cleaning nested class definitions
    pub static ref CLASS_BLOCK: Regex = Regex::new(
        r"class\s+\w+(?:[^{]|\{[^}]*\})*\}"
    ).unwrap();

    // Property Patterns
    /// Matches standard and array property declarations
    pub static ref PROPERTY_PATTERN: Regex = Regex::new(
        r"(\w+(?:\[\])?)\s*=\s*([^;]+?)(?:\s*;|$)"
    ).unwrap();

    /// Matches array declarations (empty)
    pub static ref EMPTY_ARRAY_PATTERN: Regex = Regex::new(
        r"^\s*\{\s*}\s*$"
    ).unwrap();

    /// Matches array declarations with content
    pub static ref ARRAY_PATTERN: Regex = Regex::new(
        r"^\s*\{([^{}]*(?:\{[^{}]*\}[^{}]*)*)\}\s*$"
    ).unwrap();

    // Value Type Patterns
    /// Matches string values with proper escaping
    pub static ref STRING_PATTERN: Regex = Regex::new(
        r#"^\s*"([^"]*(?:\\.[^"]*)*)"?\s*$"#
    ).unwrap();

    /// Matches numeric values (integers and decimals)
    pub static ref NUMBER_PATTERN: Regex = Regex::new(
        r"^-?\d+(?:\.\d+)?$"
    ).unwrap();

    /// Matches boolean values
    pub static ref BOOLEAN_PATTERN: Regex = Regex::new(
        r"^(?i)true|false$"
    ).unwrap();

    // Code Cleaning Patterns
    /// Matches line comments while preserving structure
    pub static ref LINE_COMMENT: Regex = Regex::new(
        r"//[^\n]*(?:\n|$)"
    ).unwrap();

    /// Matches block comments with non-greedy matching
    pub static ref BLOCK_COMMENT: Regex = Regex::new(
        r"/\*[^*]*\*+(?:[^/*][^*]*\*+)*/"
    ).unwrap();

    /// Matches redundant whitespace and newlines
    pub static ref EXTRA_NEWLINES: Regex = Regex::new(
        r"\n\s*\n"
    ).unwrap();
}
