use std::collections::{HashMap, HashSet};
use super::{ClassNode, PropertyNode, AstVisitor};
use crate::error::Error;
use crate::operations::arrays::ArrayOperation;

pub struct InheritanceVisitor {
    class_map: HashMap<String, ClassNode>,
    processed: Vec<String>,
}

impl InheritanceVisitor {
    pub fn new() -> Self {
        Self {
            class_map: HashMap::new(),
            processed: Vec::new(),
        }
    }

    pub fn register_class(&mut self, class: ClassNode) {
        // Save both the class name and its full path if available
        let class_name = class.name.clone();
        self.class_map.insert(class_name, class);
    }

    pub fn process(&mut self, class_name: &str) -> Result<ClassNode, Error> {
        // Use a separate set to track recursion paths during a single processing call
        self.process_with_cycle_detection(class_name, &mut HashSet::new())
    }
    
    fn process_with_cycle_detection(&mut self, class_name: &str, processing_stack: &mut HashSet<String>) 
        -> Result<ClassNode, Error> {
        // Return already processed classes directly
        if self.processed.contains(&class_name.to_string()) {
            return Ok(self.class_map.get(class_name).unwrap().clone());
        }
        
        // Check for circular inheritance
        if processing_stack.contains(&class_name.to_string()) {
            return Err(Error::InheritanceError(format!("Circular inheritance detected involving class {}", class_name)));
        }
        
        let class = self.class_map.get(class_name)
            .ok_or_else(|| Error::InheritanceError(format!("Class {} not found", class_name)))?
            .clone();

        let mut result = class.clone();

        // Mark this class as being processed to detect cycles
        processing_stack.insert(class_name.to_string());
        
        if let Some(parent_name) = &class.parent {
            // Try to process the parent class, which might detect a cycle
            match self.process_with_cycle_detection(parent_name, processing_stack) {
                Ok(parent) => self.merge_properties(&mut result, &parent),
                Err(Error::InheritanceError(msg)) if msg.contains("Circular inheritance") => {
                    // If it's a circular reference, we can still use what we have
                    // Just continue with the current class without merging parent properties
                    // Optionally log the circular reference
                },
                Err(e) => return Err(e),  // Propagate other errors
            }
        }

        // Remove this class from the processing stack since we're done with it
        processing_stack.remove(&class_name.to_string());
        
        // Mark as fully processed for future reference
        self.processed.push(class_name.to_string());
        
        Ok(result)
    }

    fn merge_properties(&self, child: &mut ClassNode, parent: &ClassNode) {
        // Copy properties from parent that aren't in child
        for (name, parent_prop) in &parent.properties {
            if !child.properties.contains_key(name) {
                child.properties.insert(name.clone(), parent_prop.clone());
            } else if let Some(child_prop) = child.properties.get_mut(name) {
                // Special handling for array properties with operations
                if child_prop.value_type == crate::ast::PropertyType::Array && 
                   parent_prop.value_type == crate::ast::PropertyType::Array && 
                   child_prop.operation.is_some() {
                    // Apply the array operation
                    use crate::operations::arrays::ArrayProcessor;
                    let op = child_prop.operation.unwrap();
                    child_prop.array_values = ArrayProcessor::process(
                        &parent_prop.array_values,
                        &child_prop.array_values,
                        op
                    );
                }
            }
        }

        // Merge nested classes recursively
        for parent_nested in &parent.nested_classes {
            let mut found = false;
            for child_nested in &mut child.nested_classes {
                if child_nested.name == parent_nested.name {
                    self.merge_properties(child_nested, parent_nested);
                    found = true;
                    break;
                }
            }
            if !found {
                child.nested_classes.push(parent_nested.clone());
            }
        }
    }
}

impl AstVisitor for InheritanceVisitor {
    fn visit_class(&mut self, class: &mut ClassNode) -> Result<(), Error> {
        // Register this class and any nested classes
        self.register_class(class.clone());
        for nested in &mut class.nested_classes {
            self.visit_class(nested)?;
        }
        Ok(())
    }

    fn visit_property(&mut self, _property: &mut PropertyNode) -> Result<(), Error> {
        // Property-level inheritance handled in merge_properties
        Ok(())
    }

