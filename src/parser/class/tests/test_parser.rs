#[cfg(test)]
mod tests {
    use crate::parser::{ClassParser, ParserConfig};

    use super::*;
    use std::path::PathBuf;

    fn create_parser() -> ClassParser {
        ClassParser::with_config(ParserConfig {
            max_depth: 32,
            allow_empty_blocks: false,
            case_sensitive: true,
        })
    }

    #[test]
    fn test_parse_simple_class() {
        let mut parser = create_parser();
        let content = r#"
            class SimpleClass {
                someProperty = 1;
            };
        "#;
        let classes = parser.parse_class_definitions(content).unwrap();
        assert_eq!(classes.len(), 1);
        let class = classes.get("SimpleClass").unwrap();
        assert_eq!(class.name, "SimpleClass");
        assert!(!class.properties.is_empty());
    }

    #[test]
    fn test_parse_class_with_inheritance() {
        let mut parser = create_parser();
        let content = r#"
            class BaseClass {};
            class DerivedClass: BaseClass {};
        "#;
        let classes = parser.parse_class_definitions(content).unwrap();
        assert_eq!(classes.len(), 2);
        let derived = classes.get("DerivedClass").unwrap();
        assert_eq!(derived.parent, "BaseClass");
    }

    #[test]
    fn test_parse_multiple_classes() {
        let parser = create_parser();
        let content = r#"
            class First {};
            class Second: Base {};
            class Third {
                class Inner {};
            };
        "#;
        let classes = parser.parse_hierarchical(content).unwrap();
        assert_eq!(classes.len(), 3);
        assert_eq!(classes[1].parent, "Base");
        assert_eq!(classes[2].nested_classes.len(), 1);
    }

    #[test]
    fn test_nested_class_hierarchy() {
        let parser = create_parser();
        let content = r#"
            class Outer {
                class Inner1 {};
                class Inner2: Inner1 {
                    class DeepNested {};
                };
            };
        "#;
        let classes = parser.parse_hierarchical(content).unwrap();
        assert_eq!(classes.len(), 1);
        
        let outer = &classes[0];
        assert_eq!(outer.name, "Outer");
        assert_eq!(outer.nested_classes.len(), 2);
        
        let inner2 = &outer.nested_classes[1];
        assert_eq!(inner2.name, "Inner2");
        assert_eq!(inner2.parent, "Inner1");
        assert_eq!(inner2.nested_classes.len(), 1);
    }

    #[test]
    fn test_forward_declarations() {
        let parser = create_parser();
        let content = r#"
            class Declared;
            class Using: Declared {};
            class ForwardDeclared;
        "#;
        let classes = parser.parse_hierarchical(content).unwrap();
        assert_eq!(classes.len(), 3);
        assert!(classes[0].properties.is_empty());
        assert_eq!(classes[1].parent, "Declared");
    }

    #[test]
    fn test_addon_name_tracking() {
        let mut parser = create_parser();
        parser.set_current_file(PathBuf::from(r"D:\addons\@tc_mirrorform\addons\mirrorform\config.cpp"));
        let content = "class TestClass {};";
        let classes = parser.parse_class_definitions(content).unwrap();
        let test_class = classes.get("TestClass").unwrap();
        assert_eq!(test_class.addon.as_ref().unwrap(), "@tc_mirrorform");
    }

    #[test]
    fn test_case_sensitivity() {
        let mut parser = ClassParser::with_config(ParserConfig {
            max_depth: 32,
            allow_empty_blocks: false,
            case_sensitive: false,
        });
        let content = r#"
            class UPPER_CLASS {};
            class lower_class {};
            class MixedCase {};
        "#;
        let classes = parser.parse_class_definitions(content).unwrap();
        assert!(classes.contains_key("upper_class"));
        assert!(classes.contains_key("lower_class"));
        assert!(classes.contains_key("mixedcase"));
    }
}