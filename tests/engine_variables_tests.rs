#[cfg(test)]
mod tests {
    use catalyst::engine::variables::{
        replace_variables_in_json, replace_variables_in_json_with_files,
    };
    use serde_json::json;
    use std::collections::HashMap;
    use std::path::Path;

    #[test]
    fn replaces_numeric_variable_in_array() {
        let mut vars = HashMap::new();
        vars.insert("admin_user_id".to_string(), "123".to_string());

        let input = json!({
            "content": "Root detection comment",
            "user_mentionned_id": ["{{admin_user_id}}"],
        });

        let result = replace_variables_in_json(&input, &vars);
        let ids = result
            .get("user_mentionned_id")
            .and_then(|v| v.as_array())
            .expect("user_mentionned_id should be an array");

        assert_eq!(ids[0].as_i64(), Some(123));
        assert_eq!(
            result
                .get("content")
                .and_then(|v| v.as_str())
                .unwrap(),
            "Root detection comment"
        );
    }

    #[test]
    fn preserves_strings_with_context() {
        let mut vars = HashMap::new();
        vars.insert("admin_user_id".to_string(), "123".to_string());

        let input = json!("Mention {{admin_user_id}} user");
        let result = replace_variables_in_json(&input, &vars);

        assert_eq!(result.as_str(), Some("Mention 123 user"));
    }

    #[test]
    fn replaces_casted_variable_inside_string_context() {
        let mut vars = HashMap::new();
        vars.insert("admin_user_id".to_string(), "123".to_string());

        let input = json!("Mention {{admin_user_id|string}} user");
        let result = replace_variables_in_json(&input, &vars);

        assert_eq!(result.as_str(), Some("Mention 123 user"));
    }

    #[test]
    fn supports_explicit_string_cast() {
        let mut vars = HashMap::new();
        vars.insert("admin_user_id".to_string(), "123".to_string());

        let input = json!("{{admin_user_id|string}}");
        let result = replace_variables_in_json(&input, &vars);

        assert_eq!(result.as_str(), Some("123"));
    }

    #[test]
    fn supports_json_cast_for_complex_values() {
        let mut vars = HashMap::new();
        vars.insert("payload".to_string(), "[1,2,3]".to_string());

        let input = json!({ "payload": "{{payload|json}}" });
        let result = replace_variables_in_json(&input, &vars);

        let payload = result
            .get("payload")
            .and_then(|v| v.as_array())
            .expect("payload should be converted to array");
        assert_eq!(payload.len(), 3);
        assert_eq!(payload[0].as_i64(), Some(1));
    }

    #[test]
    fn applies_inference_when_using_files_helper() {
        let mut vars = HashMap::new();
        vars.insert("admin_user_id".to_string(), "321".to_string());
        let input = json!(["{{admin_user_id}}"]);

        let result =
            replace_variables_in_json_with_files(&input, &vars, Path::new("."))
                .expect("replace should succeed");

        let array = result.as_array().expect("result should be an array");
        assert_eq!(array[0].as_i64(), Some(321));
    }
}