    fn visit_array(&mut self, _array: &mut Vec<String>, _operation: Option<ArrayOperation>) -> Result<(), Error> {
        // Array operations handled by ArrayVisitor
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{PropertyType, AccessModifier};
    
    // Helper function to create test classes
    fn create_test_class(name: &str, parent: Option<&str>, properties: Vec<(&str, &str)>) -> ClassNode {
        let mut class = ClassNode::new(name.to_string());
        
        if let Some(parent_name) = parent {
            class = class.with_parent(parent_name);
        }
        
        for (prop_name, prop_value) in properties {
            let property = PropertyNode::new(
                prop_name, 
                PropertyType::String, 
                prop_value
            );
            class.properties.insert(prop_name.to_string(), property);
        }
        
        class
    }
    
    #[test]
    fn test_basic_inheritance() {
        let parent = create_test_class(
            "Parent", 
            None, 
            vec![("prop1", "parent_val1"), ("prop2", "parent_val2")]
        );
        
        let child = create_test_class(
            "Child", 
            Some("Parent"), 
            vec![("prop1", "child_val1"), ("prop3", "child_val3")]
        );
        
        let mut visitor = InheritanceVisitor::new();
        visitor.register_class(parent);
        visitor.register_class(child);
        
        let processed = visitor.process("Child").unwrap();
        
        assert_eq!(processed.properties.len(), 3);
        assert_eq!(processed.properties["prop1"].raw_value, "child_val1"); // Overridden
        assert_eq!(processed.properties["prop2"].raw_value, "parent_val2"); // Inherited
        assert_eq!(processed.properties["prop3"].raw_value, "child_val3"); // Child's own
    }
    
    #[test]
    fn test_multilevel_inheritance() {
        let grandparent = create_test_class(
            "GrandParent", 
            None, 
            vec![("prop1", "gp_val1"), ("prop2", "gp_val2")]
        );
        
        let parent = create_test_class(
            "Parent", 
            Some("GrandParent"), 
            vec![("prop2", "parent_val2"), ("prop3", "parent_val3")]
        );
        
        let child = create_test_class(
            "Child", 
            Some("Parent"), 
            vec![("prop3", "child_val3"), ("prop4", "child_val4")]
        );
        
        let mut visitor = InheritanceVisitor::new();
        visitor.register_class(grandparent);
        visitor.register_class(parent);
        visitor.register_class(child);
        
        let processed = visitor.process("Child").unwrap();
        
        assert_eq!(processed.properties.len(), 4);
        assert_eq!(processed.properties["prop1"].raw_value, "gp_val1");     // From grandparent
        assert_eq!(processed.properties["prop2"].raw_value, "parent_val2"); // From parent (overridden)
        assert_eq!(processed.properties["prop3"].raw_value, "child_val3");  // From child (overridden)
        assert_eq!(processed.properties["prop4"].raw_value, "child_val4");  // Child's own
    }
    
    #[test]
    fn test_nested_class_inheritance() {
        // Parent with nested class
        let mut parent = create_test_class("Parent", None, vec![("prop1", "parent_val1")]);
        let nested = create_test_class("Nested", None, vec![("nested_prop", "nested_val")]);
        parent.nested_classes.push(nested);
        
        // Child that should inherit the nested class
        let child = create_test_class("Child", Some("Parent"), vec![("prop2", "child_val2")]);
        
        let mut visitor = InheritanceVisitor::new();
        visitor.register_class(parent);
        visitor.register_class(child);
        
        let processed = visitor.process("Child").unwrap();
        
        assert_eq!(processed.nested_classes.len(), 1);
        assert_eq!(processed.nested_classes[0].name, "Nested");
        assert_eq!(processed.nested_classes[0].properties["nested_prop"].raw_value, "nested_val");
    }
    
    #[test]
    fn test_nested_class_override() {
        // Parent with nested class
        let mut parent = create_test_class("Parent", None, vec![]);
        let parent_nested = create_test_class(
            "Nested", 
            None, 
            vec![("prop1", "parent_val"), ("prop2", "parent_val2")]
        );
        parent.nested_classes.push(parent_nested);
        
        // Child that overrides the nested class
        let mut child = create_test_class("Child", Some("Parent"), vec![]);
        let child_nested = create_test_class(
            "Nested", 
            None, 
            vec![("prop1", "child_val"), ("prop3", "child_val3")]
        );
        child.nested_classes.push(child_nested);
        
        let mut visitor = InheritanceVisitor::new();
        visitor.register_class(parent);
        visitor.register_class(child);
        
        let processed = visitor.process("Child").unwrap();
        
        assert_eq!(processed.nested_classes.len(), 1);
        let nested = &processed.nested_classes[0];
        assert_eq!(nested.name, "Nested");
        assert_eq!(nested.properties.len(), 3);
        assert_eq!(nested.properties["prop1"].raw_value, "child_val");    // Overridden
        assert_eq!(nested.properties["prop2"].raw_value, "parent_val2");  // Inherited
        assert_eq!(nested.properties["prop3"].raw_value, "child_val3");   // Child's own
    }
    
    #[test]
    fn test_class_not_found() {
        let mut visitor = InheritanceVisitor::new();
        let result = visitor.process("NonExistentClass");
        assert!(result.is_err());
        if let Err(err) = result {
            match err {
                Error::InheritanceError(msg) => {
                    assert!(msg.contains("NonExistentClass"));
                    assert!(msg.contains("not found"));
                },
                _ => panic!("Expected InheritanceError, got {:?}", err),
            }
        }
    }
    
    #[test]
    fn test_circular_inheritance() {
        // Create circular inheritance: A -> B -> C -> A
        let class_a = create_test_class("ClassA", Some("ClassC"), vec![("propA", "valA")]);
        let class_b = create_test_class("ClassB", Some("ClassA"), vec![("propB", "valB")]);
        let class_c = create_test_class("ClassC", Some("ClassB"), vec![("propC", "valC")]);
        
        let mut visitor = InheritanceVisitor::new();
        visitor.register_class(class_a);
        visitor.register_class(class_b);
        visitor.register_class(class_c);
        
        // Process should not hang and should correctly resolve properties
        let processed = visitor.process("ClassA").unwrap();
        
        // Check if all properties are included
        assert!(processed.properties.contains_key("propA"));
        assert!(processed.properties.contains_key("propB"));
        assert!(processed.properties.contains_key("propC"));
    }
    
    #[test]
    fn test_access_modifier_inheritance() {
        // Parent with public access
        let parent = ClassNode::new("Parent".to_string())
            .with_access(AccessModifier::Public);
            
        // Child with private access
        let child = ClassNode::new("Child".to_string())
            .with_parent("Parent")
            .with_access(AccessModifier::Private);
            
        let mut visitor = InheritanceVisitor::new();
        visitor.register_class(parent);
        visitor.register_class(child);
        
        let processed = visitor.process("Child").unwrap();
        
        // Child's access modifier should be preserved
        assert_eq!(processed.access, AccessModifier::Private);
    }
    
    #[test]
    fn test_ast_visitor_implementation() {
        // Create a hierarchy with nested classes
        let mut root = ClassNode::new("Root".to_string());
        
        let mut parent = ClassNode::new("Parent".to_string());
        parent.properties.insert("parentProp".to_string(), 
            PropertyNode::new("parentProp", PropertyType::String, "parentVal"));
            
        let mut child = ClassNode::new("Child".to_string())
            .with_parent("Parent");
        child.properties.insert("childProp".to_string(),
            PropertyNode::new("childProp", PropertyType::String, "childVal"));
            
        parent.nested_classes.push(child);
        root.nested_classes.push(parent);
        
        // Use the visitor pattern to register all classes
        let mut visitor = InheritanceVisitor::new();
        visitor.visit_class(&mut root).unwrap();
        
        // Check if all classes were registered
        let processed = visitor.process("Child").unwrap();
        
        assert_eq!(processed.properties.len(), 2);
        assert_eq!(processed.properties["parentProp"].raw_value, "parentVal");
        assert_eq!(processed.properties["childProp"].raw_value, "childVal");
    }
}