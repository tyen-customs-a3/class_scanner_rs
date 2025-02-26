use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub mod lexer;
pub mod parser;
pub mod ast;
pub mod types;
pub mod operations;
pub mod error;
pub mod utils;
pub mod models;

pub use error::Error;
pub use parser::Parser;
pub use models::property_value::PropertyValue;
pub use ast::{PropertyType, ClassNode};

#[derive(Debug, Clone)]
pub struct ClassScanner {
    base_path: Option<PathBuf>,
}

impl ClassScanner {
    pub fn new() -> Self {
        Self {
            base_path: None,
        }
    }

    pub fn with_base_path<P: AsRef<Path>>(mut self, path: P) -> Self {
        self.base_path = Some(path.as_ref().to_path_buf());
        self
    }

    pub fn parse_file<P: AsRef<Path>>(&self, path: P) -> Result<Vec<ClassNode>, Error> {
        let mut parser = Parser::new_from_file(path)?;
        let mut classes = Vec::new();
        let class = parser.parse()?;
        classes.push(class);
        Ok(classes)
    }

    pub fn parse_string(&self, content: &str) -> Result<Vec<ClassNode>, Error> {
        let tokens = lexer::Tokenizer::new(content).tokenize()?;
        let mut parser = Parser::new(tokens);
        let mut classes = Vec::new();
        let class = parser.parse()?;
        classes.push(class);
        Ok(classes)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ClassConfig {
    pub name: String,
    pub extends: Option<String>,
    pub properties: HashMap<String, PropertyValue>,
    pub nested_classes: Vec<ClassConfig>,
    pub raw_block: String,
    pub file_path: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use parser::Parser;

    #[test]
    fn test_basic_class_parsing() {
        let input = r#"
            class TestClass {
                stringProp = "value";
                numberProp = 42;
                boolProp = true;
            }
        "#;

        let tokens = lexer::Tokenizer::new(input).tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let result = parser.parse().unwrap();

        // Get the first top-level class (TestClass)
        let test_class = result.nested_classes.first().expect("No classes found");
        assert_eq!(test_class.name, "TestClass");
        assert_eq!(test_class.properties.len(), 3);
        assert_eq!(test_class.properties["stringProp"].raw_value, "value");
        assert_eq!(test_class.properties["numberProp"].raw_value, "42");
        assert_eq!(test_class.properties["boolProp"].raw_value, "true");
    }

    #[test]
    fn test_array_operations() {
        let input = r#"
            class ArrayTest {
                basic[] = {"one", "two"};
                append[] += {"three"};
                remove[] -= {"two"};
            }
        "#;

        let tokens = lexer::Tokenizer::new(input).tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let mut ast = parser.parse().unwrap();

        // Get the first top-level class (ArrayTest)
        let array_test = &ast.nested_classes[0];
        assert_eq!(array_test.name, "ArrayTest");
        assert_eq!(array_test.properties["basic"].raw_value, r#"{one,two}"#);
        assert_eq!(array_test.properties["append"].raw_value, r#"{three}"#);
        assert_eq!(array_test.properties["remove"].raw_value, r#"{two}"#);
    }

    #[test]
    fn test_inheritance() {
        let base_class = r#"
            class BaseClass {
                inherited = "base";
                overridden = "base_value";
            }
        "#;

        let derived_class = r#"
            class DerivedClass : public BaseClass {
                new_prop = "derived";
                overridden = "derived_value";
            }
        "#;

        let tokens = lexer::Tokenizer::new(base_class).tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let base = parser.parse().unwrap();
        let base_class = base.nested_classes[0].clone();

        let tokens = lexer::Tokenizer::new(derived_class).tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let derived = parser.parse().unwrap();
        let derived_class = derived.nested_classes[0].clone();

        let mut inheritance_visitor = ast::InheritanceVisitor::new();
        inheritance_visitor.register_class(base_class);
        inheritance_visitor.register_class(derived_class.clone());
        let processed = inheritance_visitor.process("DerivedClass").unwrap();

        assert_eq!(processed.properties["inherited"].raw_value, "base");
        assert_eq!(processed.properties["overridden"].raw_value, "derived_value");
        assert_eq!(processed.properties["new_prop"].raw_value, "derived");
    }
}