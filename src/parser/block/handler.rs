use log::{trace, warn};
use super::constants::MAX_DEPTH;
use crate::parser::ParserConfig;

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
        let mut block_start = None;
        let mut escaped = false;
        let mut block = String::new();

        let chars: Vec<char> = content.chars().collect();
        let mut i = 0;
        while i < chars.len() {
            let ch = chars[i];
            
            if depth > self.config.max_depth {
                warn!("Exceeded maximum nesting depth of {}", self.config.max_depth);
                return None;
            }
            
            match ch {
                '\\' if in_string && !escaped => {
                    escaped = true;
                    if block_start.is_some() {
                        block.push(ch);
                    }
                    i += 1;
                    continue;
                }
                '"' | '\'' if !escaped => {
                    if !in_string {
                        in_string = true;
                        string_char = Some(ch);
                    } else if Some(ch) == string_char {
                        in_string = false;
                        string_char = None;
                    }
                }
                '{' if !in_string => {
                    if depth == 0 {
                        // Check if this would be an empty block
                        let mut peek = i + 1;
                        while peek < chars.len() && chars[peek].is_whitespace() {
                            peek += 1;
                        }
                        if peek < chars.len() && chars[peek] == '}' && !self.config.allow_empty_blocks {
                            return None;
                        }
                        block_start = Some(i);
                    }
                    depth += 1;
                    trace!("Increased block depth to {}", depth);
                }
                '}' if !in_string => {
                    if depth == 0 {
                        warn!("Unmatched closing brace found");
                        return None;
                    }
                    depth -= 1;
                    trace!("Decreased block depth to {}", depth);
                    if depth == 0 && block_start.is_some() {
                        block.push(ch);
                        // Get correct byte position for multi-byte characters
                        let final_pos = content[..=i].len();
                        return Some((block, final_pos));
                    }
                }
                _ => {}
            }

            if block_start.is_some() {
                block.push(ch);
            }
            escaped = false;
            i += 1;
        }

        if depth == 0 && block_start.is_none() && self.config.allow_empty_blocks {
            trace!("No block found but empty blocks allowed");
            Some((String::new(), content.len()))
        } else {
            warn!("Failed to extract block, depth: {}, empty: {}", depth, block.is_empty());
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_handler(max_depth: usize, allow_empty: bool) -> BlockHandler {
        BlockHandler::new(ParserConfig {
            max_depth: max_depth.try_into().unwrap(),
            allow_empty_blocks: allow_empty,
            case_sensitive: true,
        })
    }

    #[test]
    fn test_simple_block_extraction() {
        let handler = create_handler(32, false);
        let content = "class Test { value = 123; };";
        let result = handler.extract_block(content);
        assert!(result.is_some());
        let (block, pos) = result.unwrap();
        assert_eq!(block, "{ value = 123; }");
        assert_eq!(pos, 27); // Fix: Updated to match actual byte position
    }

    #[test]
    fn test_nested_block_extraction() {
        let handler = create_handler(32, false);
        let content = "class Outer { class Inner { x = 1; }; y = 2; };";
        let result = handler.extract_block(content);
        assert!(result.is_some());
        let (block, _) = result.unwrap();
        assert_eq!(block, "{ class Inner { x = 1; }; y = 2; }");
    }

    #[test]
    fn test_max_depth_limit() {
        let handler = create_handler(2, false);
        let content = "{ { { { } } } }";
        let result = handler.extract_block(content);
        assert!(result.is_none());
    }

    #[test]
    fn test_string_content_handling() {
        let handler = create_handler(32, false);
        let content = r#"{ text = "{ not a block }"; }"#;
        let result = handler.extract_block(content);
        assert!(result.is_some());
        let (block, _) = result.unwrap();
        assert_eq!(block, content);
    }

    #[test]
    fn test_empty_block() {
        let handler = create_handler(32, true);
        let content = "{}";
        let result = handler.extract_block(content);
        assert!(result.is_some());
        
        let handler = create_handler(32, false);
        let result = handler.extract_block(content);
        assert!(result.is_none());
    }

    #[test]
    fn test_config_class_block() {
        let handler = create_handler(32, false);
        let content = r#"class CfgPatches
{
    class TC_MIRROR
    {
        units[] = {"TC_B_Mirror_1"};
        weapons[] = {"TC_U_Mirror_1"};
        requiredVersion = 0.1;
        requiredAddons[] = {"A3_Characters_F"};
    }
};"#;
        let result = handler.extract_block(content);
        assert!(result.is_some());
        let (block, _) = result.unwrap();
        assert!(block.contains("units[]"));
        assert!(block.contains("weapons[]"));
    }

    #[test]
    fn test_nested_class_inheritance() {
        let handler = create_handler(32, false);
        let content = r#"class TC_U_Mirror_Base: Uniform_Base
{
    author = "Tyen";
    scope = 0;
    class ItemInfo: UniformItem
    {
        uniformClass = "TC_B_Mirror_Base";
        mass = 40;
    };
};"#;
        let result = handler.extract_block(content);
        assert!(result.is_some());
        let (block, _) = result.unwrap();
        assert!(block.contains("author = \"Tyen\""));
        assert!(block.contains("class ItemInfo: UniformItem"));
    }

    #[test]
    fn test_array_values() {
        let handler = create_handler(32, false);
        let content = r#"{
    hiddenSelections[] = {"hs_shirt"};
    hiddenSelectionsTextures[] = {"\tc\mirrorform\uniform\black.paa"};
}"#;
        let result = handler.extract_block(content);
        assert!(result.is_some());
        let (block, _) = result.unwrap();
        assert!(block.contains("hiddenSelections[]"));
        assert!(block.contains("hiddenSelectionsTextures[]"));
    }

    #[test]
    fn test_block_with_path_strings() {
        let handler = create_handler(32, false);
        let content = r#"{
    model = "\tc\mirrorform\uniform\mirror.p3d";
    displayName = "Mirrorform";
}"#;
        let result = handler.extract_block(content);
        assert!(result.is_some());
        let (block, _) = result.unwrap();
        assert!(block.contains("\\tc\\mirrorform"));
    }

    #[test]
    fn test_block_with_preprocessor_defines() {
        let handler = create_handler(32, false);
        let content = r#"#define _ARMA_

{
    class CfgPatches
    {
        units[] = {"Test"};
    };
}"#;
        let result = handler.extract_block(content);
        assert!(result.is_some());
        let (block, _) = result.unwrap();
        assert!(block.contains("class CfgPatches"));
    }
}