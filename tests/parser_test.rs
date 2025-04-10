#[cfg(test)]
mod tests {
    use catalyst::models::test::JsonAssertion;
    use catalyst::parser::{list_tests, parse_tests};

    #[test]
    fn test_parse_tests_with_custom_file() {
        // Use the existing test file we created earlier
        let file_path = "tests/syntax/toml_syntax_validation.toml";

        // Act
        let result = parse_tests(Some(file_path));

        // Assert
        assert!(result.is_ok(), "Failed to parse test file");
        let test_suite = result.unwrap();
        assert_eq!(test_suite.config.base_url, "https://httpbin.org");
        assert_eq!(test_suite.tests.len(), 7); // We have 7 tests in the file

        // Check the first test
        let first_test = &test_suite.tests[0];
        assert_eq!(first_test.name, "Basic Test");
        assert_eq!(first_test.method, "GET");
        assert_eq!(first_test.endpoint, "/get");
        assert_eq!(first_test.expected_status, 200);

        // Check that assertions were parsed correctly
        assert!(first_test.assertions.is_some());
        assert_eq!(first_test.assertions.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn test_parse_tests_with_complex_toml_syntax() {
        // Use the existing test file with complex syntax
        let file_path = "tests/syntax/toml_syntax_validation.toml";

        // Act
        let result = parse_tests(Some(file_path));

        // Assert
        assert!(
            result.is_ok(),
            "Failed to parse test file with complex TOML syntax"
        );
        let test_suite = result.unwrap();

        // Find the test with complex assertions (Test 2 in our file)
        let complex_test = test_suite
            .tests
            .iter()
            .find(|t| t.name == "Complex Test")
            .unwrap();

        // Check body
        assert!(complex_test.body.is_some());
        let body = complex_test.body.as_ref().unwrap();
        assert!(body.to_string().contains("Test User"));

        // Check assertions
        assert!(complex_test.assertions.is_some());
        let assertions = complex_test.assertions.as_ref().unwrap();
        assert_eq!(assertions.len(), 2);

        // Check assertion types using pattern matching
        match &assertions[0] {
            JsonAssertion::Contains(_) => {} // Success if it matches Contains
            _ => panic!("First assertion should be Contains"),
        }

        match &assertions[1] {
            JsonAssertion::PathRegex(_, _) => {} // Success if it matches PathRegex
            _ => panic!("Second assertion should be PathRegex"),
        }
    }

    #[test]
    fn test_parse_tests_invalid_file() {
        // Act
        let result = parse_tests(Some("/path/to/nonexistent/file.toml"));

        // Assert
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Failed to read tests file");
    }

    // This test requires capturing stdout which is more complex
    // We'll just test the function doesn't crash
    #[test]
    fn test_list_tests_with_custom_file() {
        // Use our existing test file
        let file_path = "tests/syntax/toml_syntax_validation.toml";

        // Act & Assert - just make sure it doesn't panic
        list_tests(false, Some(file_path));
        list_tests(true, Some(file_path));
    }
}
