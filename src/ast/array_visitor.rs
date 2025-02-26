use super::{ClassNode, PropertyNode, AstVisitor};
use crate::error::Error;
use crate::operations::arrays::{ArrayOperation, ArrayProcessor};

pub struct ArrayVisitor;

impl ArrayVisitor {
    pub fn new() -> Self {
        Self
    }
}

impl AstVisitor for ArrayVisitor {
    fn visit_class(&mut self, _class: &mut ClassNode) -> Result<(), Error> {
        // No class-level processing needed
        Ok(())
    }

    fn visit_property(&mut self, _property: &mut PropertyNode) -> Result<(), Error> {
        // Property-level processing handled in visit_array
        Ok(())
    }

    fn visit_array(&mut self, array: &mut Vec<String>, operation: Option<ArrayOperation>) -> Result<(), Error> {
        if let Some(op) = operation {
            let original = array.clone();
            *array = ArrayProcessor::process(&[], &original, op);
        }
        Ok(())
    }
}