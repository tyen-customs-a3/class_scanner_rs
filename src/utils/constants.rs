use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    pub static ref CLASS_PATTERN: Regex = Regex::new(
        r"class\s+(\w+)(?:\s*:\s*(?:public\s+)?(\w+))?\s*\{"
    ).unwrap();

    pub static ref PROPERTY_PATTERN: Regex = Regex::new(
        r#"(?m)^\s*(\w+)(?:\[\])?\s*((?:\+|-)?=)\s*(.+?);\s*$"#
    ).unwrap();

    pub static ref INCLUDE_PATTERN: Regex = Regex::new(
        r#"#include\s+"([^"]+)""#
    ).unwrap();

    pub static ref DEFINE_PATTERN: Regex = Regex::new(
        r"#define\s+(\w+)\s+(.+)"
    ).unwrap();
}

// Common string literals used in parsing
pub const CLASS_KEYWORD: &str = "class";
pub const PUBLIC_KEYWORD: &str = "public";
pub const PRIVATE_KEYWORD: &str = "private";

// Common file extensions
pub const CONFIG_FILE_EXTENSION: &str = ".cpp";
pub const HEADER_FILE_EXTENSION: &str = ".hpp";