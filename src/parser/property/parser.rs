use std::collections::HashMap;
use log::{debug, trace};

use crate::parser::ParserConfig;
use crate::parser::patterns;
use super::{PropertyValue, PropertyKey, PropertyValidator};
use crate::models::error::{Error, Result};

pub struct PropertyParser {
    config: ParserConfig,
    validators: HashMap<String, PropertyValidator>,
}

impl Default for PropertyParser {
    fn default() -> Self {
        Self::new()
    }
}

impl PropertyParser {
    pub fn new() -> Self {
        Self {
            config: ParserConfig::default(),
            validators: HashMap::new(),
        }
    }

    pub fn with_config(config: ParserConfig) -> Self {
        Self {
            config,
            validators: HashMap::new(),
        }
    }

    pub fn register_validator(&mut self, property_name: impl Into<String>, validator: PropertyValidator) {
        self.validators.insert(property_name.into(), validator);
    }

    pub fn parse_block_properties(&self, block: &str) -> Result<HashMap<String, PropertyValue>> {
        let mut properties = HashMap::new();
        let cleaned_block = self.preprocess_block(block)?;
        
        // Find and process all property matches
        for cap in patterns::PROPERTY_PATTERN.captures_iter(&cleaned_block) {
            let name = cap[1].to_string();
            let value_str = cap[2].trim();
            
            // Skip if this looks like a nested class
            if value_str.starts_with("class") {
                continue;
            }

            // Determine if it's an array property by checking if name ends with []
            let is_array = name.ends_with("[]");
            let property_name = if !is_array && value_str.starts_with('{') {
                // If value looks like an array but name doesn't have [], add it
                format!("{}[]", name)
            } else {
                name
            };
            
            // Parse the value
            let mut value_str = value_str.to_string();
            if is_array && !value_str.starts_with('{') {
                // Wrap single values in array braces if needed
                value_str = format!("{{{}}}", value_str);
            }
            
            if let Ok(value) = PropertyValue::parse(&value_str, self.config.case_sensitive) {
                if let Some(validator) = self.validators.get(&property_name) {
                    validator.validate(&value)?;
                }
                properties.insert(property_name, value);
            }
        }
        
        Ok(properties)
    }

    fn preprocess_block(&self, block: &str) -> Result<String> {
        // Remove comments and normalize whitespace
        let mut result = String::new();
        let mut chars = block.chars().peekable();
        let mut in_block_comment = false;
        let mut in_line_comment = false;
        let mut in_string = false;
        
        while let Some(c) = chars.next() {
            match c {
                '"' if !in_block_comment && !in_line_comment => {
                    in_string = !in_string;
                    result.push(c);
                },
                '/' if !in_string => {
                    if let Some(&next) = chars.peek() {
                        match next {
                            '/' if !in_block_comment => {
                                in_line_comment = true;
                                chars.next();
                            },
                            '*' if !in_line_comment => {
                                in_block_comment = true;
                                chars.next();
                            },
                            _ => result.push(c)
                        }
                    } else {
                        result.push(c);
                    }
                },
                '*' if !in_string && in_block_comment => {
                    if let Some(&'/') = chars.peek() {
                        in_block_comment = false;
                        chars.next();
                    }
                },
                '\n' if in_line_comment => {
                    in_line_comment = false;
                    result.push(c);
                },
                _ if !in_block_comment && !in_line_comment => {
                    result.push(c);
                },
                _ => {}
            }
        }
        
        Ok(result)
    }
}