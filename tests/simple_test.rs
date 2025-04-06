#[cfg(test)]
mod tests {
    use catalyst::http::response::{extract_cookie_value, extract_json_value};
    use catalyst::utils::string::replace_variables;
    use serde_json::json;
    use std::collections::HashMap;

    #[test]
    fn test_variable_replacement() {
        // Arrange
        let mut variables = HashMap::new();
        variables.insert("name".to_string(), "John".to_string());
        variables.insert("id".to_string(), "123".to_string());

        // Act
        let result = replace_variables("Hello {{name}}! Your ID is {{id}}.", &variables);

        // Assert
        assert_eq!(result, "Hello John! Your ID is 123.");
    }

    #[test]
    fn test_json_extraction() {
        // Arrange
        let json = json!({
            "user": {
                "id": 42,
                "name": "John Doe",
                "profile": {
                    "email": "john@example.com"
                }
            },
            "posts": [
                {"id": 1, "title": "First Post"},
                {"id": 2, "title": "Second Post"}
            ]
        });

        // Act & Assert
        assert_eq!(
            extract_json_value(&json, "$.user.id"),
            Some("42".to_string())
        );
        assert_eq!(
            extract_json_value(&json, "$.user.name"),
            Some("John Doe".to_string())
        );
        assert_eq!(
            extract_json_value(&json, "$.user.profile.email"),
            Some("john@example.com".to_string())
        );
        assert_eq!(
            extract_json_value(&json, "$.posts[0].title"),
            Some("First Post".to_string())
        );
        assert_eq!(
            extract_json_value(&json, "$.posts[1].id"),
            Some("2".to_string())
        );
    }

    #[test]
    fn test_cookie_extraction() {
        // Arrange
        let cookie_header = "session=abc123; Path=/; HttpOnly, theme=dark; Path=/";

        // Act & Assert
        assert_eq!(
            extract_cookie_value(cookie_header, "session"),
            Some("abc123".to_string())
        );
        assert_eq!(
            extract_cookie_value(cookie_header, "theme"),
            Some("dark".to_string())
        );
        assert_eq!(extract_cookie_value(cookie_header, "non_existent"), None);
    }
}
