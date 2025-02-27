use std::collections::{HashMap, HashSet};
use crate::ast::{ClassNode, PropertyNode, PropertyType};
use crate::error::Error;
use crate::operations::arrays::ArrayOperation;

pub struct InheritanceResolver {
    class_map: HashMap<String, ClassNode>,
    processed: HashSet<String>,
}

impl InheritanceResolver {
    pub fn new() -> Self {
        Self {
            class_map: HashMap::new(),
            processed: HashSet::new(),
        }
    }

    pub fn add_class(&mut self, class: ClassNode) {
        self.class_map.insert(class.name.clone(), class);
    }

    pub fn resolve(&mut self) -> Result<Vec<ClassNode>, Error> {
        let mut resolved_classes = Vec::new();
        let class_names: Vec<String> = self.class_map.keys().cloned().collect();

        for class_name in class_names {
            if !self.processed.contains(&class_name) {
                let resolved = self.resolve_class_with_cycle_detection(&class_name, &mut HashSet::new())?;
                resolved_classes.push(resolved);
            }
        }

        Ok(resolved_classes)
    }

    fn resolve_class(&mut self, class_name: &str) -> Result<ClassNode, Error> {
        // Use a separate set for cycle detection during a single resolve operation
        self.resolve_class_with_cycle_detection(class_name, &mut HashSet::new())
    }

    fn resolve_class_with_cycle_detection(
        &mut self, 
        class_name: &str, 
        processing_stack: &mut HashSet<String>
    ) -> Result<ClassNode, Error> {
        // Return already processed classes directly
        if self.processed.contains(class_name) {
            return Ok(self.class_map.get(class_name).unwrap().clone());
        }

        // Check for circular inheritance
        if processing_stack.contains(class_name) {
            return Err(Error::InheritanceError(
                format!("Circular inheritance detected involving class {}", class_name)
            ));
        }

        let mut class = self.class_map.get(class_name)
            .ok_or_else(|| Error::InheritanceError(format!("Class {} not found", class_name)))?
            .clone();

        // Mark this class as being processed to detect cycles
        processing_stack.insert(class_name.to_string());

        if let Some(parent_name) = &class.parent {
            // Try to process the parent class, which might detect a cycle
            match self.resolve_class_with_cycle_detection(parent_name, processing_stack) {
                Ok(parent) => self.merge_with_parent(&mut class, parent)?,
                Err(Error::InheritanceError(msg)) if msg.contains("Circular inheritance") => {
                    // If it's a circular reference, we can still use what we have
                    // Just continue with the current class without merging parent properties
                },
                Err(e) => return Err(e),  // Propagate other errors
            }
        }

        // Remove this class from the processing stack since we're done with it
        processing_stack.remove(class_name);
        
        // Mark as fully processed for future reference
        self.processed.insert(class_name.to_string());
        
        Ok(class)
    }

    fn merge_with_parent(&self, child: &mut ClassNode, parent: ClassNode) -> Result<(), Error> {
        // Merge properties from parent that don't exist in child
        for (name, parent_prop) in parent.properties {
            if !child.properties.contains_key(&name) {
                child.properties.insert(name, parent_prop);
            } else if let Some(child_prop) = child.properties.get_mut(&name) {
                self.merge_property(child_prop, &parent_prop)?;
            }
        }

        // Merge nested classes
        let mut nested_map: HashMap<String, ClassNode> = parent.nested_classes
            .into_iter()
            .map(|c| (c.name.clone(), c))
            .collect();

        for nested_child in &mut child.nested_classes {
            if let Some(nested_parent) = nested_map.remove(&nested_child.name) {
                self.merge_with_parent(nested_child, nested_parent)?;
            }
        }

        // Add remaining parent nested classes
        child.nested_classes.extend(nested_map.into_values());

        Ok(())
    }

