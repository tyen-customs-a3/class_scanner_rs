use std::collections::HashMap;
use log::{debug, trace};

use crate::models::properties::{PropertyValue, PropertyKey, PropertyValidator};
use crate::models::error::{Error, Result};
use super::ParserConfig;
use super::patterns;

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
        debug!("Creating new PropertyParser with default configuration");
        Self {
            config: ParserConfig::default(),
            validators: HashMap::new(),
        }
    }

    pub fn with_config(config: ParserConfig) -> Self {
        debug!("Creating new PropertyParser with custom configuration: {:?}", config);
        Self {
            config,
            validators: HashMap::new(),
        }
    }

    pub fn register_validator(&mut self, property_name: impl Into<String>, validator: PropertyValidator) {
        self.validators.insert(property_name.into(), validator);
    }

    pub fn parse_block_properties(&self, block: &str) -> Result<HashMap<String, PropertyValue>> {
        debug!("Starting to parse block properties, block length: {}", block.len());
        let mut properties = HashMap::new();
        let cleaned_block = self.preprocess_block(block)?;

        for cap in patterns::PROPERTY_PATTERN.captures_iter(&cleaned_block) {
            let mut name = cap[1].to_string();
            let value_str = cap[2].trim().to_string();
            
            // Handle array properties by appending [] to the name
            if cap[0].contains("[]") {
                name.push_str("[]");
            }
            
            debug!("Processing property: {} = {}", name, value_str);
            let key = PropertyKey::new(name.clone());
            let value = PropertyValue::parse(&value_str, self.config.case_sensitive)?;

            // Validate if a validator exists for this property
            if let Some(validator) = self.validators.get(&name) {
                validator.validate(&value)?;
            }

            // Convert PropertyKey to String for storage
            properties.insert(key.to_string(), value);
        }

        Ok(properties)
    }

    fn preprocess_block(&self, block: &str) -> Result<String> {
        trace!("Preprocessing block of length {}", block.len());
        
        // Remove comments and normalize whitespace
        let mut result = String::new();
        let mut in_block_comment = false;
        let mut last_char = None;
        
        let mut chars = block.chars().peekable();
        while let Some(c) = chars.next() {
            match c {
                '/' if chars.peek() == Some(&'/') => {
                    // Skip until newline
                    while let Some(ch) = chars.next() {
                        if ch == '\n' {
                            result.push('\n');
                            break;
                        }
                    }
                }
                '/' if chars.peek() == Some(&'*') => {
                    in_block_comment = true;
                    chars.next(); // Skip *
                }
                '*' if in_block_comment && chars.peek() == Some(&'/') => {
                    in_block_comment = false;
                    chars.next(); // Skip /
                }
                _ if !in_block_comment => {
                    if c.is_whitespace() {
                        if last_char != Some(' ') {
                            result.push(' ');
                        }
                    } else {
                        result.push(c);
                    }
                }
                _ => {}
            }
            last_char = Some(c);
        }

        trace!("Preprocessed block length: {}", result.len());
        Ok(result)
    }
}
