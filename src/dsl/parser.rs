use crate::dsl::models::TestSuite;
use std::fs;
use toml;

pub fn parse_tests() -> Result<TestSuite, &'static str> {
    let content =
        fs::read_to_string(".catalyst/tests.toml").map_err(|_| "Failed to read tests file")?;
    toml::from_str(&content).map_err(|_| "Invalid TOML format")
}

pub fn list_tests(verbose: bool) {
    match parse_tests() {
        Ok(test_suite) => {
            if test_suite.tests.is_empty() {
                println!("No tests found in `tests.toml`.");
                return;
            }

            println!("Available tests:");
            for test in &test_suite.tests {
                println!("- {}", test.name);
                if verbose {
                    println!("  Method: {}", test.method);
                    println!("  Endpoint: {}", test.endpoint);
                    if let Some(query_params) = &test.query_params {
                        println!("  Query Params: {:?}", query_params);
                    }
                    if let Some(headers) = &test.headers {
                        println!("  Headers: {:?}", headers);
                    }
                    if let Some(body) = &test.body {
                        println!("  Body: {}", body);
                    }
                    println!("  Expected Status: {}", test.expected_status);
                }
            }
        }
        Err(err) => println!("Failed to list tests: {}", err),
    }
}