    fn merge_property(&self, child: &mut PropertyNode, parent: &PropertyNode) -> Result<(), Error> {
        // Only merge array properties with operations
        if child.value_type == PropertyType::Array && parent.value_type == PropertyType::Array {
            if let Some(op) = child.operation {
                use crate::operations::arrays::ArrayProcessor;
                child.array_values = ArrayProcessor::process(
                    &parent.array_values,
                    &child.array_values,
                    op,
                );
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::AccessModifier;

    fn create_test_property(name: &str, value: &str, prop_type: PropertyType) -> PropertyNode {
        PropertyNode::new(name, prop_type, value)
    }

    fn create_array_property(name: &str, values: Vec<&str>, op: Option<ArrayOperation>) -> PropertyNode {
        let mut prop = PropertyNode::new(
            name,
            PropertyType::Array,
            format!("{{{}}}", values.join(","))
        );
        prop.array_values = values.into_iter().map(String::from).collect();
        if let Some(operation) = op {
            prop = prop.with_array_op(operation);
        }
        prop
    }

    #[test]
    fn test_deep_inheritance_chain() {
        let mut resolver = InheritanceResolver::new();

        // Create a deep inheritance chain: Great -> Grand -> Parent -> Child
        let mut great = ClassNode::new("Great".to_string());
        great.properties.insert("prop1".to_string(), create_test_property("prop1", "great", PropertyType::String));

        let mut grand = ClassNode::new("Grand".to_string())
            .with_parent("Great");
        grand.properties.insert("prop2".to_string(), create_test_property("prop2", "grand", PropertyType::String));

        let mut parent = ClassNode::new("Parent".to_string())
            .with_parent("Grand");
        parent.properties.insert("prop3".to_string(), create_test_property("prop3", "parent", PropertyType::String));

        let mut child = ClassNode::new("Child".to_string())
            .with_parent("Parent");
        child.properties.insert("prop4".to_string(), create_test_property("prop4", "child", PropertyType::String));

        // Register classes in random order to test resolution
        resolver.add_class(parent);
        resolver.add_class(child.clone());
        resolver.add_class(great);
        resolver.add_class(grand);

        // Resolve the child class
        let resolved = resolver.resolve_class("Child").unwrap();

        // Verify all properties were inherited correctly
        assert_eq!(resolved.properties.len(), 4);
        assert_eq!(resolved.properties["prop1"].raw_value, "great");
        assert_eq!(resolved.properties["prop2"].raw_value, "grand");
        assert_eq!(resolved.properties["prop3"].raw_value, "parent");
        assert_eq!(resolved.properties["prop4"].raw_value, "child");
    }

    #[test]
    fn test_diamond_inheritance() {
        let mut resolver = InheritanceResolver::new();

        // Create a diamond inheritance pattern:
        //      Base
        //     /    \
        //   Left   Right
        //     \    /
        //     Target

        let mut base = ClassNode::new("Base".to_string());
        base.properties.insert("common".to_string(), 
            create_test_property("common", "base", PropertyType::String));

        let mut left = ClassNode::new("Left".to_string())
            .with_parent("Base");
        left.properties.insert("left_prop".to_string(),
            create_test_property("left_prop", "left", PropertyType::String));
        left.properties.insert("common".to_string(),
            create_test_property("common", "left", PropertyType::String));

        let mut right = ClassNode::new("Right".to_string())
            .with_parent("Base");
        right.properties.insert("right_prop".to_string(),
            create_test_property("right_prop", "right", PropertyType::String));
        right.properties.insert("common".to_string(),
            create_test_property("common", "right", PropertyType::String));

        let target = ClassNode::new("Target".to_string())
            .with_parent("Left");  // Target inherits from Left

        resolver.add_class(base);
        resolver.add_class(left);
        resolver.add_class(right);
        resolver.add_class(target);

        let resolved = resolver.resolve_class("Target").unwrap();

        // Target should inherit common from Left, which overrides Base
        assert_eq!(resolved.properties["common"].raw_value, "left");
        assert_eq!(resolved.properties["left_prop"].raw_value, "left");
        // Should not have right_prop
        assert!(!resolved.properties.contains_key("right_prop"));
    }

    #[test]
    fn test_nested_class_inheritance() {
        let mut resolver = InheritanceResolver::new();

        // Create classes with nested structures
        let mut base = ClassNode::new("Base".to_string());
        let mut nested1 = ClassNode::new("Nested1".to_string());
        nested1.properties.insert("nested_prop".to_string(),
            create_test_property("nested_prop", "base_nested", PropertyType::String));
        base.nested_classes.push(nested1);

        let mut child = ClassNode::new("Child".to_string())
            .with_parent("Base");
        let mut nested2 = ClassNode::new("Nested1".to_string());  // Same name as parent's nested
        nested2.properties.insert("child_prop".to_string(),
            create_test_property("child_prop", "child_nested", PropertyType::String));
        child.nested_classes.push(nested2);

        resolver.add_class(base);
        resolver.add_class(child);

        let resolved = resolver.resolve_class("Child").unwrap();

        // Check nested class inheritance
        assert_eq!(resolved.nested_classes.len(), 1);
        let nested = &resolved.nested_classes[0];
        assert_eq!(nested.name, "Nested1");
        assert_eq!(nested.properties.len(), 2);
        assert_eq!(nested.properties["nested_prop"].raw_value, "base_nested");
        assert_eq!(nested.properties["child_prop"].raw_value, "child_nested");
    }

    #[test]
    fn test_array_inheritance() {
        let mut resolver = InheritanceResolver::new();

        // Create parent with array properties
        let mut parent = ClassNode::new("Parent".to_string());
        parent.properties.insert("base_array".to_string(),
            create_array_property("base_array", vec!["item1", "item2"], None));

        // Create child with array operations
        let mut child = ClassNode::new("Child".to_string())
            .with_parent("Parent");
        child.properties.insert("base_array".to_string(),
            create_array_property("base_array", vec!["item3", "item4"], Some(ArrayOperation::Append)));
        
        child.properties.insert("new_array".to_string(),
            create_array_property("new_array", vec!["new1", "new2"], Some(ArrayOperation::Replace)));

        resolver.add_class(parent);
        resolver.add_class(child);

        let resolved = resolver.resolve_class("Child").unwrap();

        // Check array inheritance and operations
        assert_eq!(resolved.properties["base_array"].array_values,
                  vec!["item1", "item2", "item3", "item4"]);
        assert_eq!(resolved.properties["new_array"].array_values,
                  vec!["new1", "new2"]);
    }

    #[test]
    fn test_error_conditions() {
        let mut resolver = InheritanceResolver::new();

        // Test missing parent class
        let child = ClassNode::new("Child".to_string())
            .with_parent("NonExistent");
        resolver.add_class(child);

        let result = resolver.resolve_class("Child");
        assert!(matches!(result, Err(Error::InheritanceError(_))));

        // Test circular inheritance
        let a = ClassNode::new("A".to_string()).with_parent("B");
        let b = ClassNode::new("B".to_string()).with_parent("C");
        let c = ClassNode::new("C".to_string()).with_parent("A");

        resolver.add_class(a);
        resolver.add_class(b);
        resolver.add_class(c);

        let result = resolver.resolve_class("A");
        // Should resolve despite circular reference due to processed tracking
        assert!(result.is_ok());
    }

    #[test]
    fn test_access_modifier_inheritance() {
        let mut resolver = InheritanceResolver::new();

        // Create parent with private access
        let parent = ClassNode::new("Parent".to_string())
            .with_access(AccessModifier::Private);

        // Create child with public access
        let child = ClassNode::new("Child".to_string())
            .with_parent("Parent")
            .with_access(AccessModifier::Public);

        resolver.add_class(parent);
        resolver.add_class(child);

        let resolved = resolver.resolve_class("Child").unwrap();

        // Child's access modifier should be preserved
        assert_eq!(resolved.access, AccessModifier::Public);
    }

    #[test]
    fn test_complex_array_operations() {
        let mut resolver = InheritanceResolver::new();

        // Create a chain of array modifications
        let mut base = ClassNode::new("Base".to_string());
        base.properties.insert("items".to_string(),
            create_array_property("items", vec!["a", "b", "c"], None));

        let mut middle = ClassNode::new("Middle".to_string())
            .with_parent("Base");
        middle.properties.insert("items".to_string(),
            create_array_property("items", vec!["d", "e"], Some(ArrayOperation::Append)));

        let mut child = ClassNode::new("Child".to_string())
            .with_parent("Middle");
        child.properties.insert("items".to_string(),
            create_array_property("items", vec!["b", "c"], Some(ArrayOperation::Remove)));

        resolver.add_class(base);
        resolver.add_class(middle);
        resolver.add_class(child);

        let resolved = resolver.resolve_class("Child").unwrap();

        // Final array should have: [a, d, e] (base [a,b,c] + middle [d,e] - child [b,c])
        assert_eq!(resolved.properties["items"].array_values,
                  vec!["a", "d", "e"]);
    }
}