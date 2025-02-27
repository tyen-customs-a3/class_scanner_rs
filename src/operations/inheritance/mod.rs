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