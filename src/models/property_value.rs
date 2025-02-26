use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    pub fn new(name: &str, value: &str, value_type: crate::ast::PropertyType) -> Self {
        match value_type {
            crate::ast::PropertyType::String => {
                PropertyValue::String(value.to_string())
            },
            crate::ast::PropertyType::Number => {
                if let Ok(n) = value.parse() {
                    PropertyValue::Number(n)
                } else {
                    PropertyValue::String(value.to_string())
                }
            },
            crate::ast::PropertyType::Boolean => {
                if let Ok(b) = value.parse() {
                    PropertyValue::Bool(b)
                } else {
                    PropertyValue::String(value.to_string())
                }
            },
            crate::ast::PropertyType::Array => {
                PropertyValue::Array(Vec::new())
            },
            crate::ast::PropertyType::Object => {
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