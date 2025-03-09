use crate::dsl::models::{Test, TestSuite};
use crate::dsl::parser::parse_tests;
use reqwest::Client;
use serde_json::Value;
use std::collections::HashMap;

pub struct TestRunner {
    client: Client,
    variables: HashMap<String, String>,
}

impl TestRunner {
    pub fn new() -> Self {
        TestRunner {
            client: Client::new(),
            variables: HashMap::new(),
        }
    }

    fn replace_variables(&self, input: &str) -> String {
        let mut result = input.to_string();
        for (key, value) in &self.variables {
            let placeholder = format!("{{{{{}}}}}", key);
            result = result.replace(&placeholder, value);
        }
        result
    }

    pub async fn execute_tests(&mut self, filter: Option<String>, verbose: bool) {
        let test_suite = match parse_tests() {
            Ok(tests) => tests,
            Err(err) => {
                println!("Error parsing tests.toml: {}", err);
                return;
            }
        };

        for test in &test_suite.tests {
            if let Some(ref name_filter) = filter {
                if &test.name != name_filter {
                    continue;
                }
            }

            self.execute_test(test, &test_suite, verbose).await;
        }
    }

    async fn execute_test(&mut self, test: &Test, test_suite: &TestSuite, verbose: bool) {
        let url = format!(
            "{}{}",
            test_suite.config.base_url,
            self.replace_variables(&test.endpoint)
        );
        let mut request = self.client.request(test.method.parse().unwrap(), &url);
        let mut headers = test_suite
            .config
            .default_headers
            .clone()
            .unwrap_or_default();
        if let Some(test_headers) = &test.headers {
            for (key, value) in test_headers {
                headers.insert(key.clone(), self.replace_variables(value));
            }
        }
        for (key, value) in headers {
            request = request.header(&key, &value);
        }

        if let Some(query_params) = &test.query_params {
            let mut query_string = Vec::new();
            for (key, value) in query_params {
                query_string.push((key.clone(), self.replace_variables(value)));
            }
            request = request.query(&query_string);
        }

        if let Some(body) = &test.body {
            let body_json = self.replace_variables(&body.to_string());
            request = request.body(body_json);
        }

        match request.send().await {
            Ok(response) => self.process_response(test, response, verbose).await,
            Err(_) => println!("Test `{}` failed: Request error", test.name),
        }
    }

    async fn process_response(&mut self, test: &Test, response: reqwest::Response, verbose: bool) {
        let status = response.status();
        let headers = response.headers().clone();
        let body: Value = match response.json().await {
            Ok(json) => json,
            Err(_) => Value::Null,
        };

        let mut success = true;

        if status.as_u16() != test.expected_status {
            success = false;
            println!(
                "Test `{}` failed: Expected status {}, got {}",
                test.name, test.expected_status, status
            );
        }

        if let Some(expected_headers) = &test.expected_headers {
            for (key, expected_value) in expected_headers {
                if let Some(actual_value) = headers.get(key) {
                    let actual_value_str = actual_value.to_str().unwrap_or("");
                    if actual_value_str != expected_value {
                        success = false;
                        println!(
                            "Test `{}` failed: Expected header `{}` = `{}`, got `{}`",
                            test.name, key, expected_value, actual_value_str
                        );
                    }
                } else {
                    success = false;
                    println!("Test `{}` failed: Missing header `{}`", test.name, key);
                }
            }
        }

        if let Some(expected_body) = &test.expected_body {
            if !self.compare_json(expected_body, &body) {
                success = false;
                println!(
                    "Test `{}` failed: Expected body `{}`, got `{}`",
                    test.name, expected_body, body
                );
            }
        }

        if success {
            println!("Test `{}` passed", test.name);
        }

        if let Some(store) = &test.store {
            for (json_path, var_name) in store {
                if let Some(value) = self.extract_json_value(&body, json_path) {
                    self.variables.insert(var_name.clone(), value);
                }
            }
        }

        if let Some(set_cookie) = headers.get("set-cookie") {
            if let Ok(cookie_str) = set_cookie.to_str() {
                self.variables
                    .insert("session_cookie".to_string(), cookie_str.to_string());
            }
        }

        if verbose {
            println!("Response body: {}", body);
        }
    }

    fn compare_json(&self, expected: &Value, actual: &Value) -> bool {
        match expected {
            Value::String(s) if s == "*" => true,
            _ => expected == actual,
        }
    }

    fn extract_json_value(&self, json: &Value, path: &str) -> Option<String> {
        let parts: Vec<&str> = path.split('.').collect();
        let mut current = json;
        for part in parts {
            if let Some(obj) = current.as_object() {
                current = obj.get(part)?;
            } else {
                return None;
            }
        }
        Some(current.to_string())
    }
}
