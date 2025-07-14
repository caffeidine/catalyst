use crate::checker::parse_tests;
use crate::debug;
use crate::engine::variables::load_env_files;
use crate::http::client::HttpClient;
use crate::models::test::Test;
use colored::*;
use serde_json::Value;
use std::collections::HashMap;
use std::env;
use std::path::Path;

fn get_terminal_width() -> usize {
    env::var("COLUMNS")
        .ok()
        .and_then(|cols| cols.parse().ok())
        .unwrap_or(80)
}

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

    async fn execute_test(
        &mut self,
        test: &Test,
        client: &HttpClient,
        test_file_dir: &Path,
    ) -> TestResult {
        debug!(
            "Variables before test '{}': {:?}",
            test.name, self.variables
        );
        let result =
            crate::engine::execution::run(client, test, test_file_dir, &mut self.variables).await;
        debug!("Variables after test '{}': {:?}", test.name, self.variables);
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
        var: Option<String>,
    ) {
        load_env_files();

        // Parse CLI variables and add them to the runner's variables
        let cli_variables = crate::cli::Commands::parse_variables(var);
        self.variables.extend(cli_variables);

        // Calculate test file directory
        let test_file_path = file.as_deref().unwrap_or(".catalyst/tests.toml");
        let test_file_dir = Path::new(test_file_path).parent().unwrap_or(Path::new("."));

        let test_suite = match parse_tests(file.as_deref()) {
            Ok(suite) => suite,
            Err(e) => {
                eprintln!("{}", format!("Failed to parse tests: {e}").red());
                return;
            }
        };

        let client = HttpClient::new(&test_suite.config);
        let mut skipped = 0;
        let total = test_suite.tests.len();

        for test in test_suite.tests.iter() {
            if let Some(ref f) = filter {
                if !test.name.contains(f) {
                    skipped += 1;
                    if verbose {
                        println!("{} {}", "SKIP".yellow(), test.name);
                    }
                    continue;
                }
            }

            let result = self.execute_test(test, &client, test_file_dir).await;
            let status_matches = result.expected_status == result.actual_status;

            if verbose {
                println!("\n{}", "━".repeat(get_terminal_width()).blue());
                println!("Test: {}", test.name.bold());
                println!(
                    "Endpoint: {} {}",
                    test.method.yellow(),
                    test.endpoint.yellow()
                );
                println!(
                    "Success: {}",
                    if result.success {
                        "Yes".green()
                    } else {
                        "No".red()
                    }
                );

                let status_display = format!(
                    "Status: {} (expected {})",
                    if status_matches {
                        result.actual_status.to_string().green()
                    } else {
                        result.actual_status.to_string().red()
                    },
                    result.expected_status.to_string().bold()
                );
                println!("{status_display}");

                if let Some(body) = &result.response_body {
                    println!(
                        "\nResponse Body: {}",
                        serde_json::to_string_pretty(body).unwrap_or(body.to_string())
                    );
                }

                if let (Some(expected_body), Some(actual_body)) =
                    (&test.expected_body, &result.response_body)
                {
                    println!("\nBody comparison:");
                    let body_matches = expected_body == actual_body;
                    if body_matches {
                        println!("  {}", "✓ Body matches expected value".green());
                    } else {
                        println!("  {}", "✗ Body differs from expected value".red());
                        println!(
                            "    Expected: {}",
                            serde_json::to_string_pretty(&expected_body)
                                .unwrap()
                                .green()
                        );
                        println!(
                            "    Actual:   {}",
                            serde_json::to_string_pretty(&actual_body).unwrap().red()
                        );
                    }
                }

                if !result.messages.is_empty() {
                    println!("\nMessages:");
                    for msg in &result.messages {
                        println!("  {} {}", "-".bold(), msg.red());
                    }
                }
            } else if !self.disable_color {
                let status_indicator = if status_matches {
                    "✓".green()
                } else {
                    "✗".red()
                };
                println!(
                    "{} {} {}",
                    status_indicator,
                    test.name,
                    if !status_matches {
                        format!(
                            "(expected {}, got {})",
                            result.expected_status,
                            result.actual_status.to_string().red()
                        )
                    } else {
                        format!(
                            "(expected {}, got {})",
                            result.expected_status,
                            result.actual_status.to_string().green()
                        )
                    }
                );

                if !result.success {
                    if let (Some(expected_body), Some(actual_body)) =
                        (&test.expected_body, &result.response_body)
                    {
                        if expected_body != actual_body {
                            println!("  {}", "Body mismatch".red());
                        }
                    }
                    for msg in &result.messages {
                        println!("  {} {}", "-".bold(), msg.red());
                    }
                }
            } else {
                println!("{} {}", if result.success { "✓" } else { "✗" }, test.name);
                if !result.success {
                    println!("  Expected status: {}", result.expected_status);
                    println!("  Actual status: {}", result.actual_status);
                    for msg in &result.messages {
                        println!("  - {msg}");
                    }
                }
            }

            self.results.push(result);
        }

        if !self.disable_color {
            println!("\n{}", "━".repeat(get_terminal_width()).blue());
            let success_count = self.results.iter().filter(|r| r.success).count();
            let fail_count = self.results.len() - success_count;

            println!("\nSummary:");
            if success_count > 0 {
                print!("{} passed", format!("{success_count} tests").green());
            }
            if fail_count > 0 {
                if success_count > 0 {
                    print!(", ");
                }
                print!("{} failed", format!("{fail_count} tests").red());
            }
            if skipped > 0 {
                if success_count > 0 || fail_count > 0 {
                    print!(", ");
                }
                print!("{} skipped", format!("{skipped} tests").yellow());
            }
            println!(" (total: {total})");
        }
    }
}
