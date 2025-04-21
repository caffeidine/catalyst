#[cfg(test)]
mod tests {
    use catalyst::utils::string::replace_variables;
    use std::{collections::HashMap, env};

    fn with_env_var<F>(key: &str, value: &str, test: F)
    where
        F: FnOnce() + std::panic::UnwindSafe,
    {
        unsafe {
            env::set_var(key, value);
            let _ = std::panic::catch_unwind(test);
            env::remove_var(key);
        }
    }

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
    fn test_replace_env_var() {
        with_env_var("TEST_USER", "Alice", || {
            let variables = HashMap::new();
            let result = replace_variables("Hello ${{TEST_USER}}!", &variables);
            assert_eq!(result, "Hello Alice!");
        });
    }

    #[test]
    fn test_replace_env_and_test_vars() {
        with_env_var("API_KEY", "secret123", || {
            let mut variables = HashMap::new();
            variables.insert("username".to_string(), "john_doe".to_string());

            let result =
                replace_variables("Auth: Bearer ${{API_KEY}}, User: {{username}}", &variables);
            assert_eq!(result, "Auth: Bearer secret123, User: john_doe");
        });
    }

    #[test]
    fn test_replace_variables_json() {
        with_env_var("TOKEN", "xyz789", || {
            let mut variables = HashMap::new();
            variables.insert("user_id".to_string(), "42".to_string());

            let result = replace_variables(
                r#"{
                    "authorization": "Bearer ${{TOKEN}}",
                    "user": {
                        "id": {{user_id}}
                    }
                }"#,
                &variables,
            );

            assert_eq!(
                result.replace([' ', '\n'], ""),
                r#"{"authorization":"Bearer xyz789","user":{"id":42}}"#
            );
        });
    }
}
