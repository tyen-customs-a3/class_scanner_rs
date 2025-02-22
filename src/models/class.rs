use std::path::{Path, PathBuf};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::models::properties::PropertyValue;
use crate::models::properties::TypedProperty;
use crate::models::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassData {
    pub name: String,
    pub parent: String,
    pub properties: HashMap<String, PropertyValue>,
    pub nested_classes: Vec<ClassData>,
    pub source_file: Option<PathBuf>,
    pub addon: Option<String>,
}

impl ClassData {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            parent: String::new(),
            properties: HashMap::new(),
            nested_classes: Vec::new(),
            source_file: None,
            addon: None,
        }
    }

    pub fn with_parent(mut self, parent: impl Into<String>) -> Self {
        self.parent = parent.into();
        self
    }

    pub fn with_source(mut self, path: impl AsRef<Path>) -> Self {
        self.source_file = Some(path.as_ref().to_path_buf());
        self
    }

    pub fn with_addon(mut self, addon: impl Into<String>) -> Self {
        self.addon = Some(addon.into());
        self
    }
}
