#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::parser::{block::BlockHandler, class::{hierarchy::parse_hierarchical_classes, tests::test_utils::create_test_config}, ParserConfig, PropertyParser};

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
                class Inner {};
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
        assert_eq!(outer.nested_classes[0].name, "Inner");
    }

    #[test]
    fn test_parse_multiple_top_level_classes() {
        let (block_handler, property_parser, config) = setup_parser();

        let content = r#"
            class First {};
            class Second: Base {};
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
                        class Level4 {};
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
        
        assert_eq!(level1.name, "Level1");
        assert_eq!(level2.name, "Level2");
        assert_eq!(level3.name, "Level3");
        assert_eq!(level4.name, "Level4");
    }

    #[test]
    fn test_parse_complex_inheritance() {
        let (block_handler, property_parser, config) = setup_parser();

        let content = r#"
            class CfgVehicles {
                class Base;
                class Vehicle: Base {};
                class Tank: Vehicle {
                    class Turret {};
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
        assert_eq!(tank.nested_classes.len(), 1);
        assert_eq!(tank.nested_classes[0].name, "Turret");
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
        assert!(classes[0].nested_classes.is_empty());
        assert!(classes[1].properties.is_empty());
        assert_eq!(classes[2].parent, "Base");
    }
}