use serde::{Deserialize, Serialize};
use crate::models::error::{Error, PropertyTypeMismatchError, Result};
use crate::parser::patterns;
use log::trace;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PropertyValue {
    String(String),
    Number(f64),
    Integer(i64),
    Boolean(bool),
    Identifier(String),
    Array(Vec<String>),
}

impl PropertyValue {
    pub fn parse(value: &str, case_sensitive: bool) -> Result<Self> {
        let value = value.trim();
        
        // Try matching in order of most specific to least specific
        if patterns::EMPTY_ARRAY_PATTERN.is_match(value) {
            trace!("Detected empty array value");
            return Ok(Self::Array(Vec::new()));
        }
        
        if let Some(cap) = patterns::STRING_PATTERN.captures(value) {
            trace!("Detected string value: {}", &cap[1]);
            return Ok(Self::String(cap[1].to_string()));
        }
        
        if let Some(cap) = patterns::ARRAY_PATTERN.captures(value) {
            trace!("Detected array value: {}", value);
            return Ok(Self::Array(Self::split_array_items(&cap[1])));
        }
        
        if patterns::BOOLEAN_PATTERN.is_match(value) {
            trace!("Detected boolean value: {}", value);
            return Ok(Self::Boolean(value.to_lowercase() == "true"));
        }
        
        if patterns::NUMBER_PATTERN.is_match(value) {
            trace!("Detected number value: {}", value);
            return if value.contains('.') {
                Ok(Self::Number(value.parse().unwrap()))
            } else {
                Ok(Self::Integer(value.parse().unwrap()))
            };
        }
        
        // Identifier is the fallback for any valid alphanumeric+special chars
        if value.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '/' || c == '\\' || c == '.') {
            trace!("Detected identifier value: {}", value);
            return Ok(Self::Identifier(if case_sensitive {
                value.to_string()
            } else {
                value.to_lowercase()
            }));
        }

        Err(Error::Validation(format!("Invalid property value: {}", value)))
    }

    fn split_array_items(content: &str) -> Vec<String> {
        let mut items = Vec::new();
        let mut current = String::new();
        let mut depth = 0;
        let mut in_string = false;
        let mut escape = false;

        for c in content.chars() {
            match c {
                '\\' if in_string => {
                    escape = true;
                    current.push(c);
                }
                '"' if !escape => {
                    in_string = !in_string;
                    current.push(c);
                }
                '{' if !in_string => {
                    depth += 1;
                    current.push(c);
                }
                '}' if !in_string => {
                    depth -= 1;
                    current.push(c);
                }
                ',' if !in_string && depth == 0 => {
                    if !current.is_empty() {
                        trace!("Split array item: {}", current.trim());
                        items.push(current.trim().to_string());
                        current.clear();
                    }
                }
                _ => {
                    escape = false;
                    current.push(c);
                }
            }
        }

        if !current.is_empty() {
            trace!("Split final array item: {}", current.trim());
            items.push(current.trim().to_string());
        }

        items
    }

    pub fn type_name(&self) -> &'static str {
        match self {
            Self::String(_) => "String",
            Self::Number(_) => "Number",
            Self::Integer(_) => "Integer",
            Self::Boolean(_) => "Boolean",
            Self::Identifier(_) => "Identifier",
            Self::Array(_) => "Array",
        }
    }

    // Type conversion with Result
    pub fn get<'a, T>(&'a self) -> Result<T> 
    where 
        T: TryFrom<&'a PropertyValue, Error = Error>
    {
        T::try_from(self)
    }

    // Safe type accessors
    pub fn as_string(&self) -> Option<&String> {
        match self {
            Self::String(s) | Self::Identifier(s) => Some(s),
            _ => None
        }
    }

    pub fn as_number(&self) -> Option<f64> {
        match self {
            Self::Number(n) => Some(*n),
            Self::Integer(i) => Some(*i as f64),
            _ => None
        }
    }

    pub fn as_integer(&self) -> Option<i64> {
        match self {
            Self::Integer(i) => Some(*i),
            _ => None
        }
    }

    pub fn as_boolean(&self) -> Option<bool> {
        match self {
            Self::Boolean(b) => Some(*b),
            _ => None
        }
    }

    pub fn as_array(&self) -> Option<&[String]> {
        match self {
            Self::Array(arr) => Some(arr),
            _ => None
        }
    }

    // Required accessors that return Results
    pub fn require_string(&self) -> Result<&String> {
        self.as_string().ok_or_else(|| Error::TypeMismatch(PropertyTypeMismatchError {
            expected_type: "String".to_string(),
            actual_type: self.type_name().to_string(),
            property_name: "unknown".to_string(),
        }))
    }

    pub fn require_number(&self) -> Result<f64> {
        self.as_number().ok_or_else(|| Error::TypeMismatch(PropertyTypeMismatchError {
            expected_type: "Number".to_string(),
            actual_type: self.type_name().to_string(),
            property_name: "unknown".to_string(),
        }))
    }

    pub fn require_integer(&self) -> Result<i64> {
        self.as_integer().ok_or_else(|| Error::TypeMismatch(PropertyTypeMismatchError {
            expected_type: "Integer".to_string(),
            actual_type: self.type_name().to_string(),
            property_name: "unknown".to_string(),
        }))
    }

    pub fn require_boolean(&self) -> Result<bool> {
        self.as_boolean().ok_or_else(|| Error::TypeMismatch(PropertyTypeMismatchError {
            expected_type: "Boolean".to_string(),
            actual_type: self.type_name().to_string(),
            property_name: "unknown".to_string(),
        }))
    }

    pub fn require_array(&self) -> Result<&[String]> {
        self.as_array().ok_or_else(|| Error::TypeMismatch(PropertyTypeMismatchError {
            expected_type: "Array".to_string(),
            actual_type: self.type_name().to_string(),
            property_name: "unknown".to_string(),
        }))
    }

    // Add these new constructors after the existing impl block
    pub fn array(values: Vec<String>) -> Self {
        Self::Array(values)
    }

    pub fn single(value: String) -> Self {
        // Try to parse as a more specific type first
        if let Ok(parsed) = Self::parse(&value, true) {
            parsed
        } else {
            Self::String(value)
        }
    }

    pub fn is_array(&self) -> bool {
        matches!(self, Self::Array(_))
    }

    pub fn array_values(&self) -> Option<&Vec<String>> {
        match self {
            Self::Array(values) => Some(values),
            _ => None
        }
    }
}

