#[cfg(test)]
mod tests {
    use catalyst::cli::Commands;

    #[test]
    fn test_parse_variables_single() {
        let var_string = Some("key=value".to_string());
        let result = Commands::parse_variables(var_string);

        assert_eq!(result.len(), 1);
        assert_eq!(result.get("key"), Some(&"value".to_string()));
    }

    #[test]
    fn test_parse_variables_multiple() {
        let var_string = Some("key1=value1,key2=value2,key3=value3".to_string());
        let result = Commands::parse_variables(var_string);

        assert_eq!(result.len(), 3);
        assert_eq!(result.get("key1"), Some(&"value1".to_string()));
        assert_eq!(result.get("key2"), Some(&"value2".to_string()));
        assert_eq!(result.get("key3"), Some(&"value3".to_string()));
    }

    #[test]
    fn test_parse_variables_with_spaces() {
        let var_string = Some("key1 = value1 , key2= value2,key3 =value3".to_string());
        let result = Commands::parse_variables(var_string);

        assert_eq!(result.len(), 3);
        assert_eq!(result.get("key1"), Some(&"value1".to_string()));
        assert_eq!(result.get("key2"), Some(&"value2".to_string()));
        assert_eq!(result.get("key3"), Some(&"value3".to_string()));
    }

    #[test]
    fn test_parse_variables_empty() {
        let result = Commands::parse_variables(None);
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_parse_variables_malformed() {
        let var_string = Some("key1,key2=value2,=value3,key4=".to_string());
        let result = Commands::parse_variables(var_string);

        // Only key2=value2 and key4= should be parsed
        assert_eq!(result.len(), 2);
        assert_eq!(result.get("key2"), Some(&"value2".to_string()));
        assert_eq!(result.get("key4"), Some(&"".to_string()));
    }

    #[test]
    fn test_parse_variables_with_complex_values() {
        let var_string =
            Some("api_key=abc123,base_url=https://api.example.com,user_id=42".to_string());
        let result = Commands::parse_variables(var_string);

        assert_eq!(result.len(), 3);
        assert_eq!(result.get("api_key"), Some(&"abc123".to_string()));
        assert_eq!(
            result.get("base_url"),
            Some(&"https://api.example.com".to_string())
        );
        assert_eq!(result.get("user_id"), Some(&"42".to_string()));
    }
}
