use std::path::{Path, PathBuf};
use crate::error::Error;

pub struct PathResolver {
    base_path: PathBuf,
    include_paths: Vec<PathBuf>,
}

impl PathResolver {
    pub fn new<P: AsRef<Path>>(base_path: P) -> Self {
        Self {
            base_path: base_path.as_ref().to_path_buf(),
            include_paths: Vec::new(),
        }
    }

    pub fn add_include_path<P: AsRef<Path>>(&mut self, path: P) {
        self.include_paths.push(path.as_ref().to_path_buf());
    }

    pub fn resolve_include(&self, include_path: &str, source_file: &Path) -> Result<PathBuf, Error> {
        // First try relative to the current file
        let source_dir = source_file.parent().unwrap_or(Path::new(""));
        let relative_path = source_dir.join(include_path);
        if (relative_path.exists()) {
            return Ok(relative_path);
        }

        // Try each include path
        for include_dir in &self.include_paths {
            let full_path = include_dir.join(include_path);
            if (full_path.exists()) {
                return Ok(full_path);
            }
        }

        // Finally try relative to base path
        let base_path = self.base_path.join(include_path);
        if (base_path.exists()) {
            return Ok(base_path);
        }

        Err(Error::IncludeError(
            include_path.to_string(),
            source_file.display().to_string(),
        ))
    }
}