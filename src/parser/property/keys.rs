use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PropertyKey {
    name: String,
}

impl PropertyKey {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into()
        }
    }

    pub fn as_str(&self) -> &str {
        &self.name
    }
}

impl From<PropertyKey> for String {
    fn from(key: PropertyKey) -> Self {
        key.name
    }
}

impl fmt::Display for PropertyKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}