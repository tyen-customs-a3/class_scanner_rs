pub mod parser;
mod class_extractor;
mod hierarchy;
mod utils;

use crate::models::{ClassData, Result};
use std::path::PathBuf;

/// Trait defining the common interface for class parsing
pub trait ClassParsing {
    fn parse_class(&self, content: &str, context: &ClassParsingContext) -> Result<Option<(ClassData, usize)>>;
}

/// Shared context for class parsing operations
#[derive(Clone, Debug)]
pub struct ClassParsingContext {
    pub current_file: Option<PathBuf>,
    pub current_addon: Option<String>,
    pub case_sensitive: bool,
}

pub use parser::ClassParser;

#[cfg(test)]
pub mod tests;