pub mod array_visitor;
pub mod inheritance_visitor;

pub use array_visitor::ArrayVisitor;
pub use inheritance_visitor::InheritanceVisitor;

use std::collections::HashMap;
use crate::models::property_value::PropertyValue;
use crate::operations::arrays::ArrayOperation;
use crate::error::Error;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct ClassNode {
    pub name: String,
    pub parent: Option<String>,
    pub properties: HashMap<String, PropertyNode>,
    pub nested_classes: Vec<ClassNode>,
    pub access: AccessModifier,
    pub raw_block: String,
    pub file_path: Option<PathBuf>,
}

#[derive(Debug, Clone)]
pub struct PropertyNode {
    pub name: String,
    pub value_type: PropertyType,
    pub raw_value: String,
    pub operation: Option<ArrayOperation>,
    pub array_values: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PropertyType {
    String,
    Number,
    Boolean,
    Array,
    Object,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AccessModifier {
    Public,
    Private,
}

pub trait AstVisitor {
    fn visit_class(&mut self, class: &mut ClassNode) -> Result<(), Error>;
    fn visit_property(&mut self, property: &mut PropertyNode) -> Result<(), Error>;
    fn visit_array(&mut self, array: &mut Vec<String>, operation: Option<ArrayOperation>) -> Result<(), Error>;
}

impl ClassNode {
    pub fn new(name: String) -> Self {
        Self {
            name,
            parent: None,
            properties: HashMap::new(),
            nested_classes: Vec::new(),
            access: AccessModifier::Public,
            raw_block: String::new(),
            file_path: None,
        }
    }

    pub fn with_parent(mut self, parent: impl Into<String>) -> Self {
        self.parent = Some(parent.into());
        self
    }

    pub fn with_access(mut self, access: AccessModifier) -> Self {
        self.access = access;
        self
    }

    pub fn accept<V: AstVisitor>(&mut self, visitor: &mut V) -> Result<(), Error> {
        visitor.visit_class(self)?;
        
        for property in self.properties.values_mut() {
            visitor.visit_property(property)?;
            if property.value_type == PropertyType::Array {
                visitor.visit_array(&mut property.array_values, property.operation)?;
            }
        }

        for nested in &mut self.nested_classes {
            nested.accept(visitor)?;
        }

        Ok(())
    }

    pub fn get_array(&self, name: &str) -> Option<&Vec<String>> {
        self.properties.get(name).and_then(|prop| {
            if prop.value_type == PropertyType::Array {
                Some(&prop.array_values)
            } else {
                None
            }
        })
    }
}

impl PropertyNode {
    pub fn new(name: impl Into<String>, value_type: PropertyType, raw_value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value_type,
            raw_value: raw_value.into(),
            operation: None,
            array_values: Vec::new(),
        }
    }

    pub fn with_array_op(mut self, operation: ArrayOperation) -> Self {
        self.operation = Some(operation);
        self
    }

    pub fn with_array_values(mut self, values: Vec<String>) -> Self {
        self.array_values = values;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Mock visitor for testing visitor pattern
    struct TestVisitor {
        class_visit_count: usize,
        property_visit_count: usize,
        array_visit_count: usize,
    }

    impl TestVisitor {
        fn new() -> Self {
            Self {
                class_visit_count: 0,
                property_visit_count: 0,
                array_visit_count: 0,
            }
        }
    }

    impl AstVisitor for TestVisitor {
        fn visit_class(&mut self, _class: &mut ClassNode) -> Result<(), Error> {
            self.class_visit_count += 1;
            Ok(())
        }

        fn visit_property(&mut self, _property: &mut PropertyNode) -> Result<(), Error> {
            self.property_visit_count += 1;
            Ok(())
        }

        fn visit_array(&mut self, _array: &mut Vec<String>, _operation: Option<ArrayOperation>) -> Result<(), Error> {
            self.array_visit_count += 1;
            Ok(())
        }
    }

    #[test]
    fn test_visitor_pattern() {
        // Create a complex class hierarchy
        let mut root = ClassNode::new("Root".to_string());
        
        // Add properties to root
        root.properties.insert("str_prop".to_string(), 
            PropertyNode::new("str_prop", PropertyType::String, "value"));
        root.properties.insert("arr_prop".to_string(), 
            PropertyNode::new("arr_prop", PropertyType::Array, "{a,b,c}")
                .with_array_values(vec!["a".to_string(), "b".to_string(), "c".to_string()]));

        // Add nested class with properties
        let mut nested = ClassNode::new("Nested".to_string());
        nested.properties.insert("nested_arr".to_string(),
            PropertyNode::new("nested_arr", PropertyType::Array, "{1,2}")
                .with_array_values(vec!["1".to_string(), "2".to_string()]));
        root.nested_classes.push(nested);

        // Create and apply visitor
        let mut visitor = TestVisitor::new();
        root.accept(&mut visitor).unwrap();

        // Verify visit counts
        assert_eq!(visitor.class_visit_count, 2); // Root + 1 nested class
        assert_eq!(visitor.property_visit_count, 3); // 2 root props + 1 nested prop
        assert_eq!(visitor.array_visit_count, 2); // 2 array properties
    }

    #[test]
    fn test_visitor_error_propagation() {
        // Error-generating visitor
        struct ErrorVisitor;
        impl AstVisitor for ErrorVisitor {
            fn visit_class(&mut self, _: &mut ClassNode) -> Result<(), Error> {
                Err(Error::InheritanceError("Test error".to_string()))
            }
            fn visit_property(&mut self, _: &mut PropertyNode) -> Result<(), Error> {
                Ok(())
            }
            fn visit_array(&mut self, _: &mut Vec<String>, _: Option<ArrayOperation>) -> Result<(), Error> {
                Ok(())
            }
        }

        let mut class = ClassNode::new("Test".to_string());
        let result = class.accept(&mut ErrorVisitor);
        assert!(result.is_err());
        if let Err(Error::InheritanceError(msg)) = result {
            assert_eq!(msg, "Test error");
        } else {
            panic!("Expected InheritanceError");
        }
    }

    #[test]
    fn test_property_type_operations() {
        // Test property type equality
        assert_eq!(PropertyType::String, PropertyType::String);
        assert_ne!(PropertyType::String, PropertyType::Number);
        assert_ne!(PropertyType::Array, PropertyType::Object);

        // Test property creation with different types
        let string_prop = PropertyNode::new("str", PropertyType::String, "value");
        assert_eq!(string_prop.value_type, PropertyType::String);

        let num_prop = PropertyNode::new("num", PropertyType::Number, "123");
        assert_eq!(num_prop.value_type, PropertyType::Number);

        let bool_prop = PropertyNode::new("bool", PropertyType::Boolean, "true");
        assert_eq!(bool_prop.value_type, PropertyType::Boolean);

        let array_prop = PropertyNode::new("arr", PropertyType::Array, "{1,2,3}");
        assert_eq!(array_prop.value_type, PropertyType::Array);

        let obj_prop = PropertyNode::new("obj", PropertyType::Object, "{}");
        assert_eq!(obj_prop.value_type, PropertyType::Object);
    }

    #[test]
    fn test_class_array_operations() {
        let mut class = ClassNode::new("Test".to_string());
        
        // Test get_array with non-existent property
        assert!(class.get_array("nonexistent").is_none());

        // Test get_array with non-array property
        class.properties.insert("string_prop".to_string(),
            PropertyNode::new("string_prop", PropertyType::String, "value"));
        assert!(class.get_array("string_prop").is_none());

        // Test get_array with array property
        let values = vec!["a".to_string(), "b".to_string()];
        class.properties.insert("array_prop".to_string(),
            PropertyNode::new("array_prop", PropertyType::Array, "{a,b}")
                .with_array_values(values.clone()));
        
        let array = class.get_array("array_prop");
        assert!(array.is_some());
        assert_eq!(array.unwrap(), &values);
    }

    #[test]
    fn test_nested_class_operations() {
        let mut parent = ClassNode::new("Parent".to_string());
        let child1 = ClassNode::new("Child1".to_string());
        let child2 = ClassNode::new("Child2".to_string());

        parent.nested_classes.push(child1);
        parent.nested_classes.push(child2);

        assert_eq!(parent.nested_classes.len(), 2);
        assert_eq!(parent.nested_classes[0].name, "Child1");
        assert_eq!(parent.nested_classes[1].name, "Child2");

        // Test nested class modifications
        let mut grandchild = ClassNode::new("GrandChild".to_string());
        grandchild.properties.insert("prop".to_string(),
            PropertyNode::new("prop", PropertyType::String, "value"));

        parent.nested_classes[0].nested_classes.push(grandchild);

        assert_eq!(parent.nested_classes[0].nested_classes.len(), 1);
        assert_eq!(parent.nested_classes[0].nested_classes[0].name, "GrandChild");
    }

    #[test]
    fn test_multiple_visitor_interaction() {
        // Create mock visitors that maintain state
        struct CountingVisitor {
            name: String,
            visit_order: Vec<String>,
        }

        impl CountingVisitor {
            fn new(name: &str) -> Self {
                Self {
                    name: name.to_string(),
                    visit_order: Vec::new(),
                }
            }
        }

        impl AstVisitor for CountingVisitor {
            fn visit_class(&mut self, class: &mut ClassNode) -> Result<(), Error> {
                self.visit_order.push(format!("{}-{}", self.name, class.name));
                Ok(())
            }
            fn visit_property(&mut self, _: &mut PropertyNode) -> Result<(), Error> {
                Ok(())
            }
            fn visit_array(&mut self, _: &mut Vec<String>, _: Option<ArrayOperation>) -> Result<(), Error> {
                Ok(())
            }
        }

        // Create test structure
        let mut root = ClassNode::new("Root".to_string());
        let child = ClassNode::new("Child".to_string());
        root.nested_classes.push(child);

        // Apply multiple visitors
        let mut visitor1 = CountingVisitor::new("V1");
        let mut visitor2 = CountingVisitor::new("V2");

        root.accept(&mut visitor1).unwrap();
        root.accept(&mut visitor2).unwrap();

        // Verify visit order
        assert_eq!(visitor1.visit_order, vec!["V1-Root", "V1-Child"]);
        assert_eq!(visitor2.visit_order, vec!["V2-Root", "V2-Child"]);
    }
}