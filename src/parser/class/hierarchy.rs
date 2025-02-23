use log::{debug, trace, info};
use std::path::PathBuf;

use crate::models::{Result, ClassData};
use crate::parser::property::PropertyParser;
use crate::parser::{ParserConfig, patterns};
use crate::parser::block::{BlockHandler, BlockCleaner};
use super::{ClassParsing, ClassParsingContext};

pub fn parse_hierarchical_classes<'a>(
    content: &str,
    block_handler: &'a BlockHandler,
    property_parser: &'a PropertyParser,
    current_file: &Option<PathBuf>,
    current_addon: &Option<String>,
    config: &'a ParserConfig,
) -> Result<Vec<ClassData>> {
    info!("Starting hierarchical class parsing of content length {}", content.len());
    if let Some(file) = current_file {
        debug!("Parsing from file: {:?}", file);
    }
    if let Some(addon) = current_addon {
        debug!("Using addon context: {}", addon);
    }

    let mut classes = Vec::new();
    let mut pos = 0;

    let context = ClassParsingContext {
        current_file: current_file.clone(),
        current_addon: current_addon.clone(),
        case_sensitive: config.case_sensitive,
    };

    trace!("Created parsing context with case_sensitive: {}", config.case_sensitive);
    let extractor = super::class_extractor::ClassExtractor::new(
        block_handler,
        property_parser,
        config
    );

    while pos < content.len() {
        let mut found = false;
        if let Some((class_data, consumed)) = extractor.parse_class(&content[pos..], &context)? {
            debug!("Successfully parsed class '{}' at position {}", class_data.name, pos);
            trace!("Class '{}' has {} properties and {} nested classes", 
                class_data.name, 
                class_data.properties.len(),
                class_data.nested_classes.len()
            );
            classes.push(class_data);
            pos += consumed;
            found = true;
        }
        
        if !found {
            trace!("No class found at position {}, searching for next class pattern", pos);
            if let Some(next_class) = patterns::CLASS_PATTERN.find(&content[pos..]) {
                let skip = next_class.start();
                trace!("Skipping {} characters to next potential class", skip);
                pos += skip;
            } else {
                debug!("No more class patterns found, ending parse");
                break;
            }
        }
    }
    
    info!("Completed hierarchical parsing. Found {} total classes", classes.len());
    Ok(classes)
}
