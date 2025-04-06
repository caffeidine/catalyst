#[cfg(test)]
mod tests {
    use crate::models::{Config, Test, TestSuite};
    use crate::parser::parse_tests;
    use serde_json::json;
    use std::collections::HashMap;
    use std::fs;
    use std::path::Path;
    use tempfile::tempdir;

    fn create_test_file(content: &str) -> tempfile::TempDir {
        let dir = tempdir().unwrap();
        let catalyst_dir = dir.path().join(".catalyst");
        fs::create_dir_all(&catalyst_dir).unwrap();

        let test_file_path = catalyst_dir.join("tests.toml");
        fs::write(&test_file_path, content).unwrap();

        dir
    }

    #[test]
    fn test_parse_tests_valid() {
        // Arrange
        let test_content = r#"
        [config]
        base_url = "http://localhost:8080"
        default_headers = { "Content-Type" = "application/json" }

        [[tests]]
        name = "Get Users"
        method = "GET"
        endpoint = "/api/users"
        expected_status = 200

        [[tests]]
        name = "Create User"
        method = "POST"
        endpoint = "/api/users"
        body = { "name" = "John Doe", "email" = "john@example.com" }
        expected_status = 201
        "#;

        let dir = create_test_file(test_content);

        // Override the current directory for testing
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(dir.path()).unwrap();

        // Act
        let result = parse_tests();

        // Restore the original directory
        std::env::set_current_dir(original_dir).unwrap();

        // Assert
        assert!(result.is_ok());
        let test_suite = result.unwrap();
        assert_eq!(test_suite.config.base_url, "http://localhost:8080");
        assert_eq!(test_suite.tests.len(), 2);
        assert_eq!(test_suite.tests[0].name, "Get Users");
        assert_eq!(test_suite.tests[0].method, "GET");
        assert_eq!(test_suite.tests[0].endpoint, "/api/users");
        assert_eq!(test_suite.tests[0].expected_status, 200);
        assert_eq!(test_suite.tests[1].name, "Create User");
        assert_eq!(test_suite.tests[1].method, "POST");
        assert_eq!(test_suite.tests[1].endpoint, "/api/users");
        assert_eq!(test_suite.tests[1].expected_status, 201);
    }

    #[test]
    fn test_parse_tests_invalid_toml() {
        // Arrange
        let test_content = r#"
        [config]
        base_url = "http://localhost:8080"
        default_headers = { "Content-Type" = "application/json" }

        [[tests]]
        name = "Get Users"
        method = "GET"
        endpoint = "/api/users"
        expected_status = 200
        
        This is invalid TOML
        "#;

        let dir = create_test_file(test_content);

        // Override the current directory for testing
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(dir.path()).unwrap();

        // Act
        let result = parse_tests();

        // Restore the original directory
        std::env::set_current_dir(original_dir).unwrap();

        // Assert
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid TOML format");
    }

    #[test]
    fn test_parse_tests_file_not_found() {
        // Arrange
        let dir = tempdir().unwrap();

        // Override the current directory for testing
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(dir.path()).unwrap();

        // Act
        let result = parse_tests();

        // Restore the original directory
        std::env::set_current_dir(original_dir).unwrap();

        // Assert
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Failed to read tests file");
    }

    #[test]
    fn test_parse_tests_complex_config() {
        // Arrange
        let test_content = r#"
        [config]
        base_url = "http://localhost:8080"
        auth_method = "bearer"
        auth_token = "abc123"
        default_headers = { "Content-Type" = "application/json", "User-Agent" = "Catalyst" }

        [[tests]]
        name = "Login"
        method = "POST"
        endpoint = "/api/login"
        body = { "username" = "test", "password" = "password" }
        expected_status = 200
        store = { "$.token" = "auth_token", "$.user_id" = "user_id" }
        get_cookie = { "session" = "session_cookie" }

        [[tests]]
        name = "Get Profile"
        method = "GET"
        endpoint = "/api/users/{{user_id}}/profile"
        headers = { "Authorization" = "Bearer {{auth_token}}" }
        expected_status = 200
        query_params = { "include" = "details" }
        expected_headers = [["Content-Type", "application/json"]]
        "#;

        let dir = create_test_file(test_content);

        // Override the current directory for testing
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(dir.path()).unwrap();

        // Act
        let result = parse_tests();

        // Restore the original directory
        std::env::set_current_dir(original_dir).unwrap();

        // Assert
        assert!(result.is_ok());
        let test_suite = result.unwrap();
        assert_eq!(test_suite.config.base_url, "http://localhost:8080");
        assert_eq!(test_suite.config.auth_method, Some("bearer".to_string()));
        assert_eq!(test_suite.config.auth_token, Some("abc123".to_string()));

        let login_test = &test_suite.tests[0];
        assert_eq!(login_test.name, "Login");
        assert!(login_test.store.is_some());
        assert!(login_test.get_cookie.is_some());

        let profile_test = &test_suite.tests[1];
        assert_eq!(profile_test.name, "Get Profile");
        assert_eq!(profile_test.endpoint, "/api/users/{{user_id}}/profile");
        assert!(profile_test.headers.is_some());
        assert!(profile_test.query_params.is_some());
        assert!(profile_test.expected_headers.is_some());
    }
}
