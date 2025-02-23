use serde::{Deserialize, Serialize};
use crate::models::error::{Error, PropertyTypeMismatchError, Result};
use crate::parser::patterns;
use log::{trace, debug};
use std::path::{PathBuf, Component};

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
        trace!("Parsing property value: {:?}", value);
        
        // Handle empty arrays
        if patterns::EMPTY_ARRAY_PATTERN.is_match(value) {
            trace!("Matched empty array pattern");
            return Ok(Self::Array(Vec::new()));
        }
        
        // Handle quoted strings
        if value.starts_with('"') && value.ends_with('"') {
            trace!("Processing quoted string");
            let inner = &value[1..value.len()-1];
            let result = Self::parse_string(inner);
            trace!("Parsed quoted string result: {:?}", result);
            return Ok(Self::String(result));
        }
        
        // Handle arrays
        if value.starts_with('{') && value.ends_with('}') {
            trace!("Processing array value");
            let inner = &value[1..value.len()-1].trim();
            if inner.is_empty() {
                trace!("Found empty array");
                return Ok(Self::Array(Vec::new()));
            }
            
            let result = Self::parse_array(inner);
            trace!("Parsed array result: {:?}", result);
            return Ok(Self::Array(result));
        }
        
        // Handle boolean values
        if patterns::BOOLEAN_PATTERN.is_match(value) {
            let bool_val = value.to_lowercase() == "true";
            trace!("Parsed boolean value: {}", bool_val);
            return Ok(Self::Boolean(bool_val));
        }
        
        // Handle numeric values
        if patterns::NUMBER_PATTERN.is_match(value) {
            if value.contains('.') {
                let num: f64 = value.parse().unwrap();
                trace!("Parsed float value: {}", num);
                return Ok(Self::Number(num));
            } else {
                let num: i64 = value.parse().unwrap();
                trace!("Parsed integer value: {}", num);
                return Ok(Self::Integer(num));
            }
        }

        // Handle paths (both raw and forward-slash paths)
        if value.starts_with('\\') || (!value.contains('"') && value.contains('/')) {
            let normalized = Self::normalize_path(value);
            trace!("Normalized path: {:?}", normalized);
            return Ok(Self::String(normalized));
        }

        // Treat as identifier
        let ident = if case_sensitive {
            value.to_string()
        } else {
            value.to_lowercase()
        };
        trace!("Created identifier: {:?}", ident);
        Ok(Self::Identifier(ident))
    }

    // Core string parsing state machine
    fn parse_string(input: &str) -> String {
        trace!("Starting string parse for input: {:?}", input);
        #[derive(Debug, PartialEq)]
        enum State {
            Normal,
            Escaped,
        }

        // If the input starts with a backslash, treat it as a raw path
        if input.starts_with('\\') {
            return input.to_string();
        }

        let mut result = String::with_capacity(input.len());
        let mut state = State::Normal;
        
        for (i, c) in input.chars().enumerate() {
            trace!("Processing char at pos {}: {:?}, state: {:?}", i, c, state);
            match state {
                State::Normal => {
                    match c {
                        '\\' => {
                            trace!("Entering escaped state");
                            state = State::Escaped;
                        },
                        '/' => {
                            trace!("Converting forward slash to backslash");
                            result.push('\\');
                        },
                        _ => {
                            trace!("Adding normal char: {:?}", c);
                            result.push(c);
                        }
                    }
                },
                State::Escaped => {
                    match c {
                        'n' => result.push('\n'),
                        'r' => result.push('\r'),
                        't' => result.push('\t'),
                        '"' => result.push('"'),
                        '\\' => result.push('\\'),
                        _ => {
                            result.push('\\');
                            result.push(c);
                        }
                    }
                    state = State::Normal;
                }
            }
        }
        
        if state == State::Escaped {
            result.push('\\');
        }
        
        trace!("Final parsed string result: {:?}", result);
        result
    }

    fn normalize_path(value: &str) -> String {
        // Special case: if it's not a path but contains special chars, return as is
        if value.starts_with('_') || value.starts_with('-') || value.starts_with('.') {
            return value.to_string();
        }

        // For raw paths starting with backslash, normalize but preserve leading backslash
        if value.starts_with('\\') {
            let cleaned = value.replace("\\\\", "\\");
            if cleaned != "\\" {
                return cleaned;
            }
        }

        // Split by either slash type and recombine with single backslashes
        let parts: Vec<&str> = value.split(|c| c == '/' || c == '\\')
            .filter(|s| !s.is_empty())
            .collect();
        
        if parts.is_empty() {
            return value.to_string();
        }

        // Reconstruct with proper separators
        if value.starts_with('\\') {
            format!("\\{}", parts.join("\\"))
        } else {
            parts.join("\\")
        }
    }

    fn parse_array(content: &str) -> Vec<String> {
        trace!("Starting array parse for content: {:?}", content);
        #[derive(Debug, PartialEq)]
        enum State {
            Normal,
            InQuotes,
            Escaped,
        }

        let mut items = Vec::new();
        let mut current = String::new();
        let mut state = State::Normal;
        let mut depth = 0;
        
        let mut chars = content.chars().peekable();
        while let Some(c) = chars.next() {
            trace!("Processing array char: {:?}, state: {:?}, depth: {}", c, state, depth);
            match state {
                State::Normal => {
                    match c {
                        '"' => {
                            trace!("Entering quoted state");
                            state = State::InQuotes;
                            current.push(c);
                        },
                        '{' => {
                            depth += 1;
                            trace!("Increasing depth to: {}", depth);
                            current.push(c);
                        },
                        '}' => {
                            depth -= 1;
                            trace!("Decreasing depth to: {}", depth);
                            current.push(c);
                        },
                        ',' if depth == 0 => {
                            if !current.is_empty() {
                                trace!("Adding array item: {:?}", current);
                                items.push(current.trim().to_string());
                                current.clear();
                            }
                        },
                        _ => current.push(c),
                    }
                },
                State::InQuotes => {
                    match c {
                        '"' => {
                            trace!("Exiting quoted state");
                            state = State::Normal;
                            current.push(c);
                        },
                        '\\' => {
                            trace!("Entering escaped state in quotes");
                            state = State::Escaped;
                            current.push(c);
                        },
                        _ => current.push(c),
                    }
                },
                State::Escaped => {
                    trace!("Processing escaped char in quotes: {:?}", c);
                    current.push(c);
                    state = State::InQuotes;
                }
            }
        }
        
        if !current.is_empty() {
            trace!("Adding final array item: {:?}", current);
            items.push(current.trim().to_string());
        }
        
        let processed: Vec<String> = items.into_iter()
            .map(|item| {
                let item = item.trim();
                if item.starts_with('"') && item.ends_with('"') {
                    trace!("Processing quoted array item: {:?}", item);
                    Self::parse_string(&item[1..item.len()-1])
                } else {
                    trace!("Using raw array item: {:?}", item);
                    item.to_string()
                }
            })
            .collect();
        
        trace!("Final array parse result: {:?}", processed);
        processed
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

    // Helper function to determine if a string needs escaping
    pub(crate) fn needs_escaping(s: &str) -> bool {
        s.contains('\\') || s.contains('"') || s.contains('\n') || 
        s.contains('\r') || s.contains('\t')
    }

    // Helper to escape a string properly
    pub(crate) fn escape_string(s: &str) -> String {
        if !Self::needs_escaping(s) {
            return s.to_string();
        }

        let mut result = String::with_capacity(s.len() * 2);
        for c in s.chars() {
            match c {
                '\\' => result.push_str("\\\\"),
                '"' => result.push_str("\\\""),
                '\n' => result.push_str("\\n"),
                '\r' => result.push_str("\\r"),
                '\t' => result.push_str("\\t"),
                _ => result.push(c),
            }
        }
        result
    }

    // Helper to format a value as a string with proper escaping
    pub fn to_string_escaped(&self) -> String {
        match self {
            Self::String(s) | Self::Identifier(s) => {
                if Self::needs_escaping(s) {
                    format!("\"{}\"", Self::escape_string(s))
                } else {
                    s.clone()
                }
            },
            Self::Number(n) => n.to_string(),
            Self::Integer(i) => i.to_string(),
            Self::Boolean(b) => b.to_string(),
            Self::Array(arr) => {
                if arr.is_empty() {
                    "{}".to_string()
                } else {
                    let items: Vec<String> = arr.iter()
                        .map(|s| {
                            if Self::needs_escaping(s) {
                                // First escape quotes normally
                                let escaped = s.replace("\"", "\\\"");
                                // Then escape the whole string for array context
                                format!("\\\"{}\\\"", escaped)
                            } else {
                                s.clone()
                            }
                        })
                        .collect();
                    format!("{{{}}}", items.join(", "))
                }
            }
        }
    }

    // Helper to convert path separators based on target OS
    #[cfg(windows)]
    fn normalize_path_separators(path: &str) -> String {
        path.replace('/', "\\")
    }

    #[cfg(not(windows))]
    fn normalize_path_separators(path: &str) -> String {
        path.replace('\\', "/")
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