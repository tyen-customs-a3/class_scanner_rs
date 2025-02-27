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