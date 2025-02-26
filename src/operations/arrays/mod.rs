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
        let mut result = base.to_vec();
        result.extend_from_slice(to_append);
        result.dedup();
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