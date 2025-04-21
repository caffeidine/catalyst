#[cfg(test)]
mod tests {
    use catalyst::checker::parse_tests;
    use std::env;
    use std::fs::{self, File};
    use std::io::Write;

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
        assert_eq!(test_suite.tests.len(), 8);
        assert_eq!(test_suite.tests[0].name, "Basic Test");
        assert_eq!(test_suite.tests[0].method, "GET");
        assert_eq!(test_suite.tests[0].endpoint, "/get");
        assert_eq!(test_suite.tests[0].expected_status, 200);
    }

    #[test]
    fn test_parse_tests_invalid_toml() {
        let temp_dir = env::temp_dir();
        let test_dir = temp_dir.join("catalyst_test");
        let catalyst_dir = test_dir.join(".catalyst");

        fs::create_dir_all(&catalyst_dir).unwrap();

        let test_file = catalyst_dir.join("tests.toml");
        let mut file = File::create(&test_file).unwrap();
        writeln!(file, "invalid = toml [ content").unwrap();

        let original_dir = env::current_dir().unwrap();
        env::set_current_dir(&test_dir).unwrap();

        let result = parse_tests(None);

        env::set_current_dir(original_dir).unwrap();

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid TOML format");

        fs::remove_dir_all(test_dir).unwrap();
    }

    #[test]
    fn test_parse_tests_file_not_found() {
        let result = parse_tests(Some("non_existent_file.toml"));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Failed to read tests file");
    }
}
