use crate::checker::parse_tests;
use crate::engine::variables::load_env_files;
use crate::http::client::HttpClient;
use crate::models::test::Test;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug)]
pub struct TestResult {
    pub name: String,
    pub success: bool,
    pub expected_status: u16,
    pub actual_status: u16,
    pub response_body: Option<Value>,
    pub headers: HashMap<String, String>,
    pub messages: Vec<String>,
}

pub struct TestRunner {
    pub variables: HashMap<String, String>,
    pub results: Vec<TestResult>,
    pub disable_color: bool,
}

impl TestRunner {
    pub fn new(disable_color: bool) -> Self {
        TestRunner {
            variables: HashMap::new(),
            results: Vec::new(),
            disable_color,
        }
    }

    async fn execute_test(&mut self, test: &Test, client: &HttpClient) -> TestResult {
        let result = crate::engine::execution::run(client, test, &self.variables).await;

        TestResult {
            name: test.name.clone(),
            success: result.success,
            expected_status: result.status.0,
            actual_status: result.status.1,
            response_body: result.body,
            headers: result.headers,
            messages: result.errors,
        }
    }

    pub async fn execute_tests(
        &mut self,
        filter: Option<String>,
        verbose: bool,
        file: Option<String>,
    ) {
        load_env_files();

        let test_suite = match parse_tests(file.as_deref()) {
            Ok(suite) => suite,
            Err(e) => {
                eprintln!("Failed to parse tests: {}", e);
                return;
            }
        };

        let client = HttpClient::new(&test_suite.config);

        for test in test_suite.tests.iter() {
            if let Some(ref f) = filter {
                if !test.name.contains(f) {
                    continue;
                }
            }

            let result = self.execute_test(test, &client).await;

            if verbose {
                println!("Test: {}", test.name);
                println!("Success: {}", result.success);
                println!(
                    "Status: {} (expected {})",
                    result.actual_status, result.expected_status
                );
                if !result.messages.is_empty() {
                    println!("Messages:");
                    for msg in &result.messages {
                        println!("  - {}", msg);
                    }
                }
                println!();
            } else if !self.disable_color {
                if result.success {
                    println!("\x1b[32m✓\x1b[0m {}", test.name);
                } else {
                    println!("\x1b[31m✗\x1b[0m {}", test.name);
                }
            } else {
                println!("{} {}", if result.success { "✓" } else { "✗" }, test.name);
            }

            self.results.push(result);
        }
    }
}
