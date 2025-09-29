use crate::debug;
use crate::utils::string::{replace_variables, replace_variables_with_files};
use dotenv::dotenv;
use serde_json::Value;
use std::borrow::Cow;
use std::collections::HashMap;
use std::path::Path;

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
        Value::String(s) => resolve_string_value(s, replace_variables(s, vars), vars),
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

pub fn replace_variables_in_json_with_files(
    json: &Value,
    vars: &HashMap<String, String>,
    test_file_dir: &Path,
) -> Result<Value, String> {
    match json {
        Value::String(s) => {
            let result = replace_variables_with_files(s, vars, test_file_dir)?;
            Ok(resolve_string_value(s, result, vars))
        }
        Value::Object(map) => {
            let mut new_map = serde_json::Map::new();
            for (k, v) in map {
                let new_value = replace_variables_in_json_with_files(v, vars, test_file_dir)?;
                new_map.insert(k.clone(), new_value);
            }
            Ok(Value::Object(new_map))
        }
        Value::Array(arr) => {
            let mut new_arr = Vec::new();
            for v in arr {
                let new_value = replace_variables_in_json_with_files(v, vars, test_file_dir)?;
                new_arr.push(new_value);
            }
            Ok(Value::Array(new_arr))
        }
        _ => Ok(json.clone()),
    }
}

enum PlaceholderSource {
    Variable,
    Env,
}

struct PlaceholderSpec {
    name: String,
    cast: Option<String>,
    source: PlaceholderSource,
}

fn resolve_string_value(
    original: &str,
    replaced: String,
    vars: &HashMap<String, String>,
) -> Value {
    if let Some(placeholder) = parse_placeholder(original) {
        let PlaceholderSpec { name, cast, source } = placeholder;

        let raw_value: Option<Cow<'_, str>> = match source {
            PlaceholderSource::Variable => {
                vars.get(&name).map(|value| Cow::Borrowed(value.as_str()))
            }
            PlaceholderSource::Env => std::env::var(&name).ok().map(Cow::Owned),
        };

        if let Some(raw) = raw_value {
            if let Some(value) = value_from_raw(raw.as_ref(), cast.as_deref()) {
                return value;
            }
        }
    }

    Value::String(replaced)
}

fn parse_placeholder(input: &str) -> Option<PlaceholderSpec> {
    let trimmed = input.trim();

    if let Some(spec) = parse_placeholder_with_prefix(trimmed, "{{", PlaceholderSource::Variable) {
        return Some(spec);
    }

    if let Some(spec) = parse_placeholder_with_prefix(trimmed, "${{", PlaceholderSource::Env) {
        return Some(spec);
    }

    None
}

fn parse_placeholder_with_prefix(
    trimmed: &str,
    prefix: &str,
    source: PlaceholderSource,
) -> Option<PlaceholderSpec> {
    if !trimmed.starts_with(prefix) || !trimmed.ends_with("}}") {
        return None;
    }

    let start = prefix.len();

    if trimmed.len() <= start + 2 {
        return None;
    }

    let inner = &trimmed[start..trimmed.len() - 2];
    let inner = inner.trim();

    if inner.is_empty() || inner.contains("{{") || inner.contains("}}") {
        return None;
    }

    let mut parts = inner.splitn(2, '|');
    let name = parts.next()?.trim();

    if name.is_empty() {
        return None;
    }

    let cast = parts
        .next()
        .map(|c| c.trim().to_string())
        .filter(|c| !c.is_empty());

    Some(PlaceholderSpec {
        name: name.to_string(),
        cast,
        source,
    })
}

fn value_from_raw(raw: &str, cast: Option<&str>) -> Option<Value> {
    match cast.map(|c| c.to_ascii_lowercase()) {
        Some(cast) => match cast.as_str() {
            "int" | "integer" => raw.trim().parse::<i64>().ok().map(|n| Value::Number(n.into())),
            "float" | "double" | "number" => raw
                .trim()
                .parse::<f64>()
                .ok()
                .and_then(|n| serde_json::Number::from_f64(n))
                .map(Value::Number),
            "bool" | "boolean" => match raw.trim().to_ascii_lowercase().as_str() {
                "true" => Some(Value::Bool(true)),
                "false" => Some(Value::Bool(false)),
                _ => None,
            },
            "string" | "str" => Some(Value::String(raw.to_string())),
            "json" => serde_json::from_str::<Value>(raw).ok(),
            "array" | "list" => serde_json::from_str::<Value>(raw)
                .ok()
                .and_then(|value| value.as_array().cloned().map(Value::Array)),
            "object" | "map" => serde_json::from_str::<Value>(raw)
                .ok()
                .and_then(|value| value.as_object().cloned().map(Value::Object)),
            "null" => Some(Value::Null),
            _ => None,
        },
        None => {
            let candidate = raw.trim();
            if candidate.is_empty() {
                return None;
            }

            serde_json::from_str::<Value>(candidate).ok()
        }
    }
}

pub fn get_json_value(json: &Value, path: &str) -> Option<String> {
    let parts = path.strip_prefix("$.")?;
    let mut current = json;

    debug!("Extracting value from path: {}", path);
    debug!("Initial JSON: {:?}", json);

    for part in parts.split('.') {
        debug!("Processing path part: {}", part);

        if part.contains('[') && part.contains(']') {
            // Handle array access with [n] notation
            let key = part.split('[').next()?;
            let idx_str = part
                .split('[')
                .nth(1)?
                .trim_end_matches(']')
                .parse::<usize>()
                .ok()?;

            if !key.is_empty() {
                debug!("Accessing object key: {}", key);
                current = current.get(key)?;
                debug!("After key access: {:?}", current);
            }
            debug!("Accessing array index: {}", idx_str);
            current = current.get(idx_str)?;
        } else {
            // Try as array index first
            if let Ok(idx) = part.parse::<usize>() {
                debug!("Using direct array index: {}", idx);
                current = current.get(idx)?;
            } else {
                debug!("Accessing object key: {}", part);
                current = current.get(part)?;
            }
        }

        debug!("Current value after access: {:?}", current);
    }

    let result = match current {
        Value::String(s) => s.clone(),
        Value::Number(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
        _ => current.to_string().trim_matches('"').to_string(),
    };

    debug!("Extracted final value: {}", result);
    Some(result)
}

pub fn store_variables(
    body: &Value,
    store_map: &HashMap<String, String>,
    headers: &HashMap<String, String>,
    cookie_map: &Option<HashMap<String, String>>,
    response_time_ms: u64,
    vars: &mut HashMap<String, String>,
) {
    debug!("Starting variable storage");
    debug!("Current variables: {:?}", vars);
    debug!("Headers received: {:?}", headers);
    debug!("Body received: {:?}", body);
    debug!("Store map: {:?}", store_map);

    vars.insert("response_time_ms".to_string(), response_time_ms.to_string());

    for (var_name, path) in store_map {
        debug!("Attempting to extract '{}' from path '{}'", var_name, path);
        if let Some(value) = get_json_value(body, path) {
            let clean_value = value.trim_matches('"').to_string();
            debug!("Successfully stored {} = {}", var_name, clean_value);
            vars.insert(var_name.clone(), clean_value);
        } else {
            debug!(
                "⚠️ Failed to extract value at path '{}' for variable '{}'",
                path, var_name
            );
            debug!(
                "Current JSON body: {}",
                serde_json::to_string_pretty(body).unwrap_or_default()
            );
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
        .find(|s| s.trim().starts_with(&format!("{name}=")))
        .and_then(|cookie| cookie.split('=').nth(1))
        .map(|v| v.trim().to_string())
}
