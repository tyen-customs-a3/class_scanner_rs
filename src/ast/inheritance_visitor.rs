use std::collections::HashMap;
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
        if self.processed.contains(&class_name.to_string()) {
            return Ok(self.class_map.get(class_name).unwrap().clone());
        }

        let class = self.class_map.get(class_name)
            .ok_or_else(|| Error::InheritanceError(format!("Class {} not found", class_name)))?
            .clone();

        let mut result = class.clone();

        if let Some(parent_name) = &class.parent {
            let parent = self.process(parent_name)?;
            self.merge_properties(&mut result, &parent);
        }

        self.processed.push(class_name.to_string());
        Ok(result)
    }

    fn merge_properties(&self, child: &mut ClassNode, parent: &ClassNode) {
        // Copy properties from parent that aren't in child
        for (name, parent_prop) in &parent.properties {
            if !child.properties.contains_key(name) {
                child.properties.insert(name.clone(), parent_prop.clone());
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