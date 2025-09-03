use crate::debug;
use regex::Regex;
use serde_json::Value;

#[must_use]
pub fn body_matches(expected: &Value, actual: &Value) -> bool {
    match expected {
        Value::Object(expected_obj) => {
            if let Value::Object(actual_obj) = actual {
                for (key, expected_value) in expected_obj {
                    match actual_obj.get(key) {
                        Some(actual_value) => {
                            if !body_matches(expected_value, actual_value) {
                                return false;
                            }
                        }
                        None => return false,
                    }
                }
                true
            } else {
                false
            }
        }
        Value::Array(expected_arr) => {
            if let Value::Array(actual_arr) = actual {
                if expected_arr.len() != actual_arr.len() {
                    return false;
                }
                for (i, expected_value) in expected_arr.iter().enumerate() {
                    if !body_matches(expected_value, &actual_arr[i]) {
                        return false;
                    }
                }
                true
            } else {
                false
            }
        }
        _ => expected == actual,
    }
}

#[must_use]
pub fn validate_assertion(assertion: &crate::models::test::JsonAssertion, actual: &Value) -> bool {
    match assertion {
        crate::models::test::JsonAssertion::Exact(expected) => body_matches(expected, actual),
        crate::models::test::JsonAssertion::Contains(expected) => {
            contains_json_value(expected, actual)
        }
        crate::models::test::JsonAssertion::Regex(pattern) => {
            let json_str = actual.to_string();
            if let Ok(regex) = Regex::new(pattern) {
                regex.is_match(&json_str)
            } else {
                println!("Invalid regex pattern: {pattern}");
                false
            }
        }
        crate::models::test::JsonAssertion::PathRegex(path, pattern) => {
            if let Some(value) = extract_json_value(actual, path) {
                if let Ok(regex) = Regex::new(pattern) {
                    regex.is_match(&value)
                } else {
                    println!("Invalid regex pattern: {pattern}");
                    false
                }
            } else {
                false
            }
        }
    }
}

#[must_use]
pub fn contains_json_value(expected: &Value, actual: &Value) -> bool {
    match (expected, actual) {
        (Value::Object(expected_obj), Value::Object(actual_obj)) => {
            for (key, expected_value) in expected_obj {
                if let Some(actual_value) = actual_obj.get(key) {
                    if !contains_json_value(expected_value, actual_value) {
                        debug!("Object key '{}' value mismatch", key);
                        return false;
                    }
                } else {
                    debug!("Missing object key '{}'", key);
                    return false;
                }
            }
            true
        }
        (Value::Array(expected_arr), Value::Array(actual_arr)) => {
            // Si le tableau attendu est vide, on accepte n'importe quel tableau
            if expected_arr.is_empty() {
                return true;
            }

            for expected_value in expected_arr {
                if !actual_arr
                    .iter()
                    .any(|actual_value| contains_json_value(expected_value, actual_value))
                {
                    debug!("No matching array element found");
                    return false;
                }
            }
            true
        }
        _ => {
            let matches = expected == actual;
            if !matches {
                debug!("Value mismatch: expected {:?}, got {:?}", expected, actual);
            }
            matches
        }
    }
}

/// Extract a value from JSON using a simplified `JSONPath`
/// 
/// # Panics
/// May panic if the path contains invalid array indices
#[must_use]
pub fn extract_json_value(json: &Value, path: &str) -> Option<String> {
    crate::utils::json_path::extract_json_path(json, path)
}
