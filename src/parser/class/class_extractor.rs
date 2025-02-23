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
        
        let captures = match patterns::CLASS_PATTERN.captures(content) {
            Some(cap) => cap,
            None => {
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
            return Ok(None);
        }
        
        trace!("Found class '{}' with parent '{}'", class_name, parent_name);
        
        // When case-insensitive, convert names to lowercase for consistent comparison
        let (class_name, parent_name) = if !context.case_sensitive {
            trace!("Converting names to lowercase for case-insensitive mode");
            (class_name.to_lowercase(), 
             if parent_name.is_empty() { String::new() } else { parent_name.to_lowercase() })
        } else {
            (class_name.to_string(),
             if parent_name.is_empty() { String::new() } else { parent_name.to_string() })
        };

        let mut class_data = ClassData::new(class_name)
            .with_parent(&parent_name);
            
        if let Some(file) = &context.current_file {
            class_data = class_data.with_source(file);
        }
        if let Some(addon) = &context.current_addon {
            class_data = class_data.with_addon(addon);
        }

        if has_block {
            let block_start = declaration_end + remaining.find('{').unwrap();
            if let Some((block_content, block_end)) = self.block_handler.extract_block(&content[block_start..]) {
                let cleaned_block = self.block_handler.clean_inner_block(&block_content);
                
                class_data.properties = self.property_parser.parse_block_properties(&cleaned_block)?;

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

                class_data.nested_classes = nested_classes;

                return Ok(Some((class_data, block_start + block_end)));
            }
        } else {
            let end_pos = declaration_end + remaining.find(';').unwrap() + 1;
            return Ok(Some((class_data, end_pos)));
        }

        Ok(Some((class_data, declaration_end)))
    }
}
