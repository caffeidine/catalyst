use serde_json::Value;
use std::collections::HashMap;

pub fn replace_variables_in_json(json: &Value, variables: &HashMap<String, String>) -> Value {
    match json {
        Value::String(s) => {
            let mut result = s.clone();
            for (key, value) in variables {
                let pattern = format!("{{{{{}}}}}", key);
                result = result.replace(&pattern, value);
            }
            Value::String(result)
        }
        Value::Object(map) => {
            let mut new_map = serde_json::Map::new();
            for (k, v) in map {
                new_map.insert(k.clone(), replace_variables_in_json(v, variables));
            }
            Value::Object(new_map)
        }
        Value::Array(arr) => {
            let new_arr: Vec<Value> = arr
                .iter()
                .map(|v| replace_variables_in_json(v, variables))
                .collect();
            Value::Array(new_arr)
        }
        _ => json.clone(),
    }
}

pub fn extract_cookie_value(cookie_header: &str, cookie_name: &str) -> Option<String> {
    for cookie_group in cookie_header.split(',') {
        for cookie in cookie_group.split(';') {
            let cookie_parts: Vec<&str> = cookie.trim().split('=').collect();
            if cookie_parts.len() >= 2 && cookie_parts[0] == cookie_name {
                return Some(cookie_parts[1].to_string());
            }
        }
    }
    None
}

pub fn store_variables(
    body: &Value,
    store_map: &HashMap<String, String>,
    headers: &HashMap<String, String>,
    cookie_map: &Option<HashMap<String, String>>,
    response_time_ms: u64,
    variables: &mut HashMap<String, String>,
) {
    // Store values from JSON body
    for (json_path, variable_name) in store_map {
        if let Some(value) = super::assertions::extract_json_value(body, json_path) {
            variables.insert(variable_name.clone(), value);
        }
    }

    // Store cookies if specified
    if let Some(cookie_map) = cookie_map {
        if let Some(set_cookie) = headers.get("set-cookie") {
            for (cookie_name, variable_name) in cookie_map {
                if let Some(cookie_value) = extract_cookie_value(set_cookie, cookie_name) {
                    variables.insert(variable_name.clone(), cookie_value);
                }
            }
        }
    }

    // Store response time
    variables.insert("response_time_ms".to_string(), response_time_ms.to_string());
}
