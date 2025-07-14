#[cfg(test)]
mod tests {
    use catalyst::engine::variables::replace_variables_in_json_with_files;
    use catalyst::utils::string::{escape_json_string, replace_variables_with_files};
    use serde_json::{Value, json};
    use std::collections::HashMap;
    use std::env;
    use std::fs;
    use std::path::PathBuf;

    fn setup_test_files() -> PathBuf {
        let test_name = std::thread::current()
            .name()
            .map(|s| s.to_string())
            .unwrap_or_else(|| "unknown".to_string());
        let temp_dir = env::temp_dir().join(format!("catalyst_test_{test_name}"));
        if temp_dir.exists() {
            let _ = fs::remove_dir_all(&temp_dir);
        }
        fs::create_dir_all(&temp_dir).unwrap();

        // Create test text file
        let text_file = temp_dir.join("test.txt");
        fs::write(&text_file, "Hello from file!\nLine 2 with \"quotes\"").unwrap();

        // Create test JSON file
        let json_file = temp_dir.join("data.json");
        fs::write(&json_file, r#"{"nested": "value"}"#).unwrap();

        // Create subdirectory with file
        let sub_dir = temp_dir.join("sub");
        fs::create_dir(&sub_dir).unwrap();
        let sub_file = sub_dir.join("nested.txt");
        fs::write(&sub_file, "Nested content").unwrap();

        temp_dir
    }

    fn cleanup_test_files(temp_dir: &PathBuf) {
        if temp_dir.exists() {
            let _ = fs::remove_dir_all(temp_dir);
        }
    }

    #[test]
    fn test_escape_json_string() {
        let input = "Hello \"world\"\nNew line\tTab\\Backslash";
        let expected = "Hello \\\"world\\\"\\nNew line\\tTab\\\\Backslash";
        assert_eq!(escape_json_string(input), expected);
    }

    #[test]
    fn test_replace_variables_with_files_simple() {
        let temp_dir = setup_test_files();
        let vars = HashMap::new();

        let input = "Content: {{file:test.txt}}";
        let result = replace_variables_with_files(input, &vars, &temp_dir).unwrap();

        assert_eq!(
            result,
            "Content: Hello from file!\\nLine 2 with \\\"quotes\\\""
        );

        cleanup_test_files(&temp_dir);
    }

    #[test]
    fn test_replace_variables_with_files_multiple() {
        let temp_dir = setup_test_files();
        let vars = HashMap::new();

        let input = "Text: {{file:test.txt}} and JSON: {{file:data.json}}";
        let result = replace_variables_with_files(input, &vars, &temp_dir).unwrap();

        assert!(result.contains("Hello from file!"));
        assert!(result.contains("{\\\"nested\\\": \\\"value\\\"}"));
    }

    #[test]
    fn test_replace_variables_with_files_subdirectory() {
        let temp_dir = setup_test_files();
        let vars = HashMap::new();

        let input = "{{file:sub/nested.txt}}";
        let result = replace_variables_with_files(input, &vars, &temp_dir).unwrap();

        assert_eq!(result, "Nested content");
    }

    #[test]
    fn test_replace_variables_with_files_combined_with_vars() {
        let temp_dir = setup_test_files();
        let mut vars = HashMap::new();
        vars.insert("username".to_string(), "john".to_string());

        let input = "User: {{username}}, Content: {{file:test.txt}}";
        let result = replace_variables_with_files(input, &vars, &temp_dir).unwrap();

        assert!(result.starts_with("User: john, Content: Hello from file!"));
    }

    #[test]
    fn test_replace_variables_with_files_nonexistent() {
        let temp_dir = setup_test_files();
        let vars = HashMap::new();

        let input = "{{file:nonexistent.txt}}";
        let result = replace_variables_with_files(input, &vars, &temp_dir).unwrap();

        // Should keep original text when file doesn't exist
        assert_eq!(result, "{{file:nonexistent.txt}}");
    }

    #[test]
    fn test_replace_variables_with_files_path_traversal() {
        let temp_dir = setup_test_files();
        let vars = HashMap::new();

        let input = "{{file:../../../etc/passwd}}";
        let result = replace_variables_with_files(input, &vars, &temp_dir).unwrap();

        // Should keep original text for security
        assert_eq!(result, "{{file:../../../etc/passwd}}");
    }

    #[test]
    fn test_replace_variables_in_json_with_files_string() {
        let temp_dir = setup_test_files();
        let vars = HashMap::new();

        let json = json!("{{file:test.txt}}");
        let result = replace_variables_in_json_with_files(&json, &vars, &temp_dir).unwrap();

        assert_eq!(
            result,
            Value::String("Hello from file!\\nLine 2 with \\\"quotes\\\"".to_string())
        );
    }

    #[test]
    fn test_replace_variables_in_json_with_files_object() {
        let temp_dir = setup_test_files();
        let vars = HashMap::new();

        let json = json!({
            "content": "{{file:test.txt}}",
            "data": "{{file:data.json}}",
            "static": "value"
        });

        let result = replace_variables_in_json_with_files(&json, &vars, &temp_dir).unwrap();

        if let Value::Object(obj) = result {
            assert!(
                obj["content"]
                    .as_str()
                    .unwrap()
                    .contains("Hello from file!")
            );
            assert!(obj["data"].as_str().unwrap().contains("nested"));
            assert_eq!(obj["static"], "value");
        } else {
            panic!("Expected object");
        }
    }

    #[test]
    fn test_replace_variables_in_json_with_files_array() {
        let temp_dir = setup_test_files();
        let vars = HashMap::new();

        let json = json!(["{{file:test.txt}}", "static", "{{file:sub/nested.txt}}"]);

        let result = replace_variables_in_json_with_files(&json, &vars, &temp_dir).unwrap();

        if let Value::Array(arr) = result {
            assert!(arr[0].as_str().unwrap().contains("Hello from file!"));
            assert_eq!(arr[1], "static");
            assert_eq!(arr[2], "Nested content");
        } else {
            panic!("Expected array");
        }
    }

    #[test]
    fn test_replace_variables_in_json_with_files_nested() {
        let temp_dir = setup_test_files();
        let mut vars = HashMap::new();
        vars.insert("key".to_string(), "value".to_string());

        let json = json!({
            "level1": {
                "level2": {
                    "file_content": "{{file:test.txt}}",
                    "variable": "{{key}}"
                }
            },
            "array": [
                {
                    "item": "{{file:sub/nested.txt}}"
                }
            ]
        });

        let result = replace_variables_in_json_with_files(&json, &vars, &temp_dir).unwrap();

        // Verify nested structure is preserved and content is replaced
        assert!(
            result["level1"]["level2"]["file_content"]
                .as_str()
                .unwrap()
                .contains("Hello from file!")
        );
        assert_eq!(result["level1"]["level2"]["variable"], "value");
        assert_eq!(result["array"][0]["item"], "Nested content");
    }

    #[test]
    fn test_no_file_references() {
        let temp_dir = setup_test_files();
        let mut vars = HashMap::new();
        vars.insert("key".to_string(), "value".to_string());

        let json = json!({
            "normal": "text",
            "variable": "{{key}}",
            "number": 42
        });

        let result = replace_variables_in_json_with_files(&json, &vars, &temp_dir).unwrap();

        assert_eq!(result["normal"], "text");
        assert_eq!(result["variable"], "value");
        assert_eq!(result["number"], 42);
    }

    #[test]
    fn test_empty_file() {
        let test_name = std::thread::current()
            .name()
            .map(|s| s.to_string())
            .unwrap_or_else(|| "unknown".to_string());
        let temp_dir = env::temp_dir().join(format!("catalyst_empty_{test_name}"));
        if temp_dir.exists() {
            let _ = fs::remove_dir_all(&temp_dir);
        }
        fs::create_dir_all(&temp_dir).unwrap();

        let empty_file = temp_dir.join("empty.txt");
        fs::write(&empty_file, "").unwrap();

        let vars = HashMap::new();
        let json = json!({"content": "{{file:empty.txt}}"});

        let result = replace_variables_in_json_with_files(&json, &vars, &temp_dir).unwrap();

        assert_eq!(result["content"], "");

        cleanup_test_files(&temp_dir);
    }

    #[test]
    fn test_file_with_special_characters() {
        let test_name = std::thread::current()
            .name()
            .map(|s| s.to_string())
            .unwrap_or_else(|| "unknown".to_string());
        let temp_dir = env::temp_dir().join(format!("catalyst_special_{test_name}"));
        if temp_dir.exists() {
            let _ = fs::remove_dir_all(&temp_dir);
        }
        fs::create_dir_all(&temp_dir).unwrap();

        let special_file = temp_dir.join("special.txt");
        fs::write(&special_file, "Special: \t\n\r\"'\\{}[]").unwrap();

        let vars = HashMap::new();
        let json = json!({"content": "{{file:special.txt}}"});

        let result = replace_variables_in_json_with_files(&json, &vars, &temp_dir).unwrap();

        let content = result["content"].as_str().unwrap();
        assert!(content.contains("\\t"));
        assert!(content.contains("\\n"));
        assert!(content.contains("\\r"));
        assert!(content.contains("\\\""));
        assert!(content.contains("\\\\"));

        cleanup_test_files(&temp_dir);
    }
}
