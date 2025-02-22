use lazy_static::lazy_static;
use log::{trace, warn};
use regex::Regex;

use super::ParserConfig;

pub struct BlockHandler {
    config: ParserConfig,
}

impl BlockHandler {
    pub fn new(config: ParserConfig) -> Self {
        Self { config }
    }

    pub fn extract_block(&self, content: &str) -> Option<(String, usize)> {
        trace!("Extracting class block from content of length {}", content.len());
        let mut depth = 0;
        let mut in_string = false;
        let mut string_char = None;
        let mut block = String::new();
        let mut pos = 0;

        for (idx, ch) in content.chars().enumerate() {
            if depth > self.config.max_depth {
                warn!("Exceeded maximum nesting depth of {}", self.config.max_depth);
                return None;
            }
            
            match ch {
                '"' | '\'' if !in_string => {
                    in_string = true;
                    string_char = Some(ch);
                }
                ch if Some(ch) == string_char && !in_string => {
                    in_string = false;
                    string_char = None;
                }
                '{' if !in_string => {
                    depth += 1;
                    trace!("Increased block depth to {}", depth);
                }
                '}' if !in_string => {
                    depth -= 1;
                    trace!("Decreased block depth to {}", depth);
                    if depth == 0 {
                        block.push(ch);
                        pos = idx + 1;
                        break;
                    }
                }
                _ => {}
            }
            block.push(ch);
        }

        if depth == 0 && (!block.is_empty() || self.config.allow_empty_blocks) {
            trace!("Successfully extracted block of length {}", block.len());
            Some((block, pos))
        } else {
            warn!("Failed to extract block, depth: {}, empty: {}", depth, block.is_empty());
            None
        }
    }

    pub fn clean_code(&self, code: &str) -> String {
        trace!("Cleaning code of length {}", code.len());
        lazy_static! {
            static ref LINE_COMMENT: Regex = Regex::new(r"//[^\n]*").unwrap();
            static ref BLOCK_COMMENT: Regex = Regex::new(r"/\*.*?\*/").unwrap();
            static ref EXTRA_NEWLINES: Regex = Regex::new(r"\n\s*\n").unwrap();
        }

        let without_line_comments = LINE_COMMENT.replace_all(code, "");
        let without_block_comments = BLOCK_COMMENT.replace_all(&without_line_comments, "");
        let cleaned = EXTRA_NEWLINES.replace_all(&without_block_comments, "\n").into_owned();
        trace!("Cleaned code length: {}", cleaned.len());
        cleaned
    }

    pub fn clean_inner_block(&self, block: &str) -> String {
        trace!("Cleaning inner block of length {}", block.len());
        lazy_static! {
            static ref NESTED_CLASS: Regex = Regex::new(
                r"class\s+\w+[^;]*{[^}]*}"
            ).unwrap();
        }
        let cleaned = NESTED_CLASS.replace_all(block, "").into_owned();
        trace!("Cleaned inner block length: {}", cleaned.len());
        cleaned
    }
}