use std::ops::RangeInclusive;
use regex::Regex;
use crate::models::error::{Error, Result};
use super::values::PropertyValue;

#[derive(Debug, Clone)]
pub struct PropertyValidator {
    integer_range: Option<RangeInclusive<i64>>,
    float_range: Option<RangeInclusive<f64>>,
    string_pattern: Option<Regex>,
    allowed_values: Option<Vec<String>>,
    min_array_size: Option<usize>,
    max_array_size: Option<usize>,
}

impl Default for PropertyValidator {
    fn default() -> Self {
        Self {
            integer_range: None,
            float_range: None,
            string_pattern: None,
            allowed_values: None,
            min_array_size: None,
            max_array_size: None,
        }
    }
}

impl PropertyValidator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_integer_range(mut self, range: RangeInclusive<i64>) -> Self {
        self.integer_range = Some(range);
        self
    }

    pub fn with_float_range(mut self, range: RangeInclusive<f64>) -> Self {
        self.float_range = Some(range);
        self
    }

    pub fn with_pattern(mut self, pattern: &str) -> Result<Self> {
        self.string_pattern = Some(Regex::new(pattern).map_err(|e| Error::Validation(e.to_string()))?);
        Ok(self)
    }

    pub fn with_allowed_values(mut self, values: Vec<String>) -> Self {
        self.allowed_values = Some(values);
        self
    }

    pub fn with_array_size_range(mut self, min: Option<usize>, max: Option<usize>) -> Self {
        self.min_array_size = min;
        self.max_array_size = max;
        self
    }

    pub fn validate(&self, value: &PropertyValue) -> Result<()> {
        match value {
            PropertyValue::Number(_) | PropertyValue::Integer(_) => self.validate_number(value),
            PropertyValue::String(_) | PropertyValue::Identifier(_) => self.validate_string(value),
            PropertyValue::Array(_) => self.validate_array(value),
            _ => Ok(()),
        }
    }

    fn validate_number(&self, value: &PropertyValue) -> Result<()> {
        if let Some(range) = &self.integer_range {
            if let Some(num) = value.as_integer() {
                if !range.contains(&num) {
                    return Err(Error::Validation(format!(
                        "Integer value {} outside valid range {:?}",
                        num, range
                    )));
                }
            }
        }

        if let Some(range) = &self.float_range {
            if let Some(num) = value.as_number() {
                if !range.contains(&num) {
                    return Err(Error::Validation(format!(
                        "Float value {} outside valid range {:?}",
                        num, range
                    )));
                }
            }
        }

        Ok(())
    }

    fn validate_string(&self, value: &PropertyValue) -> Result<()> {
        let str_val = value.as_string().ok_or_else(|| Error::Validation("Expected string value".to_string()))?;

        if let Some(pattern) = &self.string_pattern {
            if !pattern.is_match(str_val) {
                return Err(Error::Validation(format!(
                    "String value '{}' does not match pattern {}",
                    str_val, pattern
                )));
            }
        }

        if let Some(allowed) = &self.allowed_values {
            if !allowed.contains(str_val) {
                return Err(Error::Validation(format!(
                    "Value '{}' not in allowed values: {:?}",
                    str_val, allowed
                )));
            }
        }

        Ok(())
    }

    fn validate_array(&self, value: &PropertyValue) -> Result<()> {
        let array = value.as_array().ok_or_else(|| Error::Validation("Expected array value".to_string()))?;

        if let Some(min) = self.min_array_size {
            if array.len() < min {
                return Err(Error::Validation(format!(
                    "Array has {} elements, minimum is {}",
                    array.len(), min
                )));
            }
        }

        if let Some(max) = self.max_array_size {
            if array.len() > max {
                return Err(Error::Validation(format!(
                    "Array has {} elements, maximum is {}",
                    array.len(), max
                )));
            }
        }

        Ok(())
    }
}