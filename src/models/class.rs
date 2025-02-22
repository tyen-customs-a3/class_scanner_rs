use std::path::PathBuf;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use super::PropertyValue;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassData {
    pub name: String,
    pub parent: String,
    pub properties: HashMap<String, PropertyValue>,
    pub source_file: PathBuf,
    pub addon: Option<String>,
    pub nested_classes: Vec<ClassData>,
}

impl ClassData {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            parent: String::new(),
            properties: HashMap::new(),
            source_file: PathBuf::from("unknown"),
            addon: None,
            nested_classes: Vec::new(),
        }
    }

    pub fn with_parent(mut self, parent: impl Into<String>) -> Self {
        self.parent = parent.into();
        self
    }

    pub fn with_source(mut self, source: impl Into<PathBuf>) -> Self {
        self.source_file = source.into();
        self
    }

    pub fn with_addon(mut self, addon: impl Into<String>) -> Self {
        self.addon = Some(addon.into());
        self
    }

    pub fn find_nested_class(&self, name: &str) -> Option<&ClassData> {
        if self.name == name {
            return Some(self);
        }
        self.nested_classes.iter()
            .find_map(|class| class.find_nested_class(name))
    }

    pub fn get_all_nested_classes(&self) -> Vec<&ClassData> {
        let mut result = Vec::new();
        for class in &self.nested_classes {
            result.push(class);
            result.extend(class.get_all_nested_classes());
        }
        result
    }

    pub fn display_name(&self) -> Option<&str> {
        self.properties.get("displayName")
            .map(|p| p.raw_value.as_str())
    }
}