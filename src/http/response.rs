use crate::models::Test;
use reqwest::Response;
use serde_json::Value;
use std::collections::HashMap;

pub async fn process_response(
    response: Response,
    test: &Test,
    variables: &mut HashMap<String, String>,
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
                println!("error response body json = {}", err);
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
                println!("error response body text = {}", err);
                Value::Null
            }
        }
    };

    let success = status == expected_status;

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

    (success, expected_status, status, Some(body), headers_map)
}

// Function to extract a JSON value from a path
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

// Function to extract a cookie value from a Set-Cookie header
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
