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

        // Read the file as raw bytes first
        let content = fs::read_to_string(&file_path)?;
        self.process_content(&content, &file_path)
    }

    fn process_content(&mut self, content: &str, source_file: &Path) -> Result<String, Error> {
        let mut result = String::new();
        let mut current_line = String::new();
        let mut in_string = false;
        let mut in_comment = false;
        let mut in_multiline_comment = false;
        let mut escape_next = false;

        let mut chars = content.chars().peekable();
        while let Some(c) = chars.next() {
            match c {
                '\\' if in_string => {
                    escape_next = !escape_next;
                    current_line.push(c);
                }
                '"' => {
                    if !escape_next {
                        in_string = !in_string;
                    }
                    current_line.push(c);
                    escape_next = false;
                }
                '/' if !in_string && !escape_next => {
                    if let Some(&next) = chars.peek() {
                        match next {
                            '/' if !in_multiline_comment => {
                                in_comment = true;
                                current_line.push(c);
                                current_line.push(next);
                                chars.next();
                                continue;
                            }
                            '*' if !in_comment => {
                                in_multiline_comment = true;
                                current_line.push(c);
                                current_line.push(next);
                                chars.next();
                                continue;
                            }
                            _ => current_line.push(c)
                        }
                    }
                }
                '*' if in_multiline_comment && !in_string => {
                    if let Some(&'/') = chars.peek() {
                        in_multiline_comment = false;
                        current_line.push(c);
                        current_line.push('/');
                        chars.next();
                        continue;
                    }
                    current_line.push(c);
                }
                '\n' => {
                    if in_comment {
                        in_comment = false;
                    }
                    escape_next = false;
                    
                    if !in_multiline_comment {
                        if !current_line.trim().is_empty() {
                            if let Some(processed) = self.process_line(&current_line) {
                                result.push_str(&processed);
                                result.push('\n');
                            }
                        } else {
                            result.push('\n');
                        }
                        current_line.clear();
                    } else {
                        current_line.push('\n');
                    }
                }
                _ => {
                    escape_next = false;
                    current_line.push(c);
                }
            }
        }

        if !current_line.is_empty() {
            if let Some(processed) = self.process_line(&current_line) {
                result.push_str(&processed);
            }
        }

        Ok(result)
    }

    fn process_line(&mut self, line: &str) -> Option<String> {
        let trimmed = line.trim();
        
        if trimmed.is_empty() {
            return None;
        }

        // Always preserve array properties without preprocessing them
        if trimmed.contains("[]") {
            return Some(line.to_string());
        }

        if trimmed.starts_with('#') {
            if let Some(captures) = INCLUDE_PATTERN.captures(line) {
                let include_path = captures.get(1).unwrap().as_str();
                if let Ok(resolved_path) = self.path_resolver.resolve_include(include_path, Path::new("")) {
                    if let Ok(included_content) = self.process_file(resolved_path) {
                        return Some(included_content);
                    }
                }
                return None;
            } else if let Some(captures) = DEFINE_PATTERN.captures(line) {
                let name = captures.get(1).unwrap().as_str();
                let value = captures.get(2).map(|m| m.as_str().trim()).unwrap_or("");
                self.defines.insert(name.to_string(), value.to_string());
                return None;
            }
            return None;
        }

        // Process defines only when not in a string
        let mut result = String::with_capacity(line.len());
        let mut in_string = false;
        let mut escape_next = false;
        
        let chars: Vec<char> = line.chars().collect();
        let mut i = 0;
        while i < chars.len() {
            match chars[i] {
                '\\' if in_string => {
                    escape_next = !escape_next;
                    result.push(chars[i]);
                }
                '"' => {
                    if !escape_next {
                        in_string = !in_string;
                    }
                    result.push(chars[i]);
                    escape_next = false;
                }
                c if !in_string => {
                    let mut found_define = false;
                    for (name, value) in &self.defines {
                        if chars[i..].starts_with(&name.chars().collect::<Vec<_>>()) {
                            result.push_str(value);
                            i += name.len() - 1;
                            found_define = true;
                            break;
                        }
                    }
                    if !found_define {
                        result.push(c);
                    }
                }
                _ => {
                    escape_next = false;
                    result.push(chars[i]);
                }
            }
            i += 1;
        }

        Some(result)
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