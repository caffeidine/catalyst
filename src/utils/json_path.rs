use serde_json::Value;

/// A utility for extracting values from JSON using simplified JSONPath expressions
pub struct JsonPath<'a> {
    path: &'a str,
}

impl<'a> JsonPath<'a> {
    /// Create a new JSONPath from a path string
    pub fn new(path: &'a str) -> Self {
        Self { path }
    }
    
    /// Extract a string value from the given JSON using this path
    /// 
    /// Supports basic JSONPath like:
    /// - "$.field" - top level field
    /// - "$.nested.field" - nested field access
    /// - "$.array[0]" - array index access  
    /// - "$.items[1].name" - combined array and field access
    ///
    /// Returns None if the path doesn't exist or contains invalid syntax
    pub fn extract_from(&self, json: &Value) -> Option<String> {
        extract_json_path(json, self.path)
    }
}

/// Extract a value from JSON using a simplified JSONPath
/// 
/// # Panics
/// May panic if the path contains invalid array indices
pub fn extract_json_path(json: &Value, path: &str) -> Option<String> {
    let parts: Vec<&str> = path.trim_start_matches("$.").split('.').collect();
    let mut current = json;

    for part in parts {
        if part.contains('[') && part.contains(']') {
            let idx_start = part.find('[')?;
            let idx_end = part.find(']')?;
            let key = &part[0..idx_start];
            let idx: usize = part[idx_start + 1..idx_end].parse().ok()?;

            if !key.is_empty() {
                current = current.get(key)?;
            }
            current = current.get(idx)?;
        } else {
            current = current.get(part)?;
        }

        if current.is_null() {
            return None;
        }
    }

    Some(match current {
        Value::String(s) => s.clone(),
        Value::Number(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
        _ => current.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_simple_field_access() {
        let json = json!({"name": "test", "value": 42});
        let path = JsonPath::new("$.name");
        assert_eq!(path.extract_from(&json), Some("test".to_string()));
    }

    #[test]
    fn test_nested_field_access() {
        let json = json!({"user": {"name": "test", "id": 123}});
        let path = JsonPath::new("$.user.name");
        assert_eq!(path.extract_from(&json), Some("test".to_string()));
    }

    #[test]
    fn test_array_access() {
        let json = json!({"items": ["first", "second", "third"]});
        let path = JsonPath::new("$.items[1]");
        assert_eq!(path.extract_from(&json), Some("second".to_string()));
    }

    #[test]
    fn test_combined_array_field_access() {
        let json = json!({"users": [{"name": "alice"}, {"name": "bob"}]});
        let path = JsonPath::new("$.users[0].name");
        assert_eq!(path.extract_from(&json), Some("alice".to_string()));
    }

    #[test]
    fn test_missing_path() {
        let json = json!({"name": "test"});
        let path = JsonPath::new("$.missing");
        assert_eq!(path.extract_from(&json), None);
    }

    #[test]
    fn test_invalid_array_index() {
        let json = json!({"items": ["first"]});
        let path = JsonPath::new("$.items[5]");
        assert_eq!(path.extract_from(&json), None);
    }

    #[test]
    fn test_number_extraction() {
        let json = json!({"count": 42});
        let path = JsonPath::new("$.count");
        assert_eq!(path.extract_from(&json), Some("42".to_string()));
    }

    #[test]
    fn test_boolean_extraction() {
        let json = json!({"active": true});
        let path = JsonPath::new("$.active");
        assert_eq!(path.extract_from(&json), Some("true".to_string()));
    }
}