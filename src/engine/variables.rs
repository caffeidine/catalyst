use crate::debug;
use crate::utils::string::replace_variables;
use dotenv::dotenv;
use serde_json::Value;
use std::collections::HashMap;

pub fn load_env_files() {
    if dotenv::from_filename(".env.local").is_ok() {
        debug!("Loaded .env.local");
    } else if dotenv::from_filename(".env.dev").is_ok() {
        debug!("Loaded .env.dev");
    } else if dotenv().is_ok() {
        debug!("Loaded .env");
    } else {
        debug!("No .env file loaded");
    }
}

pub fn replace_variables_in_json(json: &Value, vars: &HashMap<String, String>) -> Value {
    match json {
        Value::String(s) => Value::String(replace_variables(s, vars)),
        Value::Object(map) => Value::Object(
            map.iter()
                .map(|(k, v)| (k.clone(), replace_variables_in_json(v, vars)))
                .collect(),
        ),
        Value::Array(arr) => Value::Array(
            arr.iter()
                .map(|v| replace_variables_in_json(v, vars))
                .collect(),
        ),
        _ => json.clone(),
    }
}

fn get_json_value(json: &Value, path: &str) -> Option<String> {
    let parts = path.strip_prefix("$.")?;
    parts
        .split('.')
        .try_fold(json, |j, key| match j {
            Value::Object(map) => map.get(key),
            Value::Array(arr) => key.parse::<usize>().ok().and_then(|i| arr.get(i)),
            _ => None,
        })
        .map(|v| v.to_string().trim_matches('"').to_string())
}

pub fn store_variables(
    body: &Value,
    store_map: &HashMap<String, String>,
    headers: &HashMap<String, String>,
    cookie_map: &Option<HashMap<String, String>>,
    response_time_ms: u64,
    vars: &mut HashMap<String, String>,
) {
    debug!("Headers received: {:?}", headers);

    vars.insert("response_time_ms".to_string(), response_time_ms.to_string());

    for (var_name, path) in store_map {
        if let Some(value) = get_json_value(body, path) {
            vars.insert(var_name.clone(), value);
        }
    }

    for (name, value) in headers {
        vars.insert(format!("header_{}", name.to_lowercase()), value.clone());
    }

    if let Some(cookies) = cookie_map {
        debug!("Looking for cookies to extract: {:?}", cookies);

        // Case-insensitive lookup for Set-Cookie header
        let set_cookie_header = headers
            .iter()
            .find(|(k, _)| k.to_lowercase() == "set-cookie")
            .map(|(_, v)| v);

        debug!("Found Set-Cookie header: {:?}", set_cookie_header);

        if let Some(header) = set_cookie_header {
            for (name, var_name) in cookies {
                if let Some(value) = extract_cookie_value(header, name) {
                    debug!("Extracted cookie {}={}", name, value);
                    vars.insert(var_name.clone(), value);
                } else {
                    debug!("Failed to extract cookie {}", name);
                }
            }
        }
    }

    debug!("Variables after store: {:?}", vars);
}

fn extract_cookie_value(header: &str, name: &str) -> Option<String> {
    header
        .split(';')
        .find(|s| s.trim().starts_with(&format!("{}=", name)))
        .and_then(|cookie| cookie.split('=').nth(1))
        .map(|v| v.trim().to_string())
}
