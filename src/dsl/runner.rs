use crate::dsl::models::{Test, TestSuite};
use crate::dsl::parser::parse_tests;
use colored::*;
use reqwest::Client;
use serde_json::Value;
use std::collections::HashMap;

pub struct TestRunner {
    client: Client,
    variables: HashMap<String, String>,
    disable_color: bool,
    results: Vec<(
        String,
        bool,
        u16,
        u16,
        Option<Value>,
        HashMap<String, String>,
    )>,
}

impl TestRunner {
    pub fn new(disable_color: bool) -> Self {
        TestRunner {
            client: Client::new(),
            variables: HashMap::new(),
            disable_color,
            results: Vec::new(),
        }
    }

    fn colorize(&self, text: &str, color: Color) -> String {
        if self.disable_color {
            text.to_string()
        } else {
            text.color(color).bold().to_string()
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
                println!(
                    "{}",
                    self.colorize(&format!("Error parsing tests.toml: {}", err), Color::Red)
                );
                return;
            }
        };

        println!("{}", "────────────────────────────────────────".dimmed());

        for test in &test_suite.tests {
            if let Some(ref name_filter) = filter {
                if &test.name != name_filter {
                    continue;
                }
            }

            let success = self.execute_test(test, &test_suite).await;
            self.results.push((
                test.name.clone(),
                success.0,
                success.1,
                success.2,
                success.3,
                success.4,
            ));
        }

        self.print_results(verbose);
    }

    async fn execute_test(
        &mut self,
        test: &Test,
        test_suite: &TestSuite,
    ) -> (bool, u16, u16, Option<Value>, HashMap<String, String>) {
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
                headers.insert(key.clone(), value.clone());
            }
        }

        for (_key, value) in headers.iter_mut() {
            *value = self.replace_variables(value);
        }

        for (key, value) in &headers {
            request = request.header(key, value);
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
            Ok(response) => self.process_response(test, response).await,
            Err(_) => {
                println!(
                    "{}",
                    self.colorize(
                        &format!("Test `{}` failed: Request error", test.name),
                        Color::Red
                    )
                );
                (false, 0, 0, None, HashMap::new())
            }
        }
    }

    async fn process_response(
        &mut self,
        test: &Test,
        response: reqwest::Response,
    ) -> (bool, u16, u16, Option<Value>, HashMap<String, String>) {
        let status = response.status().as_u16();
        let expected_status = test.expected_status;
        let headers = response.headers().clone();
        let mut headers_map = HashMap::new();
        for (key, value) in headers.iter() {
            headers_map.insert(key.to_string(), value.to_str().unwrap_or("").to_string());
        }

        let body: Value = match response.json().await {
            Ok(json) => json,
            Err(_) => Value::Null,
        };

        let success = status == expected_status;

        if let Some(store) = &test.store {
            for (json_path, variable_name) in store {
                if let Some(value) = self.extract_json_value(&body, json_path) {
                    self.variables.insert(variable_name.clone(), value);
                }
            }
        }

        if let Some(get_cookie) = &test.get_cookie {
            for (cookie_name, variable_name) in get_cookie {
                if let Some(set_cookie) = headers.get("set-cookie") {
                    if let Ok(cookie_str) = set_cookie.to_str() {
                        if let Some(cookie_value) =
                            self.extract_cookie_value(cookie_str, cookie_name)
                        {
                            self.variables.insert(variable_name.clone(), cookie_value);
                        }
                    }
                }
            }
        }

        (success, expected_status, status, Some(body), headers_map)
    }

    fn print_results(&self, verbose: bool) {
        let mut failed_tests = Vec::new();

        for (test_name, success, expected_status, actual_status, body, headers) in &self.results {
            let status_display =
                format!("({} {})", actual_status, self.status_text(*actual_status));

            if *success {
                println!(
                    "[PASS] {:<25} {}",
                    test_name,
                    self.colorize(&status_display, Color::Green)
                );
            } else {
                println!(
                    "[FAIL] {:<25} {}",
                    test_name,
                    self.colorize(&status_display, Color::Red)
                );
                println!(
                    "       Expected: {} {}",
                    expected_status,
                    self.status_text(*expected_status)
                );
                println!(
                    "       Got:      {} {}",
                    actual_status,
                    self.status_text(*actual_status)
                );
                failed_tests.push(test_name.clone());

                if verbose {
                    println!("       Headers: {:?}", headers);
                    if let Some(b) = body {
                        println!("       Body: {}", b);
                    }
                }
            }
        }

        println!("{}", "────────────────────────────────────────".dimmed());
        self.print_summary();

        if !failed_tests.is_empty() {
            println!("\nFailed Tests:");
            for test in failed_tests {
                println!("  - {}", test);
            }
        }
    }

    fn print_summary(&self) {
        let passed = self
            .results
            .iter()
            .filter(|(_, status, _, _, _, _)| *status)
            .count();
        let failed = self.results.len() - passed;

        let summary = format!("Summary: {} passed, {} failed", passed, failed);
        if failed > 0 {
            println!("{}", self.colorize(&summary, Color::Red));
        } else {
            println!("{}", self.colorize(&summary, Color::Green));
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

        match current {
            Value::String(s) => Some(s.clone()),
            _ => Some(current.to_string().trim_matches('"').to_string()),
        }
    }

    fn status_text(&self, status: u16) -> &'static str {
        match status {
            200 => "OK",
            201 => "Created",
            204 => "No Content",
            400 => "Bad Request",
            401 => "Unauthorized",
            403 => "Forbidden",
            404 => "Not Found",
            405 => "Method Not Allowed",
            409 => "Conflict",
            500 => "Internal Server Error",
            _ => "Unknown",
        }
    }

    fn extract_cookie_value(&self, cookie_header: &str, cookie_name: &str) -> Option<String> {
        for cookie in cookie_header.split(',') {
            let parts: Vec<&str> = cookie.trim().split(';').collect();
            if let Some(main_cookie) = parts.first() {
                let key_value: Vec<&str> = main_cookie.split('=').collect();
                if key_value.len() == 2 && key_value[0].trim() == cookie_name {
                    return Some(main_cookie.trim().to_string());
                }
            }
        }
        None
    }
}
