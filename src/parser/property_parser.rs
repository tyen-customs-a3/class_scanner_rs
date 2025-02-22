use std::collections::HashMap;
use lazy_static::lazy_static;
use regex::Regex;
use log::{debug, trace, warn};

use crate::error::{Error, ParserError, PropertyParserError, Result};
use crate::models::{PropertyValue, PropertyValueType};
use super::{ParserConfig, PropertyToken, PropertyTokenType};
use super::value_parser::ValueParser;

pub struct PropertyParser {
    property_pattern: Regex,
    empty_array_pattern: Regex,
    array_pattern: Regex,
    config: ParserConfig,
    value_parser: ValueParser,
}

impl Default for PropertyParser {
    fn default() -> Self {
        Self::new()
    }
}

impl PropertyParser {
    pub fn new() -> Self {
        debug!("Creating new PropertyParser with default configuration");
        let config = ParserConfig::default();
        Self {
            property_pattern: Regex::new(r"(\w+)(?:\[\])?\s*=\s*([^;]+);").unwrap(),
            empty_array_pattern: Regex::new(r"^\s*{\s*}\s*$").unwrap(),
            array_pattern: Regex::new(r"^\s*{(.*)}\s*$").unwrap(),
            config: config.clone(),
            value_parser: ValueParser::new(config),
        }
    }

    pub fn with_config(config: ParserConfig) -> Self {
        debug!("Creating new PropertyParser with custom configuration: {:?}", config);
        Self {
            property_pattern: Regex::new(r"(\w+)(?:\[\])?\s*=\s*([^;]+);").unwrap(),
            empty_array_pattern: Regex::new(r"^\s*{\s*}\s*$").unwrap(),
            array_pattern: Regex::new(r"^\s*{(.*)}\s*$").unwrap(),
            config: config.clone(),
            value_parser: ValueParser::new(config),
        }
    }

    pub fn parse_block_properties(&self, block: &str) -> Result<HashMap<String, PropertyValue>> {
        debug!("Starting to parse block properties, block length: {}", block.len());
        let mut properties = HashMap::new();
        let cleaned_block = self.preprocess_block(block)?;

        // Parse array properties first
        if let Some(caps) = self.array_pattern.captures(&cleaned_block) {
            debug!("Found array pattern in block");
            for cap in self.property_pattern.captures_iter(&cleaned_block) {
                let name = cap[1].to_string();
                let value = cap[2].to_string();
                
                if value.starts_with('{') && value.ends_with('}') {
                    debug!("Processing array property: {}", name);
                    let array_values = self.parse_array_content(&value)?;
                    self.insert_property(&mut properties, name.clone(), PropertyValue::new(name)
                        .with_value(value)
                        .with_array_values(array_values));
                }
            }
        }

        // Parse simple properties
        for cap in self.property_pattern.captures_iter(&cleaned_block) {
            let name = cap[1].to_string();
            if !self.has_property(&properties, &name) {
                let value = cap[2].to_string();
                debug!("Processing simple property: {} = {}", name, value);
                let (value_type, raw_value) = self.value_parser.detect_type(&value)?;
                
                self.insert_property(&mut properties, name.clone(), PropertyValue::new(name)
                    .with_value(raw_value)
                    .with_type(value_type));
            }
        }

        debug!("Finished parsing block properties, found {} properties", properties.len());
        Ok(properties)
    }

    fn insert_property(&self, properties: &mut HashMap<String, PropertyValue>, name: String, value: PropertyValue) {
        let key = if self.config.case_sensitive {
            name.clone()
        } else {
            name.to_lowercase()
        };
        trace!("Inserting property: {} (key: {})", name, key);
        properties.insert(key, value);
    }

    fn has_property(&self, properties: &HashMap<String, PropertyValue>, name: &str) -> bool {
        let key = if self.config.case_sensitive {
            name.to_string()
        } else {
            name.to_lowercase()
        };
        properties.contains_key(&key)
    }

    fn preprocess_block(&self, block: &str) -> Result<String> {
        trace!("Preprocessing block of length {}", block.len());
        // Remove comments and normalize whitespace
        let result = block.lines()
            .filter_map(|line| {
                let line = line.trim();
                if line.is_empty() || line.starts_with("//") {
                    trace!("Filtered out line: {}", line);
                    None
                } else {
                    Some(Ok(line.to_string()))
                }
            })
            .collect::<Result<Vec<_>>>()?
            .join(" ");

        trace!("Preprocessed block length: {}", result.len());
        Ok(result)
    }

    fn parse_array_content(&self, content: &str) -> Result<Vec<String>> {
        debug!("Parsing array content: {}", content);
        if self.empty_array_pattern.is_match(content) {
            debug!("Found empty array");
            return Ok(Vec::new());
        }

        if let Some(cap) = self.array_pattern.captures(content) {
            let inner = cap[1].trim();
            debug!("Found array content: {}", inner);
            Ok(self.split_array_items(inner))
        } else {
            warn!("Invalid array content: {}", content);
            Err(Error::Parser(ParserError::PropertyError(
                PropertyParserError::InvalidProperty(format!("Invalid array content: {}", content))
            )))
        }
    }

    fn split_array_items(&self, content: &str) -> Vec<String> {
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

        debug!("Split array into {} items", items.len());
        items
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_property() -> Result<()> {
        let parser = PropertyParser::new();
        let input = "value = 42;";
        let props = parser.parse_block_properties(input)?;
        
        assert_eq!(props.len(), 1);
        let prop = props.get("value").unwrap();
        assert_eq!(prop.raw_value, "42");
        assert_eq!(prop.value_type, Some(PropertyValueType::Number));
        Ok(())
    }

    #[test]
    fn test_parse_array_property() -> Result<()> {
        let parser = PropertyParser::new();
        let input = r#"values[] = {"one", "two", "three"};"#;
        let props = parser.parse_block_properties(input)?;
        
        assert_eq!(props.len(), 1);
        let prop = props.get("values").unwrap();
        assert!(prop.is_array);
        assert_eq!(prop.array_values.len(), 3);
        Ok(())
    }

    #[test]
    fn test_case_sensitivity() -> Result<()> {
        let parser = PropertyParser::with_config(ParserConfig {
            case_sensitive: false,
            ..Default::default()
        });
        
        let input = r#"
            Value = "test";
            VALUE = "override";
        "#;
        
        let props = parser.parse_block_properties(input)?;
        assert_eq!(props.len(), 1);
        assert_eq!(props.get("value").unwrap().raw_value, "override");
        Ok(())
    }
}