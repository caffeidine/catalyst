use crate::error::{CatalystError, CatalystResult};
use crate::utils::string::replace_variables;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub fn load_body_from_file(
    file_path: &str,
    test_file_dir: &Path,
    vars: &HashMap<String, String>,
) -> CatalystResult<Value> {
    // Security check - ensure path doesn't contain path traversal
    if file_path.contains("..") {
        return Err(CatalystError::file_error("File path cannot escape test directory"));
    }

    // Resolve path relative to test file directory
    let full_path = test_file_dir.join(file_path);

    if !full_path.exists() {
        return Err(CatalystError::file_error(format!("File '{file_path}' does not exist")));
    }

    if !full_path.is_file() {
        return Err(CatalystError::file_error(format!("'{file_path}' is not a file")));
    }

    let content = fs::read_to_string(&full_path)
        .map_err(|e| CatalystError::file_error(format!("Cannot read file '{file_path}': {e}")))?;

    let processed_content = replace_variables(&content, vars);

    if file_path.ends_with(".json") {
        serde_json::from_str(&processed_content)
            .map_err(|e| CatalystError::json_error(format!("Invalid JSON in file '{file_path}': {e}")))
    } else {
        Ok(Value::String(processed_content))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_load_json_file() {
        let test_dir = Path::new(".catalyst");
        if !test_dir.exists() {
            fs::create_dir_all(test_dir).unwrap();
        }

        let test_file = test_dir.join("test.json");
        fs::write(&test_file, r#"{"name": "test", "value": 42}"#).unwrap();

        let vars = HashMap::new();
        let result = load_body_from_file("test.json", test_dir, &vars).unwrap();

        assert_eq!(result["name"], "test");
        assert_eq!(result["value"], 42);

        fs::remove_file(test_file).unwrap();
    }

    #[test]
    fn test_load_text_file() {
        let test_dir = Path::new(".catalyst");
        if !test_dir.exists() {
            fs::create_dir_all(test_dir).unwrap();
        }

        let test_file = test_dir.join("test.txt");
        fs::write(&test_file, "Hello, World!").unwrap();

        let vars = HashMap::new();
        let result = load_body_from_file("test.txt", test_dir, &vars).unwrap();

        assert_eq!(result, Value::String("Hello, World!".to_string()));

        fs::remove_file(test_file).unwrap();
    }

    #[test]
    fn test_variable_substitution() {
        let test_dir = Path::new(".catalyst");
        if !test_dir.exists() {
            fs::create_dir_all(test_dir).unwrap();
        }

        let test_file = test_dir.join("test_vars.json");
        fs::write(&test_file, r#"{"user": "{{username}}", "id": {{user_id}}}"#).unwrap();

        let mut vars = HashMap::new();
        vars.insert("username".to_string(), "john_doe".to_string());
        vars.insert("user_id".to_string(), "123".to_string());

        let result = load_body_from_file("test_vars.json", test_dir, &vars).unwrap();

        assert_eq!(result["user"], "john_doe");
        assert_eq!(result["id"], 123);

        fs::remove_file(test_file).unwrap();
    }

    #[test]
    fn test_security_path_traversal() {
        let test_dir = Path::new(".catalyst");
        let vars = HashMap::new();
        let result = load_body_from_file("../secret.json", test_dir, &vars);

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("escape test directory"));
    }

    #[test]
    fn test_file_not_found() {
        let test_dir = Path::new(".catalyst");
        let vars = HashMap::new();
        let result = load_body_from_file("nonexistent.json", test_dir, &vars);

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("does not exist"));
    }
}
