#[cfg(test)]
mod tests {
    use catalyst::http::client::replace_variables;
    use std::collections::HashMap;

    #[test]
    fn test_variable_replacement() {
        let mut variables = HashMap::new();
        variables.insert("name".to_string(), "John".to_string());
        variables.insert("id".to_string(), "123".to_string());

        let result = replace_variables("Hello {{name}}! Your ID is {{id}}.", &variables);

        assert_eq!(result, "Hello John! Your ID is 123.");
    }
}
