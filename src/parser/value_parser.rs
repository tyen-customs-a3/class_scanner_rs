use log::{trace, warn};
use crate::error::{Error, PropertyParserError, Result};
use crate::models::PropertyValueType;
use super::ParserConfig;

pub struct ValueParser {
    config: ParserConfig,
}

impl ValueParser {
    pub fn new(config: ParserConfig) -> Self {
        Self { config }
    }

    pub fn detect_type(&self, value: &str) -> Result<(PropertyValueType, String)> {
        let value = value.trim();
        
        if value.starts_with('"') && value.ends_with('"') {
            trace!("Detected string value: {}", value);
            return Ok((
                PropertyValueType::String,
                value[1..value.len()-1].to_string()
            ));
        }
        
        if value.eq_ignore_ascii_case("true") || value.eq_ignore_ascii_case("false") {
            trace!("Detected boolean value: {}", value);
            return Ok((
                PropertyValueType::Boolean,
                value.to_lowercase()
            ));
        }
        
        if value.parse::<f64>().is_ok() || value.parse::<i64>().is_ok() {
            trace!("Detected number value: {}", value);
            return Ok((
                PropertyValueType::Number,
                value.to_string()
            ));
        }

        if value.chars().all(|c| c.is_alphanumeric() || c == '_') {
            trace!("Detected identifier value: {}", value);
            return Ok((
                PropertyValueType::Identifier,
                if self.config.case_sensitive {
                    value.to_string()
                } else {
                    value.to_lowercase()
                }
            ));
        }
        
        warn!("Invalid property value: {}", value);
        Err(Error::Parser(PropertyParserError::InvalidProperty(
            format!("Invalid property value: {}", value)
        ).into()))
    }
}