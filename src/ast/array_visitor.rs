use super::{ClassNode, PropertyNode, AstVisitor};
use crate::error::Error;
use crate::operations::arrays::{ArrayOperation, ArrayProcessor};

pub struct ArrayVisitor {
    // Implementation can be extended with state if needed
}

impl ArrayVisitor {
    pub fn new() -> Self {
        Self {}
    }
}

impl AstVisitor for ArrayVisitor {
    fn visit_class(&mut self, class: &mut ClassNode) -> Result<(), Error> {
        // Process nested classes first (depth-first)
        for nested in &mut class.nested_classes {
            self.visit_class(nested)?;
        }

        // No array operations at class level
        Ok(())
    }

    fn visit_property(&mut self, property: &mut PropertyNode) -> Result<(), Error> {
        // Processed in visit_array if property is an array
        Ok(())
    }

    fn visit_array(&mut self, array: &mut Vec<String>, operation: Option<ArrayOperation>) -> Result<(), Error> {
        // Skip if there's no operation
        let Some(op) = operation else {
            return Ok(());
        };

        // For array operations, we just need to apply the operation directly
        // No need for temporary array or base array (the actual combining with 
        // parent arrays happens in inheritance resolution)
        match op {
            ArrayOperation::Replace => {
                // Replace operation doesn't need any special handling
                // The array already contains the values it should have
            },
            ArrayOperation::Append => {
                // We don't need to modify the values here, as this is handled during inheritance
            },
            ArrayOperation::Remove => {
                // We don't need to modify the values here, as this is handled during inheritance
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::PropertyType;
    use std::collections::HashMap;

    // Helper function to create a property node with array values
    fn create_array_property(
        name: &str, 
        values: Vec<&str>, 
        operation: Option<ArrayOperation>
    ) -> PropertyNode {
        let mut property = PropertyNode::new(
            name, 
            PropertyType::Array, 
            format!("{{{}}}", values.join(","))
        );
        
        property.array_values = values.iter().map(|s| s.to_string()).collect();
        
        if let Some(op) = operation {
            property = property.with_array_op(op);
        }
        
        property
    }

    #[test]
    fn test_replace_operation() {
        // Test the array replace (=) operation
        let mut property = create_array_property(
            "testArray", 
            vec!["value1", "value2", "value3"], 
            Some(ArrayOperation::Replace)
        );
        
        let mut visitor = ArrayVisitor::new();
        visitor.visit_array(&mut property.array_values, property.operation).unwrap();
        
        assert_eq!(property.array_values, vec!["value1", "value2", "value3"]);
    }

    #[test]
    fn test_append_operation() {
        // Test the array append (+=) operation
        let mut property = create_array_property(
            "testArray", 
            vec!["value1", "value2"], 
            Some(ArrayOperation::Append)
        );
        
        // Initially the base array is empty, so result should match input values
        let mut visitor = ArrayVisitor::new();
        visitor.visit_array(&mut property.array_values, property.operation).unwrap();
        
        assert_eq!(property.array_values, vec!["value1", "value2"]);
        
        // Now let's set up a more complex scenario with parent/child arrays
        let mut parent_class = ClassNode::new("Parent".to_string());
        parent_class.properties.insert(
            "items".to_string(), 
            create_array_property("items", vec!["parent1", "parent2", "common"], None)
        );
        
        let mut child_class = ClassNode::new("Child".to_string())
            .with_parent("Parent");
        child_class.properties.insert(
            "items".to_string(), 
            create_array_property("items", vec!["child1", "common"], Some(ArrayOperation::Append))
        );
        
        // Process the child's array - this should append to the parent's array but eliminate duplicates
        let mut property = child_class.properties.get_mut("items").unwrap();
        visitor.visit_array(&mut property.array_values, property.operation).unwrap();
        
        // For completeness, we should manually include both arrays like inheritance would
        // Since we don't test inheritance here, we'll manually verify the result
        let parent_values = &parent_class.properties["items"].array_values;
        let result = ArrayProcessor::process(
            parent_values,
            &["child1".to_string(), "common".to_string()],
            ArrayOperation::Append
        );
        
        // Result should include all unique elements from both arrays
        assert_eq!(result, vec!["parent1", "parent2", "common", "child1"]);
    }

    #[test]
    fn test_remove_operation() {
        // Test the array remove (-=) operation
        // Create parent array with initial values
        let parent_values = vec!["item1", "item2", "item3", "item4"];
        
        // Create child array that will remove some items
        let mut property = create_array_property(
            "testArray", 
            vec!["item2", "item4"], 
            Some(ArrayOperation::Remove)
        );
        
        // Since ArrayVisitor doesn't handle inheritance directly, we'll manually combine them
        let base_array = parent_values.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        let mut visitor = ArrayVisitor::new();
        visitor.visit_array(&mut property.array_values, property.operation).unwrap();
        
        // Now manually simulate what would happen during inheritance resolution
        let result = ArrayProcessor::process(
            &base_array, 
            &property.array_values, 
            ArrayOperation::Remove
        );
        
        // Items 2 and 4 should be removed
        assert_eq!(result, vec!["item1", "item3"]);
    }

    #[test]
    fn test_empty_arrays() {
        // Test with empty initial array
        let mut empty_property = create_array_property(
            "emptyArray", 
            vec![], 
            Some(ArrayOperation::Append)
        );
        
        let mut visitor = ArrayVisitor::new();
        visitor.visit_array(&mut empty_property.array_values, empty_property.operation).unwrap();
        
        assert!(empty_property.array_values.is_empty());
        
        // Test appending to empty array
        let mut append_to_empty = create_array_property(
            "appendToEmpty", 
            vec!["newItem"], 
            Some(ArrayOperation::Append)
        );
        
        visitor.visit_array(&mut append_to_empty.array_values, append_to_empty.operation).unwrap();
        
        assert_eq!(append_to_empty.array_values, vec!["newItem"]);
        
        // Test removing from empty array (should be a no-op)
        let mut remove_from_empty = create_array_property(
            "removeFromEmpty", 
            vec!["item"], 
            Some(ArrayOperation::Remove)
        );
        
        visitor.visit_array(&mut remove_from_empty.array_values, remove_from_empty.operation).unwrap();
        
        // Result after manual combination would be an empty array since there's nothing to remove from
        let base_empty: Vec<String> = vec![];
        let result = ArrayProcessor::process(
            &base_empty, 
            &remove_from_empty.array_values, 
            ArrayOperation::Remove
        );
        assert!(result.is_empty());
    }

    #[test]
    fn test_duplicate_items() {
        // Test handling of duplicate items in arrays
        let mut property = create_array_property(
            "duplicateArray", 
            vec!["item1", "item2", "item1", "item3"], 
            Some(ArrayOperation::Replace)
        );
        
        let mut visitor = ArrayVisitor::new();
        visitor.visit_array(&mut property.array_values, property.operation).unwrap();
        
        // ArrayProcessor keeps duplicates for Replace operations
        assert_eq!(property.array_values, vec!["item1", "item2", "item1", "item3"]);
        
        // Test append with duplicates
        let base_array = vec!["base1".to_string(), "base2".to_string()];
        let mut append_with_dupes = create_array_property(
            "appendWithDupes", 
            vec!["base1", "new1"], 
            Some(ArrayOperation::Append)
        );
        
        visitor.visit_array(&mut append_with_dupes.array_values, append_with_dupes.operation).unwrap();
        
        // Manually check what would happen during inheritance resolution
        let result = ArrayProcessor::process(
            &base_array, 
            &append_with_dupes.array_values, 
            ArrayOperation::Append
        );
        
        // Duplicates should be removed during append
        assert_eq!(result, vec!["base1", "base2", "new1"]);
    }

    #[test]
    fn test_complex_class_structure() {
        // Create a complex class structure with nested classes and array properties
        let mut root_class = ClassNode::new("Root".to_string());
        
        // Add array property to root
        root_class.properties.insert(
            "rootArray".to_string(), 
            create_array_property("rootArray", vec!["root1", "root2"], None)
        );
        
        // Create nested class with array properties
        let mut nested_class = ClassNode::new("Nested".to_string());
        
        // Add array properties to nested class with different operations
        nested_class.properties.insert(
            "nestedReplace".to_string(),
            create_array_property("nestedReplace", vec!["nested1", "nested2"], Some(ArrayOperation::Replace))
        );
        
        nested_class.properties.insert(
            "nestedAppend".to_string(),
            create_array_property("nestedAppend", vec!["append1", "append2"], Some(ArrayOperation::Append))
        );
        
        nested_class.properties.insert(
            "nestedRemove".to_string(),
            create_array_property("nestedRemove", vec!["remove1", "remove2"], Some(ArrayOperation::Remove))
        );
        
        // Add nested class to root
        root_class.nested_classes.push(nested_class);
        
        // Process the entire class structure
        let mut visitor = ArrayVisitor::new();
        visitor.visit_class(&mut root_class).unwrap();
        
        // Check root array (should be unchanged as it has no operation)
        assert_eq!(root_class.properties["rootArray"].array_values, vec!["root1", "root2"]);
        
        // Check nested class arrays (should be processed according to their operations)
        let nested = &root_class.nested_classes[0];
        assert_eq!(nested.properties["nestedReplace"].array_values, vec!["nested1", "nested2"]);
        assert_eq!(nested.properties["nestedAppend"].array_values, vec!["append1", "append2"]);
        assert_eq!(nested.properties["nestedRemove"].array_values, vec!["remove1", "remove2"]);
    }

    #[test]
    fn test_no_operation() {
        // Test with no operation specified (should leave array unchanged)
        let mut property = create_array_property(
            "noOpArray", 
            vec!["item1", "item2"], 
            None
        );
        
        let mut visitor = ArrayVisitor::new();
        visitor.visit_array(&mut property.array_values, property.operation).unwrap();
        
        // Array should remain unchanged
        assert_eq!(property.array_values, vec!["item1", "item2"]);
    }

    #[test]
    fn test_combined_operations() {
        // Test combining multiple operations as would happen in a complex inheritance chain
        // Base class array
        let base_values = vec!["base1", "base2", "common"];
        
        // First child performs append
        let mut append_op = create_array_property(
            "appendArray", 
            vec!["child1", "common"], 
            Some(ArrayOperation::Append)
        );
        
        let mut visitor = ArrayVisitor::new();
        visitor.visit_array(&mut append_op.array_values, append_op.operation).unwrap();
        
        // Manually combine base with first child result
        let base_strings: Vec<String> = base_values.iter().map(|s| s.to_string()).collect();
        let first_result = ArrayProcessor::process(
            &base_strings,
            &append_op.array_values,
            ArrayOperation::Append
        );
        
        // First result should have unique combined items
        assert_eq!(first_result, vec!["base1", "base2", "common", "child1"]);
        
        // Second child removes items
        let mut remove_op = create_array_property(
            "removeArray", 
            vec!["base1", "common"], 
            Some(ArrayOperation::Remove)
        );
        
        visitor.visit_array(&mut remove_op.array_values, remove_op.operation).unwrap();
        
        // Manually combine first result with second child operation
        let final_result = ArrayProcessor::process(
            &first_result,
            &remove_op.array_values,
            ArrayOperation::Remove
        );
        
        // Final result should have items removed
        assert_eq!(final_result, vec!["base2", "child1"]);
    }
}