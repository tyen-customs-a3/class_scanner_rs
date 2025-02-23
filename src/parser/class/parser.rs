use std::collections::HashMap;
use std::path::PathBuf;
use log::{debug, info, warn, trace};

use crate::models::{Error, Result};
use crate::models::ClassData;
use crate::parser::property::PropertyParser;
use crate::parser::ParserConfig;
use crate::parser::block::{BlockHandler, BlockCleaner};
use super::{ClassParsing, ClassParsingContext};

pub struct ClassParser {
    property_parser: PropertyParser,
    block_handler: BlockHandler,
    current_file: Option<PathBuf>,
    current_addon: Option<String>,
    config: ParserConfig,
}

impl ClassParser {
    pub fn new() -> Self {
        info!("Creating new ClassParser with default configuration");
        let config = ParserConfig::default();
        Self::with_config(config)
    }

    pub fn with_config(config: ParserConfig) -> Self {
        info!("Creating new ClassParser with custom configuration: {:?}", &config);
        let block_handler = BlockHandler::new(config.clone());
        Self {
            property_parser: PropertyParser::new(),
            block_handler,
            current_file: None,
            current_addon: None,
            config,
        }
    }

    pub fn set_current_file(&mut self, file: impl Into<PathBuf>) {
        let file_path = file.into();
        info!("Setting current file to: {:?}", file_path);
        self.current_file = Some(file_path);
        self.current_addon = super::utils::extract_addon_name(self.current_file.as_ref().unwrap());
        if let Some(ref addon) = self.current_addon {
            debug!("Extracted addon name: {}", addon);
        } else {
            debug!("No addon name found in current file path");
        }
    }

    pub fn parse_class_definitions(&mut self, content: &str) -> Result<HashMap<String, ClassData>> {
        info!("Starting class definitions parsing for content of length {}", content.len());
        trace!("Using parser config: {:?}", self.config);
        
        let cleaned_content = self.block_handler.clean_code(content);
        trace!("Cleaned content length: {}", cleaned_content.len());
        
        let mut classes = HashMap::new();
        let mut pos = 0;
        
        let context = ClassParsingContext {
            current_file: self.current_file.clone(),
            current_addon: self.current_addon.clone(),
            case_sensitive: self.config.case_sensitive,
        };

        let extractor = super::class_extractor::ClassExtractor::new(
            &self.block_handler,
            &self.property_parser,
            &self.config
        );

        while pos < cleaned_content.len() {
            match extractor.parse_class(&cleaned_content[pos..], &context)? {
                Some((class_data, consumed)) => {
                    debug!("Parsed class '{}' at position {}", class_data.name, pos);
                    trace!("Class '{}' details: {} properties, {} nested classes", 
                        class_data.name, 
                        class_data.properties.len(),
                        class_data.nested_classes.len()
                    );
                    
                    let parent = &class_data.parent;
                    if !parent.is_empty() {
                        debug!("Class '{}' extends '{}'", class_data.name, parent);
                    }
                    
                    classes.insert(class_data.name.clone(), class_data);
                    pos += consumed;
                }
                None => {
                    trace!("No more classes found at position {}", pos);
                    break;
                }
            }
        }

        info!("Completed parsing {} class definitions", classes.len());
        Ok(classes)
    }

    pub fn parse_hierarchical(&self, content: &str) -> Result<Vec<ClassData>> {
        info!("Starting hierarchical parsing of content length {}", content.len());
        let cleaned_content = self.block_handler.clean_code(content);
        trace!("Cleaned content length: {}", cleaned_content.len());
        
        super::hierarchy::parse_hierarchical_classes(
            &cleaned_content,
            &self.block_handler,
            &self.property_parser,
            &self.current_file,
            &self.current_addon,
            &self.config
        )
    }
}
