use log::trace;
use crate::parser::patterns::{LINE_COMMENT, BLOCK_COMMENT, EXTRA_NEWLINES};

pub trait BlockCleaner {
    fn clean_code(&self, code: &str) -> String;
    fn clean_inner_block(&self, block: &str) -> String;
}

impl BlockCleaner for super::handler::BlockHandler {
    fn clean_code(&self, code: &str) -> String {
        trace!("Cleaning code of length {}", code.len());
        let without_line_comments = LINE_COMMENT.replace_all(code, "");
        let without_block_comments = BLOCK_COMMENT.replace_all(&without_line_comments, "");
        let cleaned = EXTRA_NEWLINES.replace_all(&without_block_comments, "\n").into_owned();
        trace!("Cleaned code length: {}", cleaned.len());
        cleaned
    }

    fn clean_inner_block(&self, block: &str) -> String {
        trace!("Cleaning inner block of length {}", block.len());
        // Only clean comments and normalize whitespace, preserve nested classes
        self.clean_code(block)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{ParserConfig, block::handler::BlockHandler};

    fn create_cleaner() -> impl BlockCleaner {
        BlockHandler::new(ParserConfig {
            max_depth: 32,
            allow_empty_blocks: false,
            case_sensitive: true,
        })
    }

    #[test]
    fn test_clean_mikero_header_comments() {
        let cleaner = create_cleaner();
        let code = r#"////////////////////////////////////////////////////////////////////
        //DeRap: config.bin
        //Produced from mikero's Dos Tools Dll version 9.93
        //https://mikero.bytex.digital/Downloads
        //'now' is Sat Feb 01 13:12:55 2025
        class CfgPatches {
            // Content
        };"#;
        let cleaned = cleaner.clean_code(code);
        assert!(!cleaned.contains("DeRap"));
        assert!(cleaned.contains("class CfgPatches"));
    }

    #[test]
    fn test_preserve_nested_class_definitions() {
        let cleaner = create_cleaner();
        let block = r#"{
            class ItemInfo: UniformItem {
                uniformClass = "TC_B_Mirror_Base";
                mass = 40;
            };
            displayName = "Mirrorform";
        }"#;
        let cleaned = cleaner.clean_inner_block(block);
        assert!(cleaned.contains("class ItemInfo"));
        assert!(cleaned.contains("displayName"));
    }

    #[test]
    fn test_clean_block_comments() {
        let cleaner = create_cleaner();
        let code = r#"/* Block comment before */
        class Test {
            value = 1; /* inline comment */
            /* Multi-line
               comment */
            name = "test";
        };"#;
        let cleaned = cleaner.clean_code(code);
        assert!(!cleaned.contains("Block comment"));
        assert!(!cleaned.contains("inline comment"));
        assert!(cleaned.contains("value = 1;"));
    }

    #[test]
    fn test_clean_preprocessor_defines() {
        let cleaner = create_cleaner();
        let code = r#"#define _ARMA_  // Define comment
        // Another comment
        class Test {};"#;
        let cleaned = cleaner.clean_code(code);
        assert!(!cleaned.contains("Define comment"));
        assert!(cleaned.contains("#define _ARMA_"));
    }
}