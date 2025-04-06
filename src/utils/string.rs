use std::collections::HashMap;

/// Replace variables in a string
///
/// Variables are in the format {{variable_name}}
pub fn replace_variables(input: &str, variables: &HashMap<String, String>) -> String {
    let mut result = input.to_string();
    for (key, value) in variables {
        let pattern = format!("{{{{{}}}}}", key);
        result = result.replace(&pattern, value);
    }
    result
}
