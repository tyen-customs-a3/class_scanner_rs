#[derive(Debug, Clone)]
pub struct ScannerConfig {
    pub debug: bool,
    pub max_file_size: usize,
    pub parse_timeout: u64,
    pub parallel: bool,
    pub temp_dir_prefix: String,
}

impl Default for ScannerConfig {
    fn default() -> Self {
        Self {
            debug: false,
            max_file_size: 100_000_000,
            parse_timeout: 30,
            parallel: false,
            temp_dir_prefix: "class_scanner".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct ScannerBuilder {
    config: ScannerConfig,
}

impl ScannerBuilder {
    pub fn new() -> Self {
        Self {
            config: ScannerConfig::default(),
        }
    }

    pub fn debug(mut self, debug: bool) -> Self {
        self.config.debug = debug;
        self
    }

    pub fn max_file_size(mut self, size: usize) -> Self {
        self.config.max_file_size = size;
        self
    }

    pub fn parse_timeout(mut self, timeout: u64) -> Self {
        self.config.parse_timeout = timeout;
        self
    }

    pub fn parallel(mut self, enabled: bool) -> Self {
        self.config.parallel = enabled;
        self
    }

    pub fn temp_dir_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.config.temp_dir_prefix = prefix.into();
        self
    }

    pub fn build(self) -> ScannerConfig {
        self.config
    }
}