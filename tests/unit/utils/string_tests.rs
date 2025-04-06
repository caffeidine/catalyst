#[cfg(test)]
mod tests {
    use crate::utils::string::replace_variables;
    use std::collections::HashMap;

    #[test]
    fn test_replace_variables_single() {
        // Arrange
        let mut variables = HashMap::new();
        variables.insert("name".to_string(), "John".to_string());

        // Act
        let result = replace_variables("Hello {{name}}!", &variables);

        // Assert
        assert_eq!(result, "Hello John!");
    }

    #[test]
    fn test_replace_variables_multiple() {
        // Arrange
        let mut variables = HashMap::new();
        variables.insert("name".to_string(), "John".to_string());
        variables.insert("age".to_string(), "30".to_string());

        // Act
        let result = replace_variables("{{name}} is {{age}} years old.", &variables);

        // Assert
        assert_eq!(result, "John is 30 years old.");
    }

    #[test]
    fn test_replace_variables_no_match() {
        // Arrange
        let mut variables = HashMap::new();
        variables.insert("name".to_string(), "John".to_string());

        // Act
        let result = replace_variables("Hello world!", &variables);

        // Assert
        assert_eq!(result, "Hello world!");
    }

    #[test]
    fn test_replace_variables_partial_match() {
        // Arrange
        let mut variables = HashMap::new();
        variables.insert("name".to_string(), "John".to_string());

        // Act
        let result = replace_variables("Hello {{name}}, welcome to {{city}}!", &variables);

        // Assert
        assert_eq!(result, "Hello John, welcome to {{city}}!");
    }

    #[test]
    fn test_replace_variables_json() {
        // Arrange
        let mut variables = HashMap::new();
        variables.insert("token".to_string(), "abc123".to_string());
        variables.insert("user_id".to_string(), "42".to_string());

        // Act
        let result = replace_variables(
            r#"{"authorization": "Bearer {{token}}", "user": {"id": {{user_id}}}}"#,
            &variables,
        );

        // Assert
        assert_eq!(
            result,
            r#"{"authorization": "Bearer abc123", "user": {"id": 42}}"#
        );
    }
}
