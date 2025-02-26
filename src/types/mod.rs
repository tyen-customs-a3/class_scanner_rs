use crate::error::Error;
use crate::ast::PropertyType;

#[derive(Debug, Clone, PartialEq)]
pub enum TypeDefinition {
    String,
    Number,
    Boolean,
    Array(Box<TypeDefinition>),
    Object(Vec<(String, TypeDefinition)>),
}

pub struct TypeValidator;

impl TypeValidator {
    pub fn validate_value(value: &str, expected_type: &TypeDefinition) -> Result<(), Error> {
        match expected_type {
            TypeDefinition::String => Ok(()),
            TypeDefinition::Number => {
                value.parse::<f64>().map_err(|_| {
                    Error::TypeError(format!("Expected number, got: {}", value))
                })?;
                Ok(())
            }
            TypeDefinition::Boolean => {
                match value.to_lowercase().as_str() {
                    "true" | "false" => Ok(()),
                    _ => Err(Error::TypeError(format!("Expected boolean, got: {}", value))),
                }
            }
            TypeDefinition::Array(_) => {
                if !value.starts_with('{') || !value.ends_with('}') {
                    return Err(Error::TypeError("Invalid array format".to_string()));
                }
                Ok(())
            }
            TypeDefinition::Object(_) => {
                // Basic validation for object syntax
                if !value.starts_with('{') || !value.ends_with('}') {
                    return Err(Error::TypeError("Invalid object format".to_string()));
                }
                Ok(())
            }
        }
    }

    pub fn infer_type(value: &str) -> PropertyType {
        // Try boolean
        match value.to_lowercase().as_str() {
            "true" | "false" => return PropertyType::Boolean,
            _ => {}
        }

        // Try number
        if let Ok(_) = value.parse::<f64>() {
            return PropertyType::Number;
        }

        // Check for array/object
        if value.starts_with('{') && value.ends_with('}') {
            // Simple heuristic: if contains key-value pairs, it's an object
            if value.contains(':') {
                return PropertyType::Object;
            }
            return PropertyType::Array;
        }

        // Default to string
        PropertyType::String
    }

    pub fn convert_value(value: &str, target_type: &PropertyType) -> Result<String, Error> {
        match target_type {
            PropertyType::String => Ok(value.to_string()),
            PropertyType::Number => {
                value.parse::<f64>()
                    .map(|n| n.to_string())
                    .map_err(|_| Error::TypeError(format!("Cannot convert '{}' to number", value)))
            }
            PropertyType::Boolean => {
                match value.to_lowercase().as_str() {
                    "true" | "1" => Ok("true".to_string()),
                    "false" | "0" => Ok("false".to_string()),
                    _ => Err(Error::TypeError(format!("Cannot convert '{}' to boolean", value)))
                }
            }
            PropertyType::Array => {
                if value.starts_with('{') && value.ends_with('}') {
                    Ok(value.to_string())
                } else {
                    Ok(format!("{{{}}}", value))
                }
            }
            PropertyType::Object => {
                if value.starts_with('{') && value.ends_with('}') {
                    Ok(value.to_string())
                } else {
                    Err(Error::TypeError(format!("Cannot convert '{}' to object", value)))
                }
            }
        }
    }
}