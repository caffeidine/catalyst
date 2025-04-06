#[cfg(test)]
mod tests {
    use catalyst::http::response::{body_matches, contains_json_value, validate_assertion};
    use catalyst::models::test::JsonAssertion;
    use serde_json::json;

    #[test]
    fn test_body_matches_exact_match() {
        let expected = json!({
            "success": true,
            "data": {
                "id": 1,
                "name": "Test"
            }
        });

        let actual = json!({
            "success": true,
            "data": {
                "id": 1,
                "name": "Test"
            }
        });

        assert!(body_matches(&expected, &actual));
    }

    #[test]
    fn test_body_matches_different_values() {
        let expected = json!({
            "success": true,
            "data": {
                "id": 1,
                "name": "Test"
            }
        });

        let actual = json!({
            "success": true,
            "data": {
                "id": 2,
                "name": "Test"
            }
        });

        assert!(!body_matches(&expected, &actual));
    }

    #[test]
    fn test_body_matches_missing_field() {
        let expected = json!({
            "success": true,
            "data": {
                "id": 1,
                "name": "Test"
            }
        });

        let actual = json!({
            "success": true,
            "data": {
                "id": 1
            }
        });

        assert!(!body_matches(&expected, &actual));
    }

    #[test]
    fn test_contains_json_value_subset() {
        let expected = json!({
            "success": true,
            "data": {
                "id": 1
            }
        });

        let actual = json!({
            "success": true,
            "data": {
                "id": 1,
                "name": "Test",
                "extra": "field"
            },
            "meta": {
                "page": 1
            }
        });

        assert!(contains_json_value(&expected, &actual));
    }

    #[test]
    fn test_contains_json_value_different_values() {
        let expected = json!({
            "success": true,
            "data": {
                "id": 1
            }
        });

        let actual = json!({
            "success": true,
            "data": {
                "id": 2,
                "name": "Test"
            }
        });

        assert!(!contains_json_value(&expected, &actual));
    }

    #[test]
    fn test_contains_json_value_array() {
        let expected = json!([1, 2]);
        let actual = json!([1, 2, 3, 4]);

        assert!(contains_json_value(&expected, &actual));
    }

    #[test]
    fn test_contains_json_value_array_different_order() {
        let expected = json!([1, 2]);
        let actual = json!([3, 2, 1]);

        assert!(contains_json_value(&expected, &actual));
    }

    #[test]
    fn test_validate_assertion_exact() {
        let assertion = JsonAssertion::Exact(json!({
            "success": true,
            "data": {
                "id": 1
            }
        }));

        let actual = json!({
            "success": true,
            "data": {
                "id": 1
            }
        });

        assert!(validate_assertion(&assertion, &actual));
    }

    #[test]
    fn test_validate_assertion_contains() {
        let assertion = JsonAssertion::Contains(json!({
            "success": true
        }));

        let actual = json!({
            "success": true,
            "data": {
                "id": 1
            }
        });

        assert!(validate_assertion(&assertion, &actual));
    }

    #[test]
    fn test_validate_assertion_regex() {
        // Utiliser une regex simple qui correspond à un élément spécifique
        let assertion = JsonAssertion::Regex(r#".*"success":true.*"#.to_string());
        let actual = json!({
            "success": true,
            "data": {
                "id": 1
            }
        });

        assert!(validate_assertion(&assertion, &actual));
    }

    #[test]
    fn test_validate_assertion_path_regex() {
        let assertion = JsonAssertion::PathRegex("$.data.id".to_string(), r#"\d+"#.to_string());
        let actual = json!({
            "success": true,
            "data": {
                "id": 123
            }
        });

        assert!(validate_assertion(&assertion, &actual));
    }

    #[test]
    fn test_validate_assertion_path_regex_no_match() {
        let assertion = JsonAssertion::PathRegex("$.data.id".to_string(), r#"[a-z]+"#.to_string());
        let actual = json!({
            "success": true,
            "data": {
                "id": 123
            }
        });

        assert!(!validate_assertion(&assertion, &actual));
    }

    #[test]
    fn test_validate_assertion_path_regex_missing_path() {
        let assertion =
            JsonAssertion::PathRegex("$.data.missing".to_string(), r#"\d+"#.to_string());
        let actual = json!({
            "success": true,
            "data": {
                "id": 123
            }
        });

        assert!(!validate_assertion(&assertion, &actual));
    }
}
