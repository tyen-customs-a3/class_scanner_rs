use std::collections::HashMap;
use std::path::{Path, PathBuf};
use lazy_static::lazy_static;
use regex::Regex;
use log::{debug, trace, warn, info};

use crate::error::{Error, ParserError, Result};
use crate::models::{ClassData, PropertyValue};
use super::{PropertyParser, ParserConfig};
use super::block_handler::BlockHandler;

pub struct ClassParser {
    property_parser: PropertyParser,
    block_handler: BlockHandler,
    current_file: Option<PathBuf>,
    current_addon: Option<String>,
    config: ParserConfig,
}

impl ClassParser {
    pub fn new() -> Self {
        debug!("Creating new ClassParser with default configuration");
        let config = ParserConfig::default();
        Self {
            property_parser: PropertyParser::new(),
            block_handler: BlockHandler::new(config),
            current_file: None,
            current_addon: None,
            config,
        }
    }

    pub fn with_config(config: ParserConfig) -> Self {
        debug!("Creating new ClassParser with custom configuration: {:?}", config);
        Self {
            property_parser: PropertyParser::new(),
            block_handler: BlockHandler::new(config),
            current_file: None,
            current_addon: None,
            config,
        }
    }

    pub fn set_current_file(&mut self, file: impl Into<PathBuf>) {
        let file_path = file.into();
        debug!("Setting current file to: {:?}", file_path);
        self.current_file = Some(file_path);
        self.current_addon = self.extract_addon_name(&self.current_file.as_ref().unwrap());
        if let Some(ref addon) = self.current_addon {
            debug!("Extracted addon name: {}", addon);
        }
    }

    pub fn parse_class_definitions(&mut self, content: &str) -> Result<HashMap<String, ClassData>> {
        info!("Starting to parse class definitions from content of length {}", content.len());
        let cleaned_content = self.block_handler.clean_code(content);
        let mut classes = HashMap::new();
        
        let mut pos = 0;
        while pos < cleaned_content.len() {
            match self.parse_next_class(&cleaned_content[pos..]) {
                Ok(Some((class_data, new_pos))) => {
                    debug!("Parsed class: {} at position {}", class_data.name, pos);
                    classes.insert(class_data.name.clone(), class_data);
                    pos += new_pos;
                }
                Ok(None) => break,
                Err(e) => {
                    warn!("Error parsing class at position {}: {}", pos, e);
                    break;
                }
            }
        }

        info!("Finished parsing {} classes", classes.len());
        Ok(classes)
    }

    pub fn parse_hierarchical(&mut self, content: &str) -> Result<Vec<ClassData>> {
        info!("Starting hierarchical class parsing");
        let cleaned_content = self.block_handler.clean_code(content);
        self.parse_hierarchical_classes(&cleaned_content)
    }

    fn parse_hierarchical_classes(&self, content: &str) -> Result<Vec<ClassData>> {
        lazy_static! {
            static ref CLASS_START: Regex = Regex::new(
                r"class\s+(\w+)(?:\s*:\s*(\w+))?\s*{"
            ).unwrap();
        }

        debug!("Parsing hierarchical classes from content of length {}", content.len());
        let mut classes = Vec::new();
        let mut pos = 0;
        
        while pos < content.len() {
            if let Some(cap) = CLASS_START.captures(&content[pos..]) {
                let class_name = cap[1].to_string();
                let parent = cap.get(2).map(|m| m.as_str().to_string()).unwrap_or_default();
                let match_len = cap[0].len();
                
                debug!("Found class '{}' with parent '{}'", class_name, parent);
                let block_start = pos + match_len - 1;
                if let Some((block_content, end_pos)) = self.block_handler.extract_block(&content[block_start..]) {
                    trace!("Extracted class block of length {}", block_content.len());
                    let inner_content = block_content[1..block_content.len()-1].trim();
                    
                    // Create class data
                    let mut class_data = ClassData::new(class_name.clone())
                        .with_parent(parent);
                    
                    if let Some(file) = &self.current_file {
                        class_data = class_data.with_source(file);
                    }
                    if let Some(addon) = &self.current_addon {
                        class_data = class_data.with_addon(addon);
                    }
                    
                    // Parse properties and nested classes
                    let cleaned_content = self.block_handler.clean_inner_block(inner_content);
                    debug!("Parsing properties for class '{}'", class_name);
                    class_data.properties = self.property_parser.parse_block_properties(&cleaned_content)?;
                    debug!("Parsing nested classes for class '{}'", class_name);
                    class_data.nested_classes = self.parse_hierarchical_classes(inner_content)?;
                    
                    classes.push(class_data);
                    pos += block_start + end_pos;
                } else {
                    warn!("Could not extract block for class '{}'", class_name);
                    pos += match_len;
                }
            } else {
                break;
            }
        }
        
        debug!("Found {} hierarchical classes", classes.len());
        Ok(classes)
    }

