#[cfg(test)]
mod tests {
    use catalyst::utils::string::replace_variables;
    use std::collections::HashMap;

    #[test]
    fn test_replace_variables_single() {
        let mut variables = HashMap::new();
        variables.insert("name".to_string(), "John".to_string());
        let result = replace_variables("Hello {{name}}!", &variables);
        assert_eq!(result, "Hello John!");
    }

    #[test]
    fn test_replace_variables_multiple() {
        let mut variables = HashMap::new();
        variables.insert("name".to_string(), "John".to_string());
        variables.insert("age".to_string(), "30".to_string());
        let result = replace_variables("{{name}} is {{age}} years old.", &variables);
        assert_eq!(result, "John is 30 years old.");
    }

    #[test]
    fn test_replace_variables_no_match() {
        let mut variables = HashMap::new();
        variables.insert("name".to_string(), "John".to_string());
        let result = replace_variables("Hello world!", &variables);
        assert_eq!(result, "Hello world!");
    }

    #[test]
    fn test_replace_variables_partial_match() {
        let mut variables = HashMap::new();
        variables.insert("name".to_string(), "John".to_string());
        let result = replace_variables("Hello {{name}}, welcome to {{city}}!", &variables);
        assert_eq!(result, "Hello John, welcome to {{city}}!");
    }

    #[test]
    fn test_replace_variables_json() {
        let mut variables = HashMap::new();
        variables.insert("token".to_string(), "abc123".to_string());
        variables.insert("user_id".to_string(), "42".to_string());
        let result = replace_variables(
            r#"{"authorization": "Bearer {{token}}", "user": {"id": {{user_id}}}}"#,
            &variables,
        );
        assert_eq!(
            result,
            r#"{"authorization": "Bearer abc123", "user": {"id": 42}}"#
        );
    }
}
