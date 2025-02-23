#[cfg(test)]
mod tests {
    use crate::{parser::{block::BlockHandler, class::{class_extractor::ClassExtractor, tests::test_utils::create_test_config, ClassParsing, ClassParsingContext}, ParserConfig, PropertyParser}, ClassData};
    use std::path::PathBuf;

    fn setup_test_components() -> (BlockHandler, PropertyParser, ParserConfig, ClassParsingContext) {
        let config = create_test_config();
        let block_handler = BlockHandler::new(config.clone());
        let property_parser = PropertyParser::new();
        
        let context = ClassParsingContext {
            current_file: Some(PathBuf::from("test/config.cpp")),
            current_addon: Some("@test_addon".to_string()),
            case_sensitive: true,
        };

        (block_handler, property_parser, config, context)
    }

    // Helper to run a test with an extractor
    fn test_with_extractor<F>(content: &str, test_fn: F)
    where
        F: FnOnce(&ClassExtractor, &ClassParsingContext, Option<(ClassData, usize)>),
    {
        let (block_handler, property_parser, config, context) = setup_test_components();
        let extractor = ClassExtractor::new(&block_handler, &property_parser, &config);
        let result = extractor.parse_class(content, &context).unwrap();
        test_fn(&extractor, &context, result);
    }

    #[test]
    fn test_extract_simple_class() {
        let content = r#"class SimpleClass {};
        "#;

        test_with_extractor(content, |_, _, result| {
            let (class_data, _) = result.unwrap();
            assert_eq!(class_data.name, "SimpleClass");
            assert!(class_data.properties.is_empty());
            assert!(class_data.nested_classes.is_empty());
            assert_eq!(class_data.addon.unwrap(), "@test_addon");
        });
    }

    #[test]
    fn test_extract_inherited_class() {
        let content = r#"class DerivedClass: BaseClass {};
        "#;

        test_with_extractor(content, |_, _, result| {
            let (class_data, _) = result.unwrap();
            assert_eq!(class_data.name, "DerivedClass");
            assert_eq!(class_data.parent, "BaseClass");
        });
    }

    #[test]
    fn test_extract_nested_classes() {
        let content = r#"
            class OuterClass {
                class InnerClass {};
                class InnerClass2: BaseInner {};
            };
        "#;

        test_with_extractor(content, |_, _, result| {
            let (class_data, _) = result.unwrap();
            assert_eq!(class_data.name, "OuterClass");
            assert_eq!(class_data.nested_classes.len(), 2);
            
            let inner1 = &class_data.nested_classes[0];
            assert_eq!(inner1.name, "InnerClass");
            assert!(inner1.parent.is_empty());
            
            let inner2 = &class_data.nested_classes[1];
            assert_eq!(inner2.name, "InnerClass2");
            assert_eq!(inner2.parent, "BaseInner");
        });
    }

    #[test]
    fn test_extract_forward_declaration() {
        let content = "class ForwardDeclared;";

        test_with_extractor(content, |_, _, result| {
            let (class_data, _) = result.unwrap();
            assert_eq!(class_data.name, "ForwardDeclared");
            assert!(class_data.properties.is_empty());
            assert!(class_data.nested_classes.is_empty());
        });
    }

    #[test]
    fn test_extract_deep_nesting() {
        let content = r#"
            class Level1 {
                class Level2 {
                    class Level3 {};
                };
            };
        "#;

        test_with_extractor(content, |_, _, result| {
            let (class_data, _) = result.unwrap();
            assert_eq!(class_data.name, "Level1");
            assert_eq!(class_data.nested_classes.len(), 1);
            
            let level2 = &class_data.nested_classes[0];
            assert_eq!(level2.name, "Level2");
            assert_eq!(level2.nested_classes.len(), 1);
            
            let level3 = &level2.nested_classes[0];
            assert_eq!(level3.name, "Level3");
        });
    }

    #[test]
    fn test_extract_with_case_sensitivity() {
        let (block_handler, property_parser, config, mut context) = setup_test_components();
        context.case_sensitive = false;
        let extractor = ClassExtractor::new(&block_handler, &property_parser, &config);
        
        let content = r#"
            class UPPER_CLASS {};
        "#;

        let result = extractor.parse_class(content, &context).unwrap();
        let (class_data, _) = result.unwrap();
        assert_eq!(class_data.name, "upper_class");
    }
}