// Implement TryFrom for common types
impl TryFrom<&PropertyValue> for String {
    type Error = Error;
    
    fn try_from(value: &PropertyValue) -> Result<Self> {
        value.as_string()
            .map(Clone::clone)
            .ok_or_else(|| Error::TypeMismatch(PropertyTypeMismatchError {
                expected_type: "String".to_string(),
                actual_type: value.type_name().to_string(),
                property_name: "unknown".to_string(),
            }))
    }
}

impl TryFrom<&PropertyValue> for f64 {
    type Error = Error;
    
    fn try_from(value: &PropertyValue) -> Result<Self> {
        value.require_number()
    }
}

impl TryFrom<&PropertyValue> for i64 {
    type Error = Error;
    
    fn try_from(value: &PropertyValue) -> Result<Self> {
        value.require_integer()
    }
}

impl TryFrom<&PropertyValue> for bool {
    type Error = Error;
    
    fn try_from(value: &PropertyValue) -> Result<Self> {
        value.require_boolean()
    }
}

impl TryFrom<&PropertyValue> for Vec<String> {
    type Error = Error;
    
    fn try_from(value: &PropertyValue) -> Result<Self> {
        value.require_array().map(|arr| arr.to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_string_values() {
        let tests = vec![
            (r#""simple string""#, "simple string"),
            (r#""quoted \"string\"""#, r#"quoted "string""#),
            (r#""path\to\file.paa""#, r#"path\to\file.paa"#),
            (r#""multi\nline""#, r#"multi\nline"#),
        ];

        for (input, expected) in tests {
            if let PropertyValue::String(value) = PropertyValue::parse(input, true).unwrap() {
                assert_eq!(value, expected);
            } else {
                panic!("Expected String variant for input: {}", input);
            }
        }
    }

    #[test]
    fn test_parse_numeric_values() {
        assert!(matches!(PropertyValue::parse("123", true).unwrap(), PropertyValue::Integer(123)));
        assert!(matches!(PropertyValue::parse("-456", true).unwrap(), PropertyValue::Integer(-456)));
        assert!(matches!(PropertyValue::parse("1.234", true).unwrap(), PropertyValue::Number(1.234)));
        assert!(matches!(PropertyValue::parse("-5.678", true).unwrap(), PropertyValue::Number(-5.678)));
    }

    #[test]
    fn test_parse_boolean_values() {
        assert!(matches!(PropertyValue::parse("true", true).unwrap(), PropertyValue::Boolean(true)));
        assert!(matches!(PropertyValue::parse("false", true).unwrap(), PropertyValue::Boolean(false)));
        assert!(matches!(PropertyValue::parse("TRUE", true).unwrap(), PropertyValue::Boolean(true)));
        assert!(matches!(PropertyValue::parse("FALSE", true).unwrap(), PropertyValue::Boolean(false)));
    }

    #[test]
    fn test_parse_array_values() {
        let tests = vec![
            ("{}", 0),
            (r#"{"single"}"#, 1),
            (r#"{"one", "two"}"#, 2),
            (r#"{"a", "b", "c"}"#, 3),
            (r#"{{"nested", "array"}, {"second", "part"}}"#, 2),
        ];

        for (input, expected_len) in tests {
            if let PropertyValue::Array(values) = PropertyValue::parse(input, true).unwrap() {
                assert_eq!(values.len(), expected_len);
            } else {
                panic!("Expected Array variant for input: {}", input);
            }
        }
    }

    #[test]
    fn test_parse_identifiers() {
        let tests = vec![
            "simple_ident",
            "WITH_CAPS",
            "mixed_Case_123",
            r"path\to\file",
            "class.subclass",
        ];

        for input in tests {
            if let PropertyValue::Identifier(value) = PropertyValue::parse(input, true).unwrap() {
                assert_eq!(value, input);
            } else {
                panic!("Expected Identifier variant for input: {}", input);
            }
        }

        // Test case sensitivity
        if let PropertyValue::Identifier(value) = PropertyValue::parse("UPPERCASE", false).unwrap() {
            assert_eq!(value, "uppercase");
        }
    }

    #[test]
    fn test_type_conversions() {
        let string_val = PropertyValue::String("test".to_string());
        let num_val = PropertyValue::Number(1.23);
        let int_val = PropertyValue::Integer(456);
        let bool_val = PropertyValue::Boolean(true);
        let array_val = PropertyValue::Array(vec!["one".to_string(), "two".to_string()]);

        // Test successful conversions
        assert_eq!(String::try_from(&string_val).unwrap(), "test");
        assert_eq!(f64::try_from(&num_val).unwrap(), 1.23);
        assert_eq!(i64::try_from(&int_val).unwrap(), 456);
        assert_eq!(bool::try_from(&bool_val).unwrap(), true);
        assert_eq!(Vec::<String>::try_from(&array_val).unwrap(), vec!["one", "two"]);

        // Test failed conversions
        assert!(String::try_from(&bool_val).is_err());
        assert!(f64::try_from(&string_val).is_err());
        assert!(i64::try_from(&array_val).is_err());
        assert!(bool::try_from(&num_val).is_err());
    }

    #[test]
    fn test_value_accessors() {
        let string_val = PropertyValue::String("test".to_string());
        let num_val = PropertyValue::Number(1.23);
        let int_val = PropertyValue::Integer(456);
        let bool_val = PropertyValue::Boolean(true);
        let array_val = PropertyValue::Array(vec!["one".to_string(), "two".to_string()]);

        // Test optional accessors
        assert_eq!(string_val.as_string().unwrap(), "test");
        assert_eq!(num_val.as_number().unwrap(), 1.23);
        assert_eq!(int_val.as_integer().unwrap(), 456);
        assert_eq!(bool_val.as_boolean().unwrap(), true);
        assert_eq!(array_val.as_array().unwrap(), &["one", "two"]);

        // Test required accessors
        assert_eq!(string_val.require_string().unwrap(), "test");
        assert_eq!(num_val.require_number().unwrap(), 1.23);
        assert_eq!(int_val.require_integer().unwrap(), 456);
        assert_eq!(bool_val.require_boolean().unwrap(), true);
        assert_eq!(array_val.require_array().unwrap(), &["one", "two"]);

        // Test type mismatches
        assert!(string_val.as_number().is_none());
        assert!(num_val.as_boolean().is_none());
        assert!(bool_val.as_array().is_none());
        assert!(array_val.as_string().is_none());

        assert!(string_val.require_number().is_err());
        assert!(num_val.require_boolean().is_err());
        assert!(bool_val.require_array().is_err());
        assert!(array_val.require_string().is_err());
    }

    #[test]
    fn test_array_handling() {
        let empty = PropertyValue::array(vec![]);
        let single = PropertyValue::array(vec!["one".to_string()]);
        let multiple = PropertyValue::array(vec!["one".to_string(), "two".to_string()]);

        assert!(empty.is_array());
        assert!(single.is_array());
        assert!(multiple.is_array());

        assert_eq!(empty.array_values().unwrap().len(), 0);
        assert_eq!(single.array_values().unwrap().len(), 1);
        assert_eq!(multiple.array_values().unwrap().len(), 2);

        assert_eq!(single.array_values().unwrap()[0], "one");
        assert_eq!(multiple.array_values().unwrap()[1], "two");
    }

    #[test]
    fn test_single_value_parsing() {
        // Test automatic type inference in single()
        assert!(matches!(PropertyValue::single("123".to_string()), PropertyValue::Integer(123)));
        assert!(matches!(PropertyValue::single("1.23".to_string()), PropertyValue::Number(1.23)));
        assert!(matches!(PropertyValue::single("true".to_string()), PropertyValue::Boolean(true)));
        assert!(matches!(PropertyValue::single("simple_id".to_string()), PropertyValue::Identifier(_)));
        assert!(matches!(PropertyValue::single("\"quoted\"".to_string()), PropertyValue::String(_)));
    }
}