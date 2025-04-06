use crate::models::Test;
use crate::models::test::JsonAssertion;
use regex::Regex;
use reqwest::Response;
use serde_json::Value;
use std::collections::HashMap;
use std::time::Instant;

pub async fn process_response(
    response: Response,
    test: &Test,
    variables: &mut HashMap<String, String>,
    start_time: Instant,
) -> (bool, u16, u16, Option<Value>, HashMap<String, String>) {
    let status = response.status().as_u16();
    let expected_status = test.expected_status;
    let headers = response.headers().clone();
    let mut headers_map = HashMap::new();
    for (key, value) in headers.iter() {
        headers_map.insert(key.to_string(), value.to_str().unwrap_or("").to_string());
    }

    // Check the Content-Type to determine how to process the body
    let content_type = headers
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    let body: Value = if content_type.contains("application/json") {
        match response.json().await {
            Ok(json) => json,
            Err(err) => {
                println!("Error parsing response body as JSON: {}", err);
                Value::Null
            }
        }
    } else {
        match response.text().await {
            Ok(text) => match serde_json::from_str(&text) {
                Ok(json_value) => json_value,
                Err(_) => Value::String(text),
            },
            Err(err) => {
                println!("Error reading response body as text: {}", err);
                Value::Null
            }
        }
    };

    // Calculate response time
    let response_time = start_time.elapsed();
    let response_time_ms = response_time.as_millis() as u64;

    // Check status code
    let mut success = status == expected_status;

    // Check response time if specified
    if let Some(max_time) = test.max_response_time {
        if response_time_ms > max_time {
            println!(
                "Test '{}' failed: Response time {} ms exceeds maximum allowed {} ms",
                test.name, response_time_ms, max_time
            );
            success = false;
        }
    }

    // Validate body if expected_body is specified (backward compatibility)
    if success && test.expected_body.is_some() {
        let expected = test.expected_body.as_ref().unwrap();
        if !body_matches(expected, &body) {
            println!(
                "Test '{}' failed: Response body does not match expected body",
                test.name
            );
            success = false;
        }
    }

    // Validate advanced assertions if specified
    if success && test.assertions.is_some() {
        for assertion in test.assertions.as_ref().unwrap() {
            if !validate_assertion(assertion, &body) {
                println!(
                    "Test '{}' failed: Assertion failed: {:?}",
                    test.name, assertion
                );
                success = false;
                break;
            }
        }
    }

    // Extract and store variables from the JSON body
    if let Some(store) = &test.store {
        for (json_path, variable_name) in store {
            if let Some(value) = extract_json_value(&body, json_path) {
                variables.insert(variable_name.clone(), value);
            }
        }
    }

    // Extract and store cookies
    if let Some(get_cookie) = &test.get_cookie {
        for (cookie_name, variable_name) in get_cookie {
            if let Some(set_cookie) = headers.get("set-cookie") {
                if let Ok(cookie_str) = set_cookie.to_str() {
                    if let Some(cookie_value) = extract_cookie_value(cookie_str, cookie_name) {
                        variables.insert(variable_name.clone(), cookie_value);
                    }
                }
            }
        }
    }

    // Store response time in variables for potential use in subsequent tests
    variables.insert("response_time_ms".to_string(), response_time_ms.to_string());

    (success, expected_status, status, Some(body), headers_map)
}