    fn parse_next_class(&self, content: &str) -> Result<Option<(ClassData, usize)>> {
        lazy_static! {
            static ref CLASS_PATTERN: Regex = Regex::new(
                r"class\s+(\w+)(?:\s*:\s*(\w+))?\s*({|;)"
            ).unwrap();
        }

        let captures = match CLASS_PATTERN.captures(content) {
            Some(cap) => cap,
            None => return Ok(None),
        };

        let class_name = captures.get(1)
            .ok_or_else(|| Error::Parser(ParserError::InvalidClass("Missing class name".to_string())))?
            .as_str();

        let parent_name = captures.get(2).map_or("", |m| m.as_str());
        let has_block = captures.get(3).map_or(false, |m| m.as_str() == "{");
        let class_start = captures.get(0).unwrap().end();
        
        let class_name = if !self.config.case_sensitive {
            class_name.to_lowercase()
        } else {
            class_name.to_string()
        };

        let mut class_data = ClassData::new(class_name)
            .with_parent(parent_name);
            
        if let Some(file) = &self.current_file {
            class_data = class_data.with_source(file);
        }
        if let Some(addon) = &self.current_addon {
            class_data = class_data.with_addon(addon);
        }

        if has_block {
            if let Some((block_content, block_end)) = self.block_handler.extract_block(&content[class_start..]) {
                // Parse properties
                let cleaned_block = self.block_handler.clean_inner_block(&block_content);
                class_data.properties = self.property_parser.parse_block_properties(&cleaned_block)?;

                // Parse nested classes
                class_data.nested_classes = self.parse_hierarchical_classes(&block_content)?;

                return Ok(Some((class_data, class_start + block_end)));
            }
        }

        Ok(Some((class_data, class_start)))
    }

    fn extract_addon_name(&self, path: &Path) -> Option<String> {
        trace!("Extracting addon name from path: {:?}", path);
        let addon = path.components()
            .find(|c| {
                if let std::path::Component::Normal(name) = c {
                    name.to_string_lossy().starts_with('@')
                } else {
                    false
                }
            })
            .map(|c| c.as_os_str().to_string_lossy().into_owned());
        
        if let Some(ref name) = addon {
            debug!("Found addon name: {}", name);
        } else {
            debug!("No addon name found in path");
        }
        
        addon
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_class_parsing() -> Result<()> {
        let mut parser = ClassParser::new();
        let content = r#"
            class SimpleClass {
                value = "test";
                number = 42;
                array[] = {"one", "two", "three"};
            };
        "#;

        let classes = parser.parse_class_definitions(content)?;
        assert_eq!(classes.len(), 1);
        
        let class = classes.get("SimpleClass").unwrap();
        assert_eq!(class.name, "SimpleClass");
        assert_eq!(class.properties.len(), 3);
        Ok(())
    }

    #[test]
    fn test_nested_classes() -> Result<()> {
        let mut parser = ClassParser::new();
        let content = r#"
            class ParentClass {
                class NestedClass {
                    value = "nested";
                };
                parentValue = "parent";
            };
        "#;

        let classes = parser.parse_hierarchical(content)?;
        assert_eq!(classes.len(), 1);
        
        let parent = &classes[0];
        assert_eq!(parent.name, "ParentClass");
        assert_eq!(parent.nested_classes.len(), 1);
        
        let nested = &parent.nested_classes[0];
        assert_eq!(nested.name, "NestedClass");
        Ok(())
    }
}