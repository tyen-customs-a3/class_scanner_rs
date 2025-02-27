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
pub use ast::{PropertyType, ClassNode, AstVisitor};

/// A high-level interface for parsing and processing class configuration files.
///
/// The `ClassScanner` provides a convenient API for working with class configuration files.
/// It encapsulates the complexity of lexing, parsing, and processing class definitions,
/// including inheritance relationships and array operations.
///
/// # Examples
///
/// ```
/// use class_scanner::{ClassScanner, Error};
///
/// fn main() -> Result<(), Error> {
///     let scanner = ClassScanner::new();
///     
///     let input = r#"
///         class Vehicle {
///             crew = 1;
///             maxSpeed = 120;
///         };
///         
///         class Car: Vehicle {
///             crew = 2;
///             wheels = 4;
///         };
///     "#;
///     
///     let classes = scanner.parse_string(input)?;
///     println!("Found {} classes", classes.len());
///     
///     // Process inheritance
///     let processed = scanner.process_inheritance(classes, "Car")?;
///     println!("Car has {} properties after inheritance", processed.properties.len());
///     
///     Ok(())
/// }
/// ```
#[derive(Debug, Clone)]
pub struct ClassScanner {
    base_path: Option<PathBuf>,
}

impl ClassScanner {
    /// Create a new ClassScanner instance.
    pub fn new() -> Self {
        Self {
            base_path: None,
        }
    }

    /// Set the base path for resolving file includes.
    ///
    /// When parsing files with `#include` directives, the preprocessor uses this
    /// path as a base directory for resolving relative paths.
    pub fn with_base_path<P: AsRef<Path>>(mut self, path: P) -> Self {
        self.base_path = Some(path.as_ref().to_path_buf());
        self
    }

    /// Parse a class configuration file.
    ///
    /// This method reads the file, preprocesses it to handle includes,
    /// tokenizes the content, and then parses it into a class hierarchy.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the configuration file to parse.
    ///
    /// # Returns
    ///
    /// A `Result` containing a `Vec<ClassNode>` if parsing succeeds, or an `Error` otherwise.
    pub fn parse_file<P: AsRef<Path>>(&self, path: P) -> Result<Vec<ClassNode>, Error> {
        let path_ref = path.as_ref();
        
        // Use the preprocessor to handle includes
        let base_dir = if let Some(ref base_path) = self.base_path {
            base_path.clone()
        } else {
            path_ref.parent()
                .unwrap_or(&PathBuf::from("."))
                .to_path_buf()
        };
        
        let mut preprocessor = lexer::Preprocessor::new(&base_dir);
        let content = preprocessor.process_file(path_ref)?;
        
        // Tokenize and parse the preprocessed content
        let mut tokenizer = lexer::Tokenizer::with_file_path(&content, path_ref);
        let tokens = tokenizer.tokenize()?;
        
        let mut parser = Parser::new(tokens);
        let class = parser.parse()?;
        
        Ok(vec![class])
    }

    /// Parse a string containing class definitions.
    ///
    /// # Arguments
    ///
    /// * `content` - A string containing class definitions to parse.
    ///
    /// # Returns
    ///
    /// A `Result` containing a `Vec<ClassNode>` if parsing succeeds, or an `Error` otherwise.
    pub fn parse_string(&self, content: &str) -> Result<Vec<ClassNode>, Error> {
        let tokens = lexer::Tokenizer::new(content).tokenize()?;
        let mut parser = Parser::new(tokens);
        let root = parser.parse()?;
        
        // Extract individual classes from the root node
        let mut classes = vec![];
        classes.push(root.clone()); // Include the root node
        classes.extend(root.nested_classes.clone()); // Add top-level classes
        
        Ok(classes)
    }
    
    /// Process inheritance relationships between classes.
    ///
    /// This method creates an `InheritanceVisitor`, registers all provided classes,
    /// and then processes inheritance for the specified target class.
    ///
    /// # Arguments
    ///
    /// * `classes` - A collection of `ClassNode` objects to register for inheritance processing.
    /// * `target_class_name` - The name of the class for which to process inheritance.
    ///
    /// # Returns
    ///
    /// A `Result` containing the processed `ClassNode` with inherited properties, or an `Error` otherwise.
    pub fn process_inheritance<T>(&self, classes: T, target_class_name: &str) -> Result<ClassNode, Error> 
    where 
        T: IntoIterator<Item = ClassNode>,
    {
        let mut inheritance_visitor = ast::inheritance_visitor::InheritanceVisitor::new();
        
        // Register all classes
        for class in classes {
            inheritance_visitor.register_class(class);
        }
        
        // Process inheritance for the target class
        inheritance_visitor.process(target_class_name)
    }
    
    /// Process array operations in a class hierarchy.
    ///
    /// This method creates an `ArrayVisitor` and applies it to the provided class node,
    /// processing array operations such as append (`+=`) and remove (`-=`).
    ///
    /// # Arguments
    ///
    /// * `class` - A mutable reference to a `ClassNode` to process.
    ///
    /// # Returns
    ///
    /// A `Result` containing `()` if processing succeeds, or an `Error` otherwise.
    pub fn process_arrays(&self, class: &mut ClassNode) -> Result<(), Error> {
        let mut array_visitor = ast::array_visitor::ArrayVisitor::new();
        array_visitor.visit_class(class)
    }

    /// Complete preprocessing, parsing, and processing of a class configuration file.
    ///
    /// This is a convenience method that combines file parsing, inheritance processing,
    /// and array processing into a single operation.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the configuration file to process.
    /// * `target_class_name` - The name of the class to process.
    ///
    /// # Returns
    ///
    /// A `Result` containing the fully processed `ClassNode`, or an `Error` otherwise.
    pub fn process_file<P: AsRef<Path>>(&self, path: P, target_class_name: &str) -> Result<ClassNode, Error> {
        // Parse the file
        let classes = self.parse_file(path)?;
        
        // Process inheritance
        let mut processed_class = self.process_inheritance(classes, target_class_name)?;
        
        // Process arrays
        self.process_arrays(&mut processed_class)?;
        
        Ok(processed_class)
    }
}

/// Configuration class representation used for serialization/deserialization.
///
/// This struct is a more user-friendly representation of a `ClassNode` that
/// can be easily serialized to or deserialized from formats like JSON.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ClassConfig {
    pub name: String,
    pub extends: Option<String>,
    pub properties: HashMap<String, PropertyValue>,
    pub nested_classes: Vec<ClassConfig>,
    pub raw_block: String,
    pub file_path: Option<String>,
}

/// Conversion from ClassNode to ClassConfig for serialization
impl From<ClassNode> for ClassConfig {
    fn from(node: ClassNode) -> Self {
        ClassConfig {
            name: node.name,
            extends: node.parent,
            properties: node.properties.into_iter()
                .map(|(k, v)| (k, PropertyValue::from(v)))
                .collect(),
            nested_classes: node.nested_classes.into_iter().map(ClassConfig::from).collect(),
            raw_block: node.raw_block,
            file_path: node.file_path.map(|p| p.to_string_lossy().to_string()),
        }
    }
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

        let mut inheritance_visitor = ast::inheritance_visitor::InheritanceVisitor::new();
        inheritance_visitor.register_class(base_class);
        inheritance_visitor.register_class(derived_class.clone());
        let processed = inheritance_visitor.process("DerivedClass").unwrap();

        assert_eq!(processed.properties["inherited"].raw_value, "base");
        assert_eq!(processed.properties["overridden"].raw_value, "derived_value");
        assert_eq!(processed.properties["new_prop"].raw_value, "derived");
    }
}