use std::path::{Path, PathBuf};
use std::collections::HashMap;
use pbo_tools_rs::core::{PboApi, PboConfig, PboApiOps};
use pbo_tools_rs::error::types::ExtractError;
use tempfile::TempDir;
use pathdiff::diff_paths;
use log::{debug, trace};

use crate::error::{Result, ScannerError};
use crate::models::ClassData;
use crate::parser::ClassParser;

pub struct PboInfo {
    pub source: String,
    pub prefix: String,
}

pub struct PboScanner {
    api: PboApi,
    max_file_size: usize,
}

impl PboScanner {
    pub fn new(timeout: u32, max_file_size: usize) -> Self {
        Self {
            api: PboApi::builder()
                .with_config(PboConfig::builder()
                    .case_sensitive(true)
                    .max_retries(3)
                    .build())
                .with_timeout(timeout)
                .build(),
            max_file_size,
        }
    }

    pub fn extract_files(&self, path: &Path, temp_dir: &TempDir) -> Result<HashMap<String, String>> {
        debug!("Extracting files from PBO: {}", path.display());
        
        let result = self.api.extract_files(path, temp_dir.path(), None)
            .map_err(|e| ScannerError::PboScanError {
                path: path.to_owned(),
                source: Box::new(e),
            })?;

        if !result.is_success() {
            return Err(ScannerError::PboScanError {
                path: path.to_owned(),
                source: Box::new(result.get_error_message()
                    .map(|msg| ExtractError::CommandFailed {
                        cmd: "extractpbo".to_string(),
                        reason: msg,
                    })
                    .unwrap_or_else(|| ExtractError::NoFiles)),
            }.into());
        }

        let mut code_files = HashMap::new();
        for entry in walkdir::WalkDir::new(temp_dir.path())
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok()) {
                let file_path = entry.path();
                if file_path.extension().map_or(false, |ext| ext == "cpp" || ext == "hpp") {
                    if let Ok(metadata) = file_path.metadata() {
                        if metadata.len() as usize > self.max_file_size {
                            debug!("Skipping large file: {}", file_path.display());
                            continue;
                        }
                    }
                    if let Ok(content) = std::fs::read_to_string(file_path) {
                        let rel_path = diff_paths(file_path, temp_dir.path())
                            .unwrap_or_else(|| file_path.to_path_buf())
                            .to_string_lossy()
                            .into_owned();
                        code_files.insert(rel_path, content);
                    }
                }
        }

        trace!("Found {} code files in PBO", code_files.len());
        Ok(code_files)
    }

    pub fn get_info(&self, path: &Path) -> Result<PboInfo> {
        debug!("Getting PBO info for: {}", path.display());
        
        let result = self.api.list_contents(path)
            .map_err(|e| ScannerError::PboScanError {
                path: path.to_owned(),
                source: Box::new(e),
            })?;

        let prefix = result.get_prefix().unwrap_or_default().to_string();
        let source = path.parent()
            .and_then(|p| p.parent())
            .and_then(|p| p.file_name())
            .map_or(String::new(), |name| {
                name.to_string_lossy()
                    .trim_start_matches('@')
                    .to_string()
            });

        Ok(PboInfo { source, prefix })
    }
}