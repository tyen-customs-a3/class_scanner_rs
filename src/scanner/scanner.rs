use std::collections::HashMap;
use std::path::{Path, PathBuf};
use log::{debug, warn, info};

#[cfg(feature = "parallel")]
use rayon::prelude::*;

use crate::error::{Result, ScannerError};
use crate::models::{ClassData, PboScanData};
use crate::parser::ClassParser;

use super::config::{ScannerConfig, ScannerBuilder};
use super::fs::FileSystem;
use super::pbo_scanner::{PboScanner, PboInfo};

pub struct Scanner {
    config: ScannerConfig,
    fs: FileSystem,
    pbo_scanner: PboScanner,
}

impl Scanner {
    pub fn builder() -> ScannerBuilder {
        ScannerBuilder::new()
    }

    pub fn new(config: ScannerConfig) -> Self {
        Self {
            fs: FileSystem::new(config.temp_dir_prefix.clone(), config.max_file_size),
            pbo_scanner: PboScanner::new(config.parse_timeout as u32, config.max_file_size),
            config,
        }
    }

    pub fn scan_directory<P: AsRef<Path>>(&self, directory: P) -> Result<HashMap<PathBuf, PboScanData>> {
        let directory = directory.as_ref();
        if !directory.exists() || !directory.is_dir() {
            debug!("Directory does not exist or is not a directory: {:?}", directory);
            return Ok(HashMap::new());
        }

        let pbo_files = self.fs.find_pbo_files(directory)?;
        info!("Found {} PBO files to scan", pbo_files.len());

        #[cfg(feature = "parallel")]
        if self.config.parallel {
            return self.scan_files_parallel(&pbo_files);
        }

        self.scan_files_sequential(&pbo_files)
    }

    pub fn scan_pbo<P: AsRef<Path>>(&self, path: P) -> Result<PboScanData> {
        let path = path.as_ref();
        self.fs.validate_path(path)?;
        self.fs.check_file_size(path)?;

        info!("Extracting PBO: {:?}", path);
        let temp_dir = self.fs.create_temp_dir()?;
        let code_files = self.pbo_scanner.extract_files(path, &temp_dir)?;

        if code_files.is_empty() {
            return Err(ScannerError::NoCodeFiles(path.to_owned()).into());
        }

        let classes = self.process_code_files(path, &code_files)?;
        let pbo_info = self.pbo_scanner.get_info(path)?;

        Ok(PboScanData::new(pbo_info.source, pbo_info.prefix, classes))
    }

    #[cfg(feature = "parallel")]
    fn scan_files_parallel(&self, files: &[PathBuf]) -> Result<HashMap<PathBuf, PboScanData>> {
        files.par_iter()
            .filter_map(|path| {
                match self.scan_pbo(path) {
                    Ok(data) => Some(Ok((path.clone(), data))),
                    Err(e) => {
                        warn!("Error scanning {:?}: {}", path, e);
                        None
                    }
                }
            })
            .collect()
    }

    fn scan_files_sequential(&self, files: &[PathBuf]) -> Result<HashMap<PathBuf, PboScanData>> {
        let mut results = HashMap::new();
        for path in files {
            match self.scan_pbo(path) {
                Ok(data) => {
                    results.insert(path.clone(), data);
                }
                Err(e) => {
                    warn!("Error scanning {:?}: {}", path, e);
                }
            }
        }
        Ok(results)
    }

    fn process_code_files(&self, pbo_path: &Path, files: &HashMap<String, String>) -> Result<HashMap<String, ClassData>> {
        let mut parser = ClassParser::new();
        let mut classes = Vec::new();

        info!("Parsing {} files from {:?}", files.len(), pbo_path);

        for (path, content) in files {
            match parser.parse_hierarchical(content) {
                Ok(mut file_classes) => classes.append(&mut file_classes),
                Err(e) => {
                    warn!("Failed to parse file {}: {}", path, e);
                }
            }
        }

        // Convert Vec<ClassData> to HashMap<String, ClassData>
        let class_map = classes.into_iter()
            .map(|class| (class.name.clone(), class))
            .collect();

        Ok(class_map)
    }
}