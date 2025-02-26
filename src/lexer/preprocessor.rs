use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;
use crate::error::Error;
use crate::utils::{INCLUDE_PATTERN, DEFINE_PATTERN, PathResolver};

pub struct Preprocessor {
    defines: HashMap<String, String>,
    path_resolver: PathResolver,
    processed_files: Vec<PathBuf>,
}

impl Preprocessor {
    pub fn new<P: AsRef<Path>>(base_path: P) -> Self {
        Self {
            defines: HashMap::new(),
            path_resolver: PathResolver::new(base_path),
            processed_files: Vec::new(),
        }
    }

    pub fn add_include_path<P: AsRef<Path>>(&mut self, path: P) {
        self.path_resolver.add_include_path(path);
    }

    pub fn process_file<P: AsRef<Path>>(&mut self, file_path: P) -> Result<String, Error> {
        let file_path = file_path.as_ref().to_path_buf();
        
        // Check for circular includes
        if self.processed_files.contains(&file_path) {
            return Ok(String::new()); // Skip already processed files
        }
        self.processed_files.push(file_path.clone());

        let content = fs::read_to_string(&file_path)?;
        self.process_content(&content, &file_path)
    }

    fn process_content(&mut self, content: &str, source_file: &Path) -> Result<String, Error> {
        let mut result = String::new();
        let mut lines = content.lines().peekable();

        while let Some(line) = lines.next() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with("//") {
                // Preserve empty lines and comments
                result.push_str(line);
                result.push('\n');
            } else if trimmed.starts_with("#") {
                if let Some(captures) = INCLUDE_PATTERN.captures(line) {
                    let include_path = captures.get(1).unwrap().as_str();
                    let resolved_path = self.path_resolver.resolve_include(include_path, source_file)?;
                    let included_content = self.process_file(resolved_path)?;
                    result.push_str(&included_content);
                    if !included_content.ends_with('\n') {
                        result.push('\n');
                    }
                } else if let Some(captures) = DEFINE_PATTERN.captures(line) {
                    let name = captures.get(1).unwrap().as_str();
                    let value = captures.get(2).map(|m| m.as_str().trim()).unwrap_or("");
                    self.defines.insert(name.to_string(), value.to_string());
                    // Skip adding the #define line to the output
                }
                // Skip other preprocessor directives
            } else {
                // Process any defines in the line
                let processed_line = self.process_defines(line);
                let processed_trimmed = processed_line.trim();
                if !processed_trimmed.is_empty() && !processed_trimmed.starts_with('#') {
                    result.push_str(&processed_line);
                    result.push('\n');
                }
            }
        }

        Ok(result)
    }

    fn process_defines(&self, line: &str) -> String {
        let mut result = line.to_string();
        for (name, value) in &self.defines {
            result = result.replace(name, value);
        }
        result.trim_end().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::fs::File;
    use std::io::Write;
    use tempfile::TempDir;

    fn setup_test_files() -> (TempDir, PathBuf) {
        let temp_dir = TempDir::new().unwrap();
        let base_path = temp_dir.path().to_path_buf();
        
        // Create main file
        let main_content = r#"
            #include "header.h"
            #define MAX_SIZE 100
            class Test {
                public string name;
            }
        "#;
        let main_file = base_path.join("main.txt");
        File::create(&main_file).unwrap()
            .write_all(main_content.as_bytes()).unwrap();
        
        // Create header file
        let header_content = r#"
            #define VERSION "1.0"
            class Header {
                private int value;
            }
        "#;
        let header_file = base_path.join("header.h");
        File::create(&header_file).unwrap()
            .write_all(header_content.as_bytes()).unwrap();

        (temp_dir, base_path)
    }

    #[test]
    fn test_basic_preprocessing() {
        let (temp_dir, base_path) = setup_test_files();
        let mut preprocessor = Preprocessor::new(&base_path);
        
        let result = preprocessor.process_file(base_path.join("main.txt")).unwrap();
        
        // Check if header content is included
        assert!(result.contains("class Header"));
        assert!(result.contains("private int value"));
        
        // Check if main content is present
        assert!(result.contains("class Test"));
        assert!(result.contains("public string name"));
        
        // Clean up is automatic when temp_dir is dropped
    }

    #[test]
    fn test_circular_include() {
        let temp_dir = TempDir::new().unwrap();
        let base_path = temp_dir.path().to_path_buf();
        
        // Create files with circular includes
        let file1_content = "#include \"file2.h\"\nclass File1 {}";
        let file2_content = "#include \"file1.h\"\nclass File2 {}";
        
        File::create(base_path.join("file1.h")).unwrap()
            .write_all(file1_content.as_bytes()).unwrap();
        File::create(base_path.join("file2.h")).unwrap()
            .write_all(file2_content.as_bytes()).unwrap();
        
        let mut preprocessor = Preprocessor::new(&base_path);
        let result = preprocessor.process_file(base_path.join("file1.h")).unwrap();
        
        // Each file should only be included once
        assert_eq!(result.matches("class File1").count(), 1);
        assert_eq!(result.matches("class File2").count(), 1);
    }

    #[test]
    fn test_include_paths() {
        let temp_dir = TempDir::new().unwrap();
        let base_path = temp_dir.path().to_path_buf();
        let include_dir = base_path.join("include");
        fs::create_dir(&include_dir).unwrap();
        
        // Create a header in the include directory
        let header_content = "class SharedHeader {}";
        File::create(include_dir.join("shared.h")).unwrap()
            .write_all(header_content.as_bytes()).unwrap();
        
        // Create main file that includes from include path
        let main_content = "#include \"shared.h\"\nclass Main {}";
        File::create(base_path.join("main.txt")).unwrap()
            .write_all(main_content.as_bytes()).unwrap();
        
        let mut preprocessor = Preprocessor::new(&base_path);
        preprocessor.add_include_path(&include_dir);
        
        let result = preprocessor.process_file(base_path.join("main.txt")).unwrap();
        
        assert!(result.contains("class SharedHeader"));
        assert!(result.contains("class Main"));
    }

    #[test]
    fn test_define_replacement() {
        let temp_dir = TempDir::new().unwrap();
        let base_path = temp_dir.path().to_path_buf();
        
        let content = r#"
            #define VERSION "1.0"
            #define MAX_SIZE 100
            string version = VERSION;
            int size = MAX_SIZE;
        "#;
        
        File::create(base_path.join("test.txt")).unwrap()
            .write_all(content.as_bytes()).unwrap();
        
        let mut preprocessor = Preprocessor::new(&base_path);
        let result = preprocessor.process_file(base_path.join("test.txt")).unwrap();
        
        assert!(result.contains(r#"string version = "1.0""#));
        assert!(result.contains("int size = 100"));
    }

    #[test]
    fn test_file_not_found() {
        let temp_dir = TempDir::new().unwrap();
        let base_path = temp_dir.path().to_path_buf();
        let mut preprocessor = Preprocessor::new(&base_path);
        
        let result = preprocessor.process_file(base_path.join("nonexistent.txt"));
        assert!(result.is_err());
    }

    #[test]
    fn test_define_without_value() {
        let temp_dir = TempDir::new().unwrap();
        let base_path = temp_dir.path().to_path_buf();
        
        let content = r#"
            #define _ARMA_
            class Test {
                value = 123;
            };
        "#;
        
        File::create(base_path.join("test.txt")).unwrap()
            .write_all(content.as_bytes()).unwrap();
        
        let mut preprocessor = Preprocessor::new(&base_path);
        let result = preprocessor.process_file(base_path.join("test.txt")).unwrap();
        
        // The #define line should be removed
        assert!(!result.contains("#define"));
        assert!(!result.contains("_ARMA_"));
        assert!(result.contains("class Test"));
    }
}