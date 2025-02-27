use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::ast::{PropertyNode, PropertyType};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PropertyValue {
    String(String),
    Number(f64),
    Bool(bool),
    Array(Vec<String>),
    Object(HashMap<String, PropertyValue>),
}

impl PropertyValue {
    pub fn new(name: &str, value: &str, value_type: PropertyType) -> Self {
        match value_type {
            PropertyType::String => {
                PropertyValue::String(value.to_string())
            },
            PropertyType::Number => {
                if let Ok(n) = value.parse() {
                    PropertyValue::Number(n)
                } else {
                    PropertyValue::String(value.to_string())
                }
            },
            PropertyType::Boolean => {
                if let Ok(b) = value.parse() {
                    PropertyValue::Bool(b)
                } else {
                    PropertyValue::String(value.to_string())
                }
            },
            PropertyType::Array => {
                PropertyValue::Array(Vec::new())
            },
            PropertyType::Object => {
                PropertyValue::Object(HashMap::new())
            },
        }
    }

    pub fn with_array(_name: &str, _raw_value: &str, values: Vec<String>) -> Self {
        PropertyValue::Array(values)
    }

    pub fn as_string(&self) -> Option<&str> {
        match self {
            PropertyValue::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_number(&self) -> Option<f64> {
        match self {
            PropertyValue::Number(n) => Some(*n),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            PropertyValue::Bool(b) => Some(*b),
            _ => None,
        }
    }

    pub fn as_array(&self) -> Option<&Vec<String>> {
        match self {
            PropertyValue::Array(a) => Some(a),
            _ => None,
        }
    }

    pub fn as_object(&self) -> Option<&HashMap<String, PropertyValue>> {
        match self {
            PropertyValue::Object(o) => Some(o),
            _ => None,
        }
    }
}

impl From<PropertyNode> for PropertyValue {
    fn from(node: PropertyNode) -> Self {
        match node.value_type {
            PropertyType::String => PropertyValue::String(node.raw_value),
            PropertyType::Number => {
                if let Ok(n) = node.raw_value.parse() {
                    PropertyValue::Number(n)
                } else {
                    PropertyValue::String(node.raw_value)
                }
            },
            PropertyType::Boolean => {
                if let Ok(b) = node.raw_value.parse() {
                    PropertyValue::Bool(b)
                } else {
                    PropertyValue::String(node.raw_value)
                }
            },
            PropertyType::Array => PropertyValue::Array(node.array_values),
            PropertyType::Object => PropertyValue::Object(HashMap::new()),
        }
    }
}