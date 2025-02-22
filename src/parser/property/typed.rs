use std::collections::HashMap;
use std::marker::PhantomData;
use crate::models::error::{Error, Result};
use super::values::PropertyValue;
use super::keys::PropertyKey;

pub struct TypedProperty<T> {
    name: String,
    _phantom: PhantomData<T>,
}

impl<T> TypedProperty<T> 
where 
    for<'a> T: TryFrom<&'a PropertyValue, Error = Error>
{
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            _phantom: PhantomData,
        }
    }
    
    pub fn get(&self, properties: &HashMap<String, PropertyValue>) -> Result<T> {
        properties
            .get(&self.name)
            .ok_or_else(|| Error::PropertyMissing(self.name.clone()))
            .and_then(|v| T::try_from(v))
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}