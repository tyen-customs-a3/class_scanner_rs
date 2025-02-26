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