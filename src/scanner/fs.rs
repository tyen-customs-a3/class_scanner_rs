use std::path::{Path, PathBuf};
use std::io;
use tempfile::{Builder, TempDir};
use log::debug;

use crate::error::{Result, ScannerError};

pub struct FileSystem {
    temp_dir_prefix: String,
    max_file_size: usize,
}

impl FileSystem {
    pub fn new(temp_dir_prefix: String, max_file_size: usize) -> Self {
        Self {
            temp_dir_prefix,
            max_file_size,
        }
    }

    pub fn create_temp_dir(&self) -> Result<TempDir> {
        debug!("Creating temporary directory with prefix: {}", self.temp_dir_prefix);
        Builder::new()
            .prefix(&self.temp_dir_prefix)
            .tempdir()
            .map_err(|e| ScannerError::FileReadError(e).into())
    }

    pub fn find_pbo_files(&self, directory: &Path) -> Result<Vec<PathBuf>> {
        debug!("Finding PBO files in directory: {}", directory.display());
        Ok(walkdir::WalkDir::new(directory)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().map_or(false, |ext| ext == "pbo"))
            .map(|e| e.path().to_owned())
            .collect())
    }

    pub fn validate_path(&self, path: &Path) -> Result<()> {
        if !path.exists() {
            return Err(ScannerError::FileReadError(
                io::Error::new(io::ErrorKind::NotFound, "File not found")
            ).into());
        }
        Ok(())
    }

    pub fn check_file_size(&self, path: &Path) -> Result<()> {
        if let Ok(metadata) = path.metadata() {
            if metadata.len() as usize > self.max_file_size {
                return Err(ScannerError::FileTooLarge { 
                    path: path.to_owned() 
                }.into());
            }
        }
        Ok(())
    }
}