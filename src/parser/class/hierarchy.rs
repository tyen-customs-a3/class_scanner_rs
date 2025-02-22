use log::{debug, trace};
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
    debug!("Parsing hierarchical classes from content of length {}", content.len());
    let mut classes = Vec::new();
    let mut pos = 0;

    let context = ClassParsingContext {
        current_file: current_file.clone(),
        current_addon: current_addon.clone(),
        case_sensitive: config.case_sensitive,
    };

    let extractor = super::class_extractor::ClassExtractor::new(
        block_handler,
        property_parser,
        config
    );

    while pos < content.len() {
        match extractor.parse_class(&content[pos..], &context)? {
            Some((class_data, consumed)) => {
                debug!("Parsed class '{}' at position {}", class_data.name, pos);
                classes.push(class_data);
                pos += consumed;
            }
            None => break,
        }
    }
    
    debug!("Found {} hierarchical classes", classes.len());
    Ok(classes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::class::test_utils::*;

    fn setup_parser() -> (BlockHandler, PropertyParser, ParserConfig) {
        let config = create_test_config();
        let block_handler = BlockHandler::new(config.clone());
        let property_parser = PropertyParser::new();
        (block_handler, property_parser, config)
    }

    #[test]
    fn test_parse_nested_hierarchy() {
        let (block_handler, property_parser, config) = setup_parser();

        let content = r#"
            class Outer {
                value = 1;
                class Inner {
                    value = 2;
                    text = "inner";
                };
            };
        "#;

        let classes = parse_hierarchical_classes(
            content,
            &block_handler,
            &property_parser,
            &Some(PathBuf::from("test/config.cpp")),
            &Some("@test_addon".to_string()),
            &config
        ).unwrap();

        assert_eq!(classes.len(), 1);
        let outer = &classes[0];
        assert_eq!(outer.name, "Outer");
        assert_eq!(outer.nested_classes.len(), 1);
        
        let inner = &outer.nested_classes[0];
        assert_eq!(inner.name, "Inner");
        assert_eq!(inner.properties["value"].as_integer().unwrap(), 2);
    }

    #[test]
    fn test_parse_multiple_top_level_classes() {
        let (block_handler, property_parser, config) = setup_parser();

        let content = r#"
            class First {
                value = 1;
            };
            class Second: Base {
                value = 2;
            };
            class Third {
                class Inner {};
            };
        "#;

        let classes = parse_hierarchical_classes(
            content,
            &block_handler,
            &property_parser,
            &None,
            &None,
            &config
        ).unwrap();

        assert_eq!(classes.len(), 3);
        assert_eq!(classes[0].name, "First");
        assert_eq!(classes[1].name, "Second");
        assert_eq!(classes[1].parent, "Base");
        assert_eq!(classes[2].name, "Third");
        assert_eq!(classes[2].nested_classes.len(), 1);
    }

    #[test]
    fn test_parse_deep_nesting() {
        let (block_handler, property_parser, config) = setup_parser();

        let content = r#"
            class Level1 {
                class Level2 {
                    class Level3 {
                        class Level4 {
                            value = 4;
                        };
                    };
                };
            };
        "#;

        let classes = parse_hierarchical_classes(
            content,
            &block_handler,
            &property_parser,
            &None,
            &None,
            &config
        ).unwrap();

        assert_eq!(classes.len(), 1);
        let level1 = &classes[0];
        let level2 = &level1.nested_classes[0];
        let level3 = &level2.nested_classes[0];
        let level4 = &level3.nested_classes[0];
        
        assert_eq!(level4.name, "Level4");
        assert_eq!(level4.properties["value"].as_integer().unwrap(), 4);
    }

    #[test]
    fn test_parse_complex_hierarchy() {
        let (block_handler, property_parser, config) = setup_parser();

        let content = r#"
            class CfgVehicles {
                class Base;
                class Vehicle: Base {
                    scope = 2;
                    class Turret {
                        weapons[] = {"Gun"};
                    };
                };
                class Tank: Vehicle {
                    armor = 100;
                    class Turret: Turret {
                        weapons[] = {"Cannon", "MG"};
                    };
                };
            };
        "#;

        let classes = parse_hierarchical_classes(
            content,
            &block_handler,
            &property_parser,
            &None,
            &None,
            &config
        ).unwrap();

        assert_eq!(classes.len(), 1);
        let cfg = &classes[0];
        assert_eq!(cfg.name, "CfgVehicles");
        assert_eq!(cfg.nested_classes.len(), 3);

        let tank = cfg.nested_classes.iter().find(|c| c.name == "Tank").unwrap();
        assert_eq!(tank.parent, "Vehicle");
        assert_eq!(tank.properties["armor"].as_integer().unwrap(), 100);

        let turret = &tank.nested_classes[0];
        assert_eq!(turret.name, "Turret");
        let weapons = turret.properties["weapons[]"].array_values().unwrap();
        assert_eq!(weapons.len(), 2);
        assert_eq!(weapons[0], "Cannon");
    }

    #[test]
    fn test_parse_empty_blocks() {
        let (block_handler, property_parser, mut config) = setup_parser();
        config.allow_empty_blocks = true;

        let content = r#"
            class Empty {};
            class Base;
            class Derived: Base {};
        "#;

        let classes = parse_hierarchical_classes(
            content,
            &block_handler,
            &property_parser,
            &None,
            &None,
            &config
        ).unwrap();

        assert_eq!(classes.len(), 3);
        assert!(classes[0].properties.is_empty());
        assert!(classes[0].nested_classes.is_empty());
        assert!(classes[1].properties.is_empty());
        assert_eq!(classes[2].parent, "Base");
    }
}