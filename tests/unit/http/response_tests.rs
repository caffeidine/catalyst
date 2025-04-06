#[cfg(test)]
mod tests {
    use crate::http::response::{extract_cookie_value, extract_json_value};
    use serde_json::json;
    use std::collections::HashMap;

    #[test]
    fn test_extract_json_value_simple() {
        // Arrange
        let json = json!({
            "name": "John Doe",
            "age": 30,
            "is_active": true
        });

        // Act & Assert
        assert_eq!(
            extract_json_value(&json, "$.name"),
            Some("John Doe".to_string())
        );
        assert_eq!(extract_json_value(&json, "$.age"), Some("30".to_string()));
        assert_eq!(
            extract_json_value(&json, "$.is_active"),
            Some("true".to_string())
        );
    }

    #[test]
    fn test_extract_json_value_nested() {
        // Arrange
        let json = json!({
            "user": {
                "id": 123,
                "profile": {
                    "first_name": "John",
                    "last_name": "Doe"
                }
            }
        });

        // Act & Assert
        assert_eq!(
            extract_json_value(&json, "$.user.id"),
            Some("123".to_string())
        );
        assert_eq!(
            extract_json_value(&json, "$.user.profile.first_name"),
            Some("John".to_string())
        );
        assert_eq!(
            extract_json_value(&json, "$.user.profile.last_name"),
            Some("Doe".to_string())
        );
    }

    #[test]
    fn test_extract_json_value_array() {
        // Arrange
        let json = json!({
            "items": [
                {"id": 1, "name": "Item 1"},
                {"id": 2, "name": "Item 2"}
            ]
        });

        // Act & Assert
        assert_eq!(
            extract_json_value(&json, "$.items[0].id"),
            Some("1".to_string())
        );
        assert_eq!(
            extract_json_value(&json, "$.items[0].name"),
            Some("Item 1".to_string())
        );
        assert_eq!(
            extract_json_value(&json, "$.items[1].id"),
            Some("2".to_string())
        );
        assert_eq!(
            extract_json_value(&json, "$.items[1].name"),
            Some("Item 2".to_string())
        );
    }

    #[test]
    fn test_extract_json_value_not_found() {
        // Arrange
        let json = json!({
            "user": {
                "id": 123
            }
        });

        // Act & Assert
        assert_eq!(extract_json_value(&json, "$.user.name"), None);
        assert_eq!(extract_json_value(&json, "$.profile"), None);
        assert_eq!(extract_json_value(&json, "$.user.profile.first_name"), None);
    }

    #[test]
    fn test_extract_cookie_value() {
        // Arrange
        let cookie_header =
            "session=abc123; Path=/; HttpOnly; SameSite=Lax, theme=dark; Path=/; Max-Age=3600";

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

    #[test]
    fn test_extract_cookie_value_complex() {
        // Arrange
        let cookie_header = "id=a3fWa; Expires=Wed, 21 Oct 2025 07:28:00 GMT; Secure; HttpOnly";

        // Act & Assert
        assert_eq!(
            extract_cookie_value(cookie_header, "id"),
            Some("a3fWa".to_string())
        );
        assert_eq!(extract_cookie_value(cookie_header, "Expires"), None);
    }

    #[test]
    fn test_extract_cookie_value_multiple_cookies() {
        // Arrange
        let cookie_header =
            "id=a3fWa; Expires=Wed, 21 Oct 2025 07:28:00 GMT, session=xyz789; Path=/";

        // Act & Assert
        assert_eq!(
            extract_cookie_value(cookie_header, "id"),
            Some("a3fWa".to_string())
        );
        assert_eq!(
            extract_cookie_value(cookie_header, "session"),
            Some("xyz789".to_string())
        );
    }
}
