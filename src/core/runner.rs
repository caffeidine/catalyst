use crate::checker::parse_tests;
use crate::engine::execution::{ExecutionResult, execute_test_case};
use crate::http::client::HttpClient;
use crate::models::suite::TestSuite;
use crate::models::test::Test;
use colored::*;
use serde_json::Value;
use std::collections::HashMap;
use std::time::Instant;

pub type StoredTestResult = (
    String,
    bool,
    u16,
    u16,
    Option<Value>,
    HashMap<String, String>,
);

pub struct TestRunner {
    pub variables: HashMap<String, String>,
    pub results: Vec<StoredTestResult>,
    disable_color: bool,
}

impl TestRunner {
    pub fn new(disable_color: bool) -> Self {
        Self {
            variables: HashMap::new(),
            results: Vec::new(),
            disable_color,
        }
    }

    fn colorize(&self, text: &str, color: Color) -> String {
        if self.disable_color {
            text.to_string()
        } else {
            text.color(color).to_string()
        }
    }

    pub async fn execute_tests(
        &mut self,
        filter: Option<String>,
        verbose: bool,
        file_path: Option<String>,
    ) -> Vec<ExecutionResult> {
        let test_suite = match parse_tests(file_path.as_deref()) {
            Ok(suite) => suite,
            Err(err) => {
                println!("Failed to parse tests: {}", err);
                return Vec::new();
            }
        };

        let client = HttpClient::new();
        let mut results = Vec::new();

        for test in &test_suite.tests {
            if let Some(filter_str) = &filter {
                if !test.name.contains(filter_str) {
                    continue;
                }
            }

            if verbose {
                println!("Executing test: {}", test.name);
            }

            let result = self.execute_test(&client, test, &test_suite).await;
            results.push(result);
        }

        self.print_results(verbose);
        self.print_summary();

        results
    }

    async fn execute_test(
        &mut self,
        client: &HttpClient,
        test: &Test,
        test_suite: &TestSuite,
    ) -> ExecutionResult {
        // Start timing the request
        let start_time = Instant::now();

        match client
            .execute_request(test, test_suite, &self.variables)
            .await
        {
            Ok(response) => {
                let result =
                    execute_test_case(response, test, &mut self.variables, start_time).await;
                self.results.push((
                    test.name.clone(),
                    result.success,
                    result.expected_status,
                    result.actual_status,
                    result.response_body.clone(),
                    result.headers.clone(),
                ));
                result
            }
            Err(err) => {
                println!("Error executing test {}: {}", test.name, err);
                self.results.push((
                    test.name.clone(),
                    false,
                    test.expected_status,
                    0,
                    None,
                    HashMap::new(),
                ));
                ExecutionResult {
                    success: false,
                    expected_status: test.expected_status,
                    actual_status: 0,
                    response_time_ms: 0,
                    response_body: None,
                    headers: HashMap::new(),
                }
            }
        }
    }

    fn print_results(&self, verbose: bool) {
        let mut failed_tests = Vec::new();

        for (test_name, success, expected_status, actual_status, body, headers) in &self.results {
            let status_display =
                format!("({} {})", actual_status, self.status_text(*actual_status));

            if *success {
                println!(
                    "[{}] {:<25} {}",
                    self.colorize("PASS", Color::Green),
                    test_name,
                    self.colorize(&status_display, Color::Green)
                );
            } else {
                println!(
                    "[{}] {:<25} {} (expected {})",
                    self.colorize("FAIL", Color::Red),
                    test_name,
                    self.colorize(&status_display, Color::Red),
                    expected_status
                );
                failed_tests.push(test_name);
            }

            if verbose {
                if let Some(body_value) = body {
                    println!("  Body: {}", body_value);
                } else {
                    println!("  Body: null");
                }

                println!("  Headers:");
                for (key, value) in headers {
                    println!("    {}: {}", key, value);
                }
                println!();
            }
        }

        if !failed_tests.is_empty() {
            println!("\nFailed tests:");
            for test_name in failed_tests {
                println!("- {}", test_name);
            }
        }
    }

    fn print_summary(&self) {
        let total = self.results.len();
        let passed = self
            .results
            .iter()
            .filter(|(_, success, _, _, _, _)| *success)
            .count();
        let failed = total - passed;

        println!("\nTest Summary:");
        println!(
            "Total: {}, Passed: {}, Failed: {}",
            total,
            self.colorize(&passed.to_string(), Color::Green),
            if failed > 0 {
                self.colorize(&failed.to_string(), Color::Red)
            } else {
                self.colorize(&failed.to_string(), Color::Green)
            }
        );
    }

    fn status_text(&self, status: u16) -> &'static str {
        match status {
            100..=199 => "Informational",
            200..=299 => "Success",
            300..=399 => "Redirection",
            400..=499 => "Client Error",
            500..=599 => "Server Error",
            _ => "Unknown",
        }
    }
}
