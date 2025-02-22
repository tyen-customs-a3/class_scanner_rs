#[derive(Debug, Clone)]
pub struct ParserConfig {
    /// Maximum depth for nested class parsing
    pub max_depth: u32,
    /// Whether to allow empty block definitions
    pub allow_empty_blocks: bool,
    /// Whether parsing should be case-sensitive
    pub case_sensitive: bool,
}

impl Default for ParserConfig {
    fn default() -> Self {
        Self {
            max_depth: 32,
            allow_empty_blocks: false,
            case_sensitive: true,
        }
    }
}

impl ParserConfig {
    pub fn with_max_depth(mut self, depth: u32) -> Self {
        self.max_depth = depth;
        self
    }

    pub fn with_empty_blocks(mut self, allow: bool) -> Self {
        self.allow_empty_blocks = allow;
        self
    }

    pub fn case_insensitive(mut self) -> Self {
        self.case_sensitive = false;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = ParserConfig::default();
        assert_eq!(config.max_depth, 32);
        assert!(!config.allow_empty_blocks);
        assert!(config.case_sensitive);
    }

    #[test]
    fn test_config_builders() {
        let config = ParserConfig::default()
            .with_max_depth(16)
            .with_empty_blocks(true)
            .case_insensitive();

        assert_eq!(config.max_depth, 16);
        assert!(config.allow_empty_blocks);
        assert!(!config.case_sensitive);
    }
}