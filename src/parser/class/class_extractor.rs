use lazy_static::lazy_static;
use log::{debug, trace, warn};
use regex::Regex;
use std::path::PathBuf;

use crate::models::{Error, Result, ClassData};
use crate::parser::property::PropertyParser;
use crate::parser::{ParserConfig, patterns};
use crate::parser::block::{BlockHandler, BlockCleaner};
use super::{ClassParsing, ClassParsingContext};

pub struct ClassExtractor<'a> {
    block_handler: &'a BlockHandler,
    property_parser: &'a PropertyParser,
    config: &'a ParserConfig,
}

impl<'a> ClassExtractor<'a> {
    pub fn new(
        block_handler: &'a BlockHandler,
        property_parser: &'a PropertyParser,
        config: &'a ParserConfig,
    ) -> Self {
        Self {
            block_handler,
            property_parser,
            config,
        }
    }
}

impl<'a> ClassParsing for ClassExtractor<'a> {
    fn parse_class(&self, content: &str, context: &ClassParsingContext) -> Result<Option<(ClassData, usize)>> {
        trace!("Attempting to parse class from content of length {}", content.len());
        
        let captures = match patterns::CLASS_PATTERN.captures(content) {
            Some(cap) => cap,
            None => {
                trace!("No class pattern match found");
                return Ok(None);
            }
        };

        let class_name = captures.get(1)
            .ok_or_else(|| {
                warn!("Found class pattern but name capture was missing");
                Error::Validation("Missing class name".to_string())
            })?
            .as_str();

        let parent_name = captures.get(2).map_or("", |m| m.as_str());
        let declaration_end = captures.get(0).unwrap().end();
        let remaining = content[declaration_end..].trim_start();
        
        let has_block = remaining.starts_with('{');
        let has_semicolon = remaining.starts_with(';');
        
        if !has_block && !has_semicolon {
            trace!("Invalid class declaration - missing block or semicolon");
            return Ok(None);
        }
        
        debug!("Found class '{}' with parent '{}'", class_name, parent_name);
        
        let class_name = if !context.case_sensitive {
            debug!("Converting class name to lowercase due to case-insensitive setting");
            class_name.to_lowercase()
        } else {
            class_name.to_string()
        };

        let mut class_data = ClassData::new(class_name.clone())
            .with_parent(if parent_name.is_empty() { "" } else { parent_name });
            
        if let Some(file) = &context.current_file {
            debug!("Setting source file for class '{}': {:?}", class_name, file);
            class_data = class_data.with_source(file);
        }
        if let Some(addon) = &context.current_addon {
            debug!("Setting addon for class '{}': {}", class_name, addon);
            class_data = class_data.with_addon(addon);
        }

        if has_block {
            debug!("Class '{}' has a block, extracting contents", class_name);
            let block_start = declaration_end + remaining.find('{').unwrap();
            if let Some((block_content, block_end)) = self.block_handler.extract_block(&content[block_start..]) {
                trace!("Extracted block of length {} for class '{}'", block_content.len(), class_name);
                let cleaned_block = self.block_handler.clean_inner_block(&block_content);
                
                debug!("Parsing properties for class '{}'", class_name);
                class_data.properties = self.property_parser.parse_block_properties(&cleaned_block)?;
                debug!("Found {} properties for class '{}'", class_data.properties.len(), class_name);

                debug!("Looking for nested classes in '{}'", class_name);
                let nested_context = ClassParsingContext {
                    current_file: context.current_file.clone(),
                    current_addon: context.current_addon.clone(),
                    case_sensitive: context.case_sensitive,
                };

                let mut current_pos = 0;
                let mut nested_classes = Vec::new();

                while current_pos < cleaned_block.len() {
                    let remaining = &cleaned_block[current_pos..];
                    match self.parse_class(remaining, &nested_context)? {
                        Some((nested_class, consumed)) => {
                            debug!("Found nested class '{}' in '{}'", nested_class.name, class_name);
                            nested_classes.push(nested_class);
                            current_pos += consumed;
                        }
                        None => {
                            if current_pos + 1 < cleaned_block.len() {
                                current_pos += 1;
                            } else {
                                break;
                            }
                        }
                    }
                }

                debug!("Found {} nested classes in '{}'", nested_classes.len(), class_name);
                class_data.nested_classes = nested_classes;

                debug!("Completed parsing class '{}' with block", class_name);
                return Ok(Some((class_data, block_start + block_end)));
            } else {
                warn!("Class '{}' indicated block but failed to extract it", class_name);
            }
        } else {
            debug!("Class '{}' has no block content", class_name);
            let end_pos = declaration_end + remaining.find(';').unwrap() + 1;
            return Ok(Some((class_data, end_pos)));
        }

        debug!("Completed parsing class '{}' without block", class_name);
        Ok(Some((class_data, declaration_end)))
    }
}
