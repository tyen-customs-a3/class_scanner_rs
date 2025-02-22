use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PropertyValueType {
    Array,
    String,
    Number,
    Boolean,
    Identifier,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyValue {
    pub name: String,
    pub raw_value: String,
    pub value_type: Option<PropertyValueType>,
    pub is_array: bool,
    pub array_values: Vec<String>,
}

impl PropertyValue {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            raw_value: String::new(),
            value_type: None,
            is_array: false,
            array_values: Vec::new(),
        }
    }

    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.raw_value = value.into();
        self
    }

    pub fn with_type(mut self, value_type: PropertyValueType) -> Self {
        self.value_type = Some(value_type);
        self
    }

    pub fn with_array_values(mut self, values: Vec<String>) -> Self {
        self.is_array = true;
        self.array_values = values;
        self.value_type = Some(PropertyValueType::Array);
        self
    }
}