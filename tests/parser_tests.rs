#[cfg(test)]
mod tests {
    use catalyst::parser::parse_tests;

    #[test]
    fn test_parse_tests_valid() {
        let syntax_dir = std::path::Path::new("tests/syntax");
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(syntax_dir).unwrap();

        let result = parse_tests(Some("toml_syntax_validation.toml"));

        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_ok());
        let test_suite = result.unwrap();
        assert_eq!(test_suite.config.base_url, "https://httpbin.org");
        assert_eq!(test_suite.tests.len(), 7);
        assert_eq!(test_suite.tests[0].name, "Basic Test");
        assert_eq!(test_suite.tests[0].method, "GET");
        assert_eq!(test_suite.tests[0].endpoint, "/get");
        assert_eq!(test_suite.tests[0].expected_status, 200);
        assert_eq!(test_suite.tests[1].name, "Complex Test");
        assert_eq!(test_suite.tests[1].method, "POST");
        assert_eq!(test_suite.tests[1].endpoint, "/post");
        assert_eq!(test_suite.tests[1].expected_status, 200);
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

        let file_path = std::path::Path::new("tests.toml");
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(original_dir).unwrap();
        let mut file = std::fs::File::create(file_path).unwrap();
        use std::io::Write;
        file.write_all(test_content.as_bytes()).unwrap();
        file.flush().unwrap();

        let result = parse_tests(Some(file_path.to_str().unwrap()));

        std::fs::remove_file(file_path).unwrap();

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid TOML format");
    }

    #[test]
    fn test_parse_tests_file_not_found() {
        let result = parse_tests(None);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Failed to read tests file");
    }
}
