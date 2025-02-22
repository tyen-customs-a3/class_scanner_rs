use serde::{Serialize, Deserialize};
use crate::models::error::{Error, Result};
use super::values::PropertyValue;
use super::keys::PropertyKey;
use super::validation::PropertyValidator;

#[derive(Debug, Clone)]
pub struct Property {
    key: PropertyKey,
    value: PropertyValue,
}

impl Property {
    pub fn new(key: PropertyKey, value: PropertyValue) -> Self {
        Self { key, value }
    }

    pub fn key(&self) -> &PropertyKey { &self.key }
    pub fn value(&self) -> &PropertyValue { &self.value }
    
    pub fn parse(key: PropertyKey, raw_value: &str, case_sensitive: bool) -> Result<Self> {
        Ok(Self::new(key, PropertyValue::parse(raw_value, case_sensitive)?))
    }

    pub fn validate_with(&self, validator: &PropertyValidator) -> Result<()> {
        validator.validate(&self.value)
    }

    pub fn with_validation(self, validator: &PropertyValidator) -> Result<Self> {
        self.validate_with(validator)?;
        Ok(self)
    }
}