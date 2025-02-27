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
        // Create a result vector with deduplicated base elements
        let mut seen = HashSet::new();
        let mut result = Vec::new();
        
        // First add base elements without duplicates
        for item in base {
            if !seen.contains(item) {
                seen.insert(item);
                result.push(item.clone());
            }
        }
        
        // Then add items from to_append that don't exist in the result yet
        for item in to_append {
            if !seen.contains(item) {
                seen.insert(item);
                result.push(item.clone());
            }
        }
        
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