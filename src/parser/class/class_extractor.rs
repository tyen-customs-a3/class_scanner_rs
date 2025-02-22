use lazy_static::lazy_static;
use log::{debug, trace};
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
            None => return Ok(None),
        };

        let class_name = captures.get(1)
            .ok_or_else(|| Error::Validation("Missing class name".to_string()))?
            .as_str();

        let parent_name = captures.get(2).map_or("", |m| m.as_str());
        let has_block = captures.get(0).map_or(false, |m| m.as_str().ends_with("{"));
        let class_start = captures.get(0).unwrap().end();
        
        let class_name = if !context.case_sensitive {
            class_name.to_lowercase()
        } else {
            class_name.to_string()
        };

        let mut class_data = ClassData::new(class_name)
            .with_parent(parent_name);
            
        if let Some(file) = &context.current_file {
            class_data = class_data.with_source(file);
        }
        if let Some(addon) = &context.current_addon {
            class_data = class_data.with_addon(addon);
        }

        if has_block {
            if let Some((block_content, block_end)) = self.block_handler.extract_block(&content[class_start..]) {
                let cleaned_block = self.block_handler.clean_inner_block(&block_content);
                class_data.properties = self.property_parser.parse_block_properties(&cleaned_block)?;

                class_data.nested_classes = super::hierarchy::parse_hierarchical_classes(
                    &block_content,
                    self.block_handler,
                    self.property_parser,
                    &context.current_file,
                    &context.current_addon,
                    self.config
                )?;

                return Ok(Some((class_data, class_start + block_end)));
            }
        }

        Ok(Some((class_data, class_start)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::class::test_utils::*;
    use std::rc::Rc;

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
    fn test_parse_simple_class() {
        let content = r#"
            class SimpleClass {
                value = 123;
                text = "test";
            };
        "#;

        test_with_extractor(content, |_, _, result| {
            let (class_data, _) = result.unwrap();
            assert_eq!(class_data.name, "SimpleClass");
            assert!(class_data.properties.contains_key("value"));
            assert!(class_data.properties.contains_key("text"));
            assert_eq!(class_data.addon.unwrap(), "@test_addon");
        });
    }

    #[test]
    fn test_parse_inherited_class() {
        let content = r#"
            class DerivedClass: BaseClass {
                scope = 2;
                model = "\test\model.p3d";
            };
        "#;

        test_with_extractor(content, |_, _, result| {
            let (class_data, _) = result.unwrap();
            assert_eq!(class_data.name, "DerivedClass");
            assert_eq!(class_data.parent, "BaseClass");
            assert_eq!(class_data.properties["scope"].as_integer().unwrap(), 2);
            assert_eq!(class_data.properties["model"].as_string().unwrap(), r"\test\model.p3d");
        });
    }

    #[test]
    fn test_parse_nested_classes() {
        let content = r#"
            class OuterClass {
                value = 1;
                class InnerClass {
                    value = 2;
                };
                class InnerClass2: BaseInner {
                    value = 3;
                };
            };
        "#;

        test_with_extractor(content, |_, _, result| {
            let (class_data, _) = result.unwrap();
            assert_eq!(class_data.name, "OuterClass");
            assert_eq!(class_data.nested_classes.len(), 2);
            
            let inner1 = &class_data.nested_classes[0];
            assert_eq!(inner1.name, "InnerClass");
            assert_eq!(inner1.properties["value"].as_integer().unwrap(), 2);
            
            let inner2 = &class_data.nested_classes[1];
            assert_eq!(inner2.name, "InnerClass2");
            assert_eq!(inner2.parent, "BaseInner");
        });
    }

    #[test]
    fn test_parse_empty_class() {
        let content = "class EmptyClass {};";

        test_with_extractor(content, |_, _, result| {
            let (class_data, _) = result.unwrap();
            assert_eq!(class_data.name, "EmptyClass");
            assert!(class_data.properties.is_empty());
            assert!(class_data.nested_classes.is_empty());
        });
    }

    #[test]
    fn test_parse_forward_declaration() {
        let content = "class ForwardDeclared;";

        test_with_extractor(content, |_, _, result| {
            let (class_data, _) = result.unwrap();
            assert_eq!(class_data.name, "ForwardDeclared");
            assert!(class_data.properties.is_empty());
            assert!(class_data.nested_classes.is_empty());
        });
    }

    #[test]
    fn test_parse_case_sensitivity() {
        let (block_handler, property_parser, config, mut context) = setup_test_components();
        context.case_sensitive = false;
        let extractor = ClassExtractor::new(&block_handler, &property_parser, &config);
        
        let content = r#"
            class UPPER_CLASS {
                VALUE = 1;
            };
        "#;

        let result = extractor.parse_class(content, &context).unwrap();
        let (class_data, _) = result.unwrap();
        assert_eq!(class_data.name, "upper_class");
    }

    #[test]
    fn test_parse_complex_arrays() {
        let content = r#"
            class ArrayClass {
                empty[] = {};
                simple[] = {"one", "two"};
                nested[] = {{"a", "1"}, {"b", "2"}};
                mixed[] = {1, "two", true};
            };
        "#;

        test_with_extractor(content, |_, _, result| {
            let (class_data, _) = result.unwrap();
            assert!(class_data.properties["empty"].is_array());
            assert_eq!(class_data.properties["simple"].array_values().unwrap().len(), 2);
            assert_eq!(class_data.properties["nested"].array_values().unwrap().len(), 2);
            assert_eq!(class_data.properties["mixed"].array_values().unwrap().len(), 3);
        });
    }
}