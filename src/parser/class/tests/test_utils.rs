use std::path::PathBuf;
use crate::parser::ParserConfig;
use crate::models::ClassData;

/// Common test configuration
pub fn create_test_config() -> ParserConfig {
    ParserConfig {
        max_depth: 32,
        allow_empty_blocks: false,
        case_sensitive: true,
    }
}

/// Creates a standard test class with common properties
pub fn create_test_class(name: &str) -> ClassData {
    ClassData::new(name.to_string())
}

/// Helper to create a test file path
pub fn test_file_path(addon: &str, file: &str) -> PathBuf {
    PathBuf::from(format!("D:/addons/{}/addons/{}", addon, file))
}

/// Test data for various class configurations
pub mod test_data {
    pub const SIMPLE_CLASS: &str = r#"
        class SimpleClass {
            value = 123;
            text = "test";
        };
    "#;

    pub const INHERITED_CLASS: &str = r#"
        class BaseClass {};
        class DerivedClass: BaseClass {
            value = 123;
        };
    "#;

    pub const NESTED_CLASS: &str = r#"
        class Outer {
            class Inner {
                value = 123;
            };
        };
    "#;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_test_class() {
        let class = create_test_class("Test");
        assert_eq!(class.name, "Test");
    }

    #[test]
    fn test_file_path_format() {
        let path = test_file_path("@test_addon", "config.cpp");
        assert!(path.to_string_lossy().contains("@test_addon"));
    }
}