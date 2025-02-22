use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use super::ClassData;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PboScanData {
    pub classes: HashMap<String, ClassData>,
    pub source: String,
    pub prefix: String,
}

impl PboScanData {
    pub fn new(
        source: impl Into<String>,
        prefix: impl Into<String>,
        classes: HashMap<String, ClassData>
    ) -> Self {
        Self {
            classes,
            source: source.into(),
            prefix: prefix.into(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.classes.is_empty()
    }

    pub fn len(&self) -> usize {
        self.classes.len()
    }
}