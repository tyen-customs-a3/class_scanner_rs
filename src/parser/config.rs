#[derive(Debug, Clone, Copy)]
pub struct ParserConfig {
    /// Whether string comparisons are case-sensitive
    pub case_sensitive: bool,
    /// Maximum nesting depth for blocks
    pub max_depth: usize,
    /// Whether to allow empty blocks in the parse result
    pub allow_empty_blocks: bool,
}

impl Default for ParserConfig {
    fn default() -> Self {
        Self {
            case_sensitive: true,
            max_depth: 32,
            allow_empty_blocks: true,
        }
    }
}

impl ParserConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_case_sensitivity(mut self, case_sensitive: bool) -> Self {
        self.case_sensitive = case_sensitive;
        self
    }

    pub fn with_max_depth(mut self, max_depth: usize) -> Self {
        self.max_depth = max_depth;
        self
    }

    pub fn with_empty_blocks(mut self, allow_empty_blocks: bool) -> Self {
        self.allow_empty_blocks = allow_empty_blocks;
        self
    }
}