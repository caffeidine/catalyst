use crate::models::suite::TestSuite;
use std::fs;
use toml;

/// Parse tests from a TOML file
/// 
/// # Errors
/// Returns an error if the file cannot be read or parsed
pub fn parse_tests(file_path: Option<&str>) -> Result<TestSuite, &'static str> {
    let path = file_path.unwrap_or(".catalyst/tests.toml");
    let content = fs::read_to_string(path).map_err(|_| "Failed to read tests file")?;
    toml::from_str(&content).map_err(|_| "Invalid TOML format")
}

pub fn list_tests(verbose: bool, file_path: Option<&str>) {
    match parse_tests(file_path) {
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
                        println!("  Query Params: {query_params:?}");
                    }
                    if let Some(headers) = &test.headers {
                        println!("  Headers: {headers:?}");
                    }
                    if let Some(body) = &test.body {
                        println!("  Body: {body}");
                    }
                    println!("  Expected Status: {}", test.expected_status);
                }
            }
        }
        Err(err) => println!("Failed to list tests: {err}"),
    }
}
