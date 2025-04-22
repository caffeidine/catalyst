#[cfg(test)]
mod tests {
    use catalyst::engine::variables::get_json_value;
    use serde_json::json;

    #[test]
    fn test_array_index_access() {
        let json = json!([
            {"id": 1, "name": "first"},
            {"id": 2, "name": "second"},
            {"id": 3, "name": "third"}
        ]);

        // Test direct array index access
        let value = get_json_value(&json, "$.1.id");
        assert_eq!(
            value,
            Some("2".to_string()),
            "Failed to access array element by index"
        );
    }

    #[test]
    fn test_nested_array_access() {
        let json = json!({
            "users": [
                {"id": 1, "name": "first"},
                {"id": 2, "name": "second"},
                {"id": 3, "name": "third"}
            ]
        });

        // Test array access within object
        let value = get_json_value(&json, "$.users[1].id");
        assert_eq!(
            value,
            Some("2".to_string()),
            "Failed to access nested array element"
        );
    }

    #[test]
    fn test_array_different_value_types() {
        let json = json!([
            {"id": 1, "active": true},
            {"id": "abc", "active": false},
            {"id": 3, "active": null}
        ]);

        // Test different value types
        assert_eq!(get_json_value(&json, "$.[0].id"), Some("1".to_string()));
        assert_eq!(get_json_value(&json, "$.[1].id"), Some("abc".to_string()));
        assert_eq!(get_json_value(&json, "$.[2].id"), Some("3".to_string()));
    }

    #[test]
    fn test_invalid_array_access() {
        let json = json!([
            {"id": 1},
            {"id": 2}
        ]);

        // Test out of bounds access
        assert_eq!(get_json_value(&json, "$.[5].id"), None);

        // Test invalid syntax
        assert_eq!(get_json_value(&json, "$.[abc].id"), None);

        // Test missing field
        assert_eq!(get_json_value(&json, "$.[0].missing"), None);
    }
}
