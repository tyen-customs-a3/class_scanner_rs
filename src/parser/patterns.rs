use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    // Class Structure Patterns
    /// Matches any class declaration including inheritance
    pub static ref CLASS_PATTERN: Regex = Regex::new(
        r"class\s+(\w+)(?:\s*:\s*(\w+))?\s*[\{;]"
    ).unwrap();

    /// Matches specifically class declarations that open a block
    pub static ref CLASS_BLOCK_START: Regex = Regex::new(
        r"class\s+(\w+)(?:\s*:\s*(\w+))?\s*\{"
    ).unwrap();

    /// Matches nested class blocks for structural analysis
    pub static ref NESTED_CLASS: Regex = Regex::new(
        r"class\s+\w+[^;]*\{[^\}]*\}"
    ).unwrap();

    // Property Patterns
    /// Matches standard and array property declarations
    pub static ref PROPERTY_PATTERN: Regex = Regex::new(
        r"(\w+)(?:\[\])?\s*=\s*([^;]*?)\s*;"
    ).unwrap();

    /// Matches array declarations (empty)
    pub static ref EMPTY_ARRAY_PATTERN: Regex = Regex::new(
        r"^\s*\{\s*\}\s*$"
    ).unwrap();

    /// Matches array declarations with content
    pub static ref ARRAY_PATTERN: Regex = Regex::new(
        r"^\s*\{([^{}]*(?:\{[^{}]*\}[^{}]*)*)\}\s*$"
    ).unwrap();

    // Value Type Patterns
    /// Matches string values with proper escaping
    pub static ref STRING_PATTERN: Regex = Regex::new(
        r#"^"([^"\\]*(?:\\.[^"\\]*)*)"$"#
    ).unwrap();

    /// Matches numeric values (integers and decimals)
    pub static ref NUMBER_PATTERN: Regex = Regex::new(
        r"^-?\d+(\.\d+)?$"
    ).unwrap();

    /// Matches boolean values
    pub static ref BOOLEAN_PATTERN: Regex = Regex::new(
        r"^(true|false)$"
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_class_pattern() {
        // Basic class declaration
        let cap = CLASS_PATTERN.captures("class TestClass {").unwrap();
        assert_eq!(cap.get(1).unwrap().as_str(), "TestClass");
        assert!(cap.get(2).is_none());

        // Class with inheritance
        let cap = CLASS_PATTERN.captures("class Child: Parent {").unwrap();
        assert_eq!(cap.get(1).unwrap().as_str(), "Child");
        assert_eq!(cap.get(2).unwrap().as_str(), "Parent");

        // Forward declaration
        let cap = CLASS_PATTERN.captures("class Forward;").unwrap();
        assert_eq!(cap.get(1).unwrap().as_str(), "Forward");
        assert!(cap.get(2).is_none());
    }

    #[test]
    fn test_property_pattern() {
        // Simple property
        let cap = PROPERTY_PATTERN.captures("name = value;").unwrap();
        assert_eq!(cap.get(1).unwrap().as_str(), "name");
        assert_eq!(cap.get(2).unwrap().as_str(), "value");

        // Array property
        let cap = PROPERTY_PATTERN.captures("items[] = {\"one\", \"two\"};").unwrap();
        assert_eq!(cap.get(1).unwrap().as_str(), "items");
        assert_eq!(cap.get(2).unwrap().as_str(), "{\"one\", \"two\"}");

        // String property with whitespace
        let cap = PROPERTY_PATTERN.captures(r#"text = "Hello World"  ;"#).unwrap();
        assert_eq!(cap.get(1).unwrap().as_str(), "text");
        assert_eq!(cap.get(2).unwrap().as_str(), r#""Hello World""#);
    }

    #[test]
    fn test_array_patterns() {
        // Empty array
        assert!(EMPTY_ARRAY_PATTERN.is_match("{}"));
        assert!(EMPTY_ARRAY_PATTERN.is_match("{ }"));
        assert!(EMPTY_ARRAY_PATTERN.is_match("  {   }  "));

        // Array with content
        let cap = ARRAY_PATTERN.captures(r#"{"one", "two"}"#).unwrap();
        assert_eq!(cap.get(1).unwrap().as_str(), r#""one", "two""#);

        // Array with nested arrays
        let cap = ARRAY_PATTERN.captures(r#"{{1, 2}, {3, 4}}"#).unwrap();
        assert_eq!(cap.get(1).unwrap().as_str(), r#"{1, 2}, {3, 4}"#);

        // Array with quoted strings containing commas
        let cap = ARRAY_PATTERN.captures(r#"{"one,two", "three,four"}"#).unwrap();
        assert_eq!(cap.get(1).unwrap().as_str(), r#""one,two", "three,four""#);
    }

    #[test]
    fn test_string_pattern() {
        // Simple string
        let cap = STRING_PATTERN.captures(r#""Hello World""#).unwrap();
        assert_eq!(cap.get(1).unwrap().as_str(), "Hello World");

        // String with escaped quotes
        let cap = STRING_PATTERN.captures(r#""Hello \"World\"""#).unwrap();
        assert_eq!(cap.get(1).unwrap().as_str(), r#"Hello \"World\""#);

        // String with path
        let cap = STRING_PATTERN.captures(r#""\tc\mirrorform\uniform\black.paa""#).unwrap();
        assert_eq!(cap.get(1).unwrap().as_str(), r#"\tc\mirrorform\uniform\black.paa"#);
    }

    #[test]
    fn test_number_pattern() {
        // Integer
        assert!(NUMBER_PATTERN.is_match("123"));
        assert!(NUMBER_PATTERN.is_match("-456"));

        // Decimal
        assert!(NUMBER_PATTERN.is_match("123.456"));
        assert!(NUMBER_PATTERN.is_match("-789.012"));

        // Invalid numbers
        assert!(!NUMBER_PATTERN.is_match("12.34.56"));
        assert!(!NUMBER_PATTERN.is_match("abc"));
    }

    #[test]
    fn test_comment_patterns() {
        // Line comments
        let input = "code; // comment\nmore code";
        let without_comments = LINE_COMMENT.replace_all(input, "\n").to_string();
        let cleaned = without_comments
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
            .replace(" ;", ";");
        assert_eq!(cleaned, "code; more code");

        // Block comments
        let input = "before /* block\ncomment */ after";
        let cleaned = BLOCK_COMMENT.replace_all(input, " ")
            .to_string()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");
        assert_eq!(cleaned, "before after");

        // Mixed comments with whitespace normalization
        let input = r#"// Line comment
        code; /* block comment */ more;
        // Another line"#;
        let without_line = LINE_COMMENT.replace_all(input, "\n").into_owned();
        let without_block = BLOCK_COMMENT.replace_all(&without_line, " ").into_owned();
        let normalized = without_block
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");
        
        assert!(normalized.contains("code;"));
        assert!(normalized.contains("more;"));
        assert!(!normalized.contains("comment"));
    }

    #[test]
    fn test_nested_class_cleaning() {
        let content = r#"
            outer = 1;
            class Inner {
                value = 2;
            };
            after = 3;
        "#;
        let cleaned = NESTED_CLASS.replace_all(content, "");
        assert!(!cleaned.contains("class Inner"));
        assert!(cleaned.contains("outer = 1"));
        assert!(cleaned.contains("after = 3"));
    }
}