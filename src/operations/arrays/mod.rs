use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ArrayOperation {
    Append,   // +=
    Remove,   // -=
    Replace,  // =
}

pub struct ArrayProcessor;

impl ArrayProcessor {
    pub fn process(base: &[String], values: &[String], operation: ArrayOperation) -> Vec<String> {
        match operation {
            ArrayOperation::Append => Self::append_operation(base, values),
            ArrayOperation::Remove => Self::remove_operation(base, values),
            ArrayOperation::Replace => values.to_vec(),
        }
    }

    fn append_operation(base: &[String], to_append: &[String]) -> Vec<String> {
        // For arrays, we should preserve duplicates since position matters
        let mut result = base.to_vec();
        result.extend(to_append.iter().cloned());
                        result
    }

    fn remove_operation(base: &[String], to_remove: &[String]) -> Vec<String> {
        let remove_set: HashSet<_> = to_remove.iter().collect();
        base.iter()
            .filter(|item| !remove_set.contains(item))
            .cloned()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_operations() {
        let base = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        
        // Test append
        let append = vec!["d".to_string(), "e".to_string()];
        let result = ArrayProcessor::process(&base, &append, ArrayOperation::Append);
        assert_eq!(result, vec!["a", "b", "c", "d", "e"]);
        
        // Test remove
        let remove = vec!["b".to_string(), "c".to_string()];
        let result = ArrayProcessor::process(&base, &remove, ArrayOperation::Remove);
        assert_eq!(result, vec!["a"]);
        
        // Test replace
        let replace = vec!["x".to_string(), "y".to_string()];
        let result = ArrayProcessor::process(&base, &replace, ArrayOperation::Replace);
        assert_eq!(result, vec!["x", "y"]);
    }

    #[test]
    fn test_edge_cases() {
        // Empty base array
        let empty: Vec<String> = vec![];
        let values = vec!["a".to_string(), "b".to_string()];
        
        assert_eq!(
            ArrayProcessor::process(&empty, &values, ArrayOperation::Append),
            values,
            "Appending to empty array should return the values"
        );
        
        assert_eq!(
            ArrayProcessor::process(&empty, &values, ArrayOperation::Remove),
            empty,
            "Removing from empty array should return empty array"
        );

        // Empty values array
        let base = vec!["a".to_string(), "b".to_string()];
        let empty: Vec<String> = vec![];
        
        assert_eq!(
            ArrayProcessor::process(&base, &empty, ArrayOperation::Append),
            base,
            "Appending empty array should return base unchanged"
        );
        
        assert_eq!(
            ArrayProcessor::process(&base, &empty, ArrayOperation::Remove),
            base,
            "Removing empty array should return base unchanged"
        );

        assert_eq!(
            ArrayProcessor::process(&base, &empty, ArrayOperation::Replace),
            empty,
            "Replacing with empty array should return empty array"
        );
    }

    #[test]
    fn test_duplicate_handling() {
        // Test duplicate values in base array
        let base = vec!["a".to_string(), "b".to_string(), "a".to_string()];
        let append = vec!["c".to_string()];
        let result = ArrayProcessor::process(&base, &append, ArrayOperation::Append);
        assert_eq!(result, vec!["a", "b", "a", "c"], "Append should preserve duplicates");

        // Test duplicate values in append array
        let base = vec!["a".to_string(), "b".to_string()];
        let append = vec!["c".to_string(), "c".to_string()];
        let result = ArrayProcessor::process(&base, &append, ArrayOperation::Append);
        assert_eq!(result, vec!["a", "b", "c", "c"], "Append should preserve duplicates");

        // Test duplicate values in remove array
        let base = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let remove = vec!["b".to_string(), "b".to_string()];
        let result = ArrayProcessor::process(&base, &remove, ArrayOperation::Remove);
        assert_eq!(result, vec!["a", "c"], "Remove should handle duplicate values in remove array");

        // Test duplicate values in replace array (should preserve duplicates)
        let base = vec!["a".to_string(), "b".to_string()];
        let replace = vec!["x".to_string(), "x".to_string()];
        let result = ArrayProcessor::process(&base, &replace, ArrayOperation::Replace);
        assert_eq!(result, vec!["x", "x"], "Replace should preserve duplicates");
    }

    #[test]
    fn test_complex_operations() {
        

        // Test removing non-existent values
        let base = vec!["a".to_string(), "b".to_string()];
        let remove = vec!["c".to_string(), "d".to_string()];
        let result = ArrayProcessor::process(&base, &remove, ArrayOperation::Remove);
        assert_eq!(result, base, "Removing non-existent values should leave array unchanged");

        // Test case sensitivity
        let base = vec!["A".to_string(), "b".to_string()];
        let append = vec!["a".to_string(), "B".to_string()];
        let result = ArrayProcessor::process(&base, &append, ArrayOperation::Append);
        assert_eq!(result, vec!["A", "b", "a", "B"], "Operations should be case-sensitive");

        // Test special characters
        let base = vec!["$a".to_string(), "#b".to_string()];
        let append = vec!["@c".to_string()];
        let result = ArrayProcessor::process(&base, &append, ArrayOperation::Append);
        assert_eq!(result, vec!["$a", "#b", "@c"], "Should handle special characters");

        // Test whitespace handling
        let base = vec!["a ".to_string(), " b".to_string()];
        let append = vec!["a".to_string()];
        let result = ArrayProcessor::process(&base, &append, ArrayOperation::Append);
        assert_eq!(result, vec!["a ", " b", "a"], "Should treat whitespace as significant");
    }

    #[test]
    fn test_unicode_handling() {
        // Test non-ASCII characters
        let base = vec!["α".to_string(), "β".to_string()];
        let append = vec!["γ".to_string()];
        let result = ArrayProcessor::process(&base, &append, ArrayOperation::Append);
        assert_eq!(result, vec!["α", "β", "γ"], "Should handle Unicode characters");

        // Test emoji
        let base = vec!["👋".to_string(), "🌟".to_string()];
        let remove = vec!["👋".to_string()];
        let result = ArrayProcessor::process(&base, &remove, ArrayOperation::Remove);
        assert_eq!(result, vec!["🌟"], "Should handle emoji characters");

        // Test mixed ASCII and Unicode
        let base = vec!["a".to_string(), "β".to_string(), "🌟".to_string()];
        let replace = vec!["α".to_string(), "b".to_string(), "👋".to_string()];
        let result = ArrayProcessor::process(&base, &replace, ArrayOperation::Replace);
        assert_eq!(result, vec!["α", "b", "👋"], "Should handle mixed character types");
    }

    #[test]
    fn test_long_arrays() {
        // Test with large arrays
        let base: Vec<String> = (0..1000).map(|i| i.to_string()).collect();
        let append: Vec<String> = (500..1500).map(|i| i.to_string()).collect();
        let result = ArrayProcessor::process(&base, &append, ArrayOperation::Append);
        assert_eq!(result.len(), 2000, "Should handle large arrays correctly");

        // Test removing many items
        let remove: Vec<String> = (0..800).map(|i| i.to_string()).collect();
        let result = ArrayProcessor::process(&base, &remove, ArrayOperation::Remove);
        assert_eq!(result.len(), 200, "Should handle removing many items");
    }

    #[test]
    fn test_numeric_arrays() {
        // Test simple numeric array
        let base = vec!["1".to_string(), "2".to_string(), "3".to_string()];
        let append = vec!["4".to_string(), "5".to_string()];
        let result = ArrayProcessor::process(&base, &append, ArrayOperation::Append);
        assert_eq!(result, vec!["1", "2", "3", "4", "5"]);

        // Test array with decimals
        let base = vec!["1.0".to_string(), "1.08".to_string(), "-0.06".to_string()];
        let append = vec!["0.5".to_string()];
        let result = ArrayProcessor::process(&base, &append, ArrayOperation::Append);
        assert_eq!(result, vec!["1.0", "1.08", "-0.06", "0.5"]);
    }

    #[test]
    fn test_nested_numeric_arrays() {
        // Test colorCorrections-style nested array
        let base = vec![
            "1".to_string(), "1".to_string(), "0".to_string(),
            "{0,0,0,0}".to_string(),
            "{1,1,1,1}".to_string(),
            "{0,0,0,0}".to_string()
        ];
        let append = vec![
            "1".to_string(), "1.08".to_string(), "-0.06".to_string(),
            "{0.5,0.5,0.5,0}".to_string(),
            "{1.0,1.0,1.0,1.21}".to_string(),
            "{0.09,0.09,0.09,0.0}".to_string()
        ];
        let result = ArrayProcessor::process(&base, &append, ArrayOperation::Append);
        assert_eq!(result, vec![
            "1", "1", "0",
            "{0,0,0,0}",
            "{1,1,1,1}",
            "{0,0,0,0}",
            "1", "1.08", "-0.06",
            "{0.5,0.5,0.5,0}",
            "{1.0,1.0,1.0,1.21}",
            "{0.09,0.09,0.09,0.0}"
        ]);

        // Test filmGrain-style array
        let base = vec!["0".to_string(), "0".to_string(), "0".to_string(), "0".to_string(), "0".to_string()];
        let append = vec!["0.1".to_string(), "0.2".to_string(), "0.3".to_string(), "0.4".to_string(), "0.5".to_string()];
        let result = ArrayProcessor::process(&base, &append, ArrayOperation::Append);
        assert_eq!(result, vec!["0", "0", "0", "0", "0", "0.1", "0.2", "0.3", "0.4", "0.5"]);
    }
}