/// Function to check if a body matches an expected value (exact match for backward compatibility)
pub fn body_matches(expected: &Value, actual: &Value) -> bool {
    match expected {
        Value::Object(expected_obj) => {
            if let Value::Object(actual_obj) = actual {
                // Check that all keys in expected exist in actual with the same values
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

/// Function to validate a JSON assertion against the actual response body
pub fn validate_assertion(assertion: &JsonAssertion, actual: &Value) -> bool {
    match assertion {
        JsonAssertion::Exact(expected) => body_matches(expected, actual),
        JsonAssertion::Contains(expected) => {
            match expected {
                Value::Object(expected_obj) => {
                    if let Value::Object(actual_obj) = actual {
                        // Check that all keys in expected exist in actual
                        for (key, expected_value) in expected_obj {
                            match actual_obj.get(key) {
                                Some(actual_value) => {
                                    if !contains_json_value(expected_value, actual_value) {
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
                _ => body_matches(expected, actual), // For non-objects, fall back to exact match
            }
        }
        JsonAssertion::Regex(pattern) => {
            // Convert JSON to string and check against regex
            let json_str = actual.to_string();
            match Regex::new(pattern) {
                Ok(regex) => regex.is_match(&json_str),
                Err(_) => {
                    println!("Invalid regex pattern: {}", pattern);
                    false
                }
            }
        }
        JsonAssertion::PathRegex(path, pattern) => {
            // Extract value at path and check against regex
            if let Some(value) = extract_json_value(actual, path) {
                match Regex::new(pattern) {
                    Ok(regex) => regex.is_match(&value),
                    Err(_) => {
                        println!("Invalid regex pattern: {}", pattern);
                        false
                    }
                }
            } else {
                false
            }
        }
    }
}

/// Helper function for Contains assertion to check if expected value is contained in actual value
pub fn contains_json_value(expected: &Value, actual: &Value) -> bool {
    match (expected, actual) {
        (Value::Object(expected_obj), Value::Object(actual_obj)) => {
            // Check that all keys in expected exist in actual with matching values
            for (key, expected_value) in expected_obj {
                match actual_obj.get(key) {
                    Some(actual_value) => {
                        if !contains_json_value(expected_value, actual_value) {
                            return false;
                        }
                    }
                    None => return false,
                }
            }
            true
        }
        (Value::Array(expected_arr), Value::Array(actual_arr)) => {
            // For arrays, we check that each item in expected has a matching item in actual
            for expected_value in expected_arr {
                if !actual_arr
                    .iter()
                    .any(|actual_value| contains_json_value(expected_value, actual_value))
                {
                    return false;
                }
            }
            true
        }
        // For primitive values, we do an exact match
        _ => expected == actual,
    }
}

/// Extracts a value from a JSON object using a dot-notation path
///
/// Example: "$.data.user.id" will extract the value at json["data"]["user"]["id"]
/// Supports array indexing with brackets, e.g., "$.data.users[0].name"
pub fn extract_json_value(json: &Value, path: &str) -> Option<String> {
    let parts: Vec<&str> = path.trim_start_matches("$.").split('.').collect();
    let mut current = json;

    for part in parts {
        // Check if the part contains an array index
        if part.contains('[') && part.contains(']') {
            let idx_start = part.find('[').unwrap();
            let idx_end = part.find(']').unwrap();
            let key = &part[0..idx_start];
            let idx: usize = part[idx_start + 1..idx_end].parse().ok()?;

            // First navigate to the object using the key
            if !key.is_empty() {
                current = &current[key];
            }

            // Then access the array element
            current = &current[idx];
        } else {
            current = &current[part];
        }

        if current.is_null() {
            return None;
        }
    }

    match current {
        Value::String(s) => Some(s.clone()),
        Value::Number(n) => Some(n.to_string()),
        Value::Bool(b) => Some(b.to_string()),
        _ => Some(current.to_string()),
    }
}

/// Extracts a cookie value from a Set-Cookie header
///
/// Handles multiple cookies separated by commas and attributes separated by semicolons
pub fn extract_cookie_value(cookie_header: &str, cookie_name: &str) -> Option<String> {
    // First split by comma to handle multiple cookies
    for cookie_group in cookie_header.split(',') {
        // Then split each cookie group by semicolon
        for cookie in cookie_group.split(';') {
            let cookie_parts: Vec<&str> = cookie.trim().split('=').collect();
            if cookie_parts.len() >= 2 && cookie_parts[0] == cookie_name {
                return Some(cookie_parts[1].to_string());
            }
        }
    }
    None
}
