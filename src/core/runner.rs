use crate::checker::parse_tests;
use crate::debug;
use crate::engine::variables::load_env_files;
use crate::http::client::HttpClient;
use crate::models::test::Test;
use crate::output::TestSummaryFormatter;
use colored::Colorize;
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
    pub method: String,
    pub endpoint: String,
}

struct TestExecutionContext<'a> {
    test_suite: &'a crate::models::suite::TestSuite,
    filter: Option<String>,
    verbose: bool,
    test_file_dir: &'a Path,
    client: &'a HttpClient,
    total: usize,
}

pub struct TestRunner {
    pub variables: HashMap<String, String>,
    pub results: Vec<TestResult>,
    pub disable_color: bool,
    pub no_fail_summary: bool,
}

impl TestRunner {
    #[must_use]
    pub fn new(disable_color: bool) -> Self {
        TestRunner {
            variables: HashMap::new(),
            results: Vec::new(),
            disable_color,
            no_fail_summary: false,
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
            method: test.method.clone(),
            endpoint: test.endpoint.clone(),
        }
    }

    async fn execute_test_with_hooks(
        &mut self,
        test: &Test,
        client: &HttpClient,
        test_file_dir: &Path,
        allowed_commands: Option<&[String]>,
    ) -> TestResult {
        let mut test_failed = false;
        let mut error_messages = Vec::new();

        // Execute before hooks
        if let Some(before_steps) = &test.before
            && let Err(e) = crate::engine::commands::execute_command_steps(
                before_steps,
                &mut self.variables,
                test_file_dir,
                allowed_commands,
                "before",
            ).await {
            error_messages.push(format!("Before hook failed: {e}"));
            test_failed = true;
        }

        // Execute the actual HTTP test (only if before hooks succeeded)
        let mut result = if test_failed {
            TestResult {
                name: test.name.clone(),
                success: false,
                expected_status: test.expected_status,
                actual_status: 0,
                response_body: None,
                headers: HashMap::new(),
                messages: error_messages.clone(),
                method: test.method.clone(),
                endpoint: test.endpoint.clone(),
            }
        } else {
            self.execute_test(test, client, test_file_dir).await
        };

        let http_test_success = result.success && !test_failed;

        // Execute after hooks (always run, but respect 'on' condition)
        if let Some(after_steps) = &test.after {
            let filtered_steps: Vec<_> = after_steps.iter().filter(|step| {
                match step.get_on_condition() {
                    "success" => http_test_success,
                    "failure" => !http_test_success,
                    _ => true,
                }
            }).cloned().collect();

            if !filtered_steps.is_empty()
                && let Err(e) = crate::engine::commands::execute_command_steps(
                    &filtered_steps,
                    &mut self.variables,
                    test_file_dir,
                    allowed_commands,
                    "after",
                ).await {
                result.messages.push(format!("After hook failed: {e}"));
                // Note: after hook failures don't change the test result success status
            }
        }

        result.messages.extend(error_messages);
        result
    }

    pub async fn execute_tests(
        &mut self,
        filter: Option<String>,
        verbose: bool,
        file: Option<String>,
        var: Option<String>,
        no_fail_summary: bool,
    ) {
        load_env_files();
        self.no_fail_summary = no_fail_summary;

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

        // Execute suite setup hooks
        if let Some(setup_steps) = &test_suite.setup
            && let Err(e) = crate::engine::commands::execute_command_steps(
                setup_steps,
                &mut self.variables,
                test_file_dir,
                test_suite.config.allowed_commands.as_deref(),
                "setup",
            ).await {
            eprintln!("{}", format!("Setup failed: {e}").red());
            return;
        }

        // Execute tests with defer for teardown
        let context = TestExecutionContext {
            test_suite: &test_suite,
            filter,
            verbose,
            test_file_dir,
            client: &client,
            total,
        };
        self.execute_tests_with_hooks(context, &mut skipped).await;

        // Execute suite teardown hooks (always runs)
        if let Some(teardown_steps) = &test_suite.teardown
            && let Err(e) = crate::engine::commands::execute_command_steps(
                teardown_steps,
                &mut self.variables,
                test_file_dir,
                test_suite.config.allowed_commands.as_deref(),
                "teardown",
            ).await {
            eprintln!("{}", format!("Teardown failed: {e}").red());
        }
    }

    async fn execute_tests_with_hooks(
        &mut self,
        context: TestExecutionContext<'_>,
        skipped: &mut usize,
    ) {
        for test in &context.test_suite.tests {
            if let Some(ref f) = context.filter
                && !test.name.contains(f) {
                *skipped += 1;
                if context.verbose {
                    println!("{} {}", "SKIP".yellow(), test.name);
                }
                continue;
            }

            let result = self.execute_test_with_hooks(test, context.client, context.test_file_dir, context.test_suite.config.allowed_commands.as_deref()).await;
            let status_matches = result.expected_status == result.actual_status;

            if context.verbose {
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
                    if status_matches {
                        format!(
                            "(expected {}, got {})",
                            result.expected_status,
                            result.actual_status.to_string().green()
                        )
                    } else {
                        format!(
                            "(expected {}, got {})",
                            result.expected_status,
                            result.actual_status.to_string().red()
                        )
                    }
                );

                if !result.success {
                    if let (Some(expected_body), Some(actual_body)) =
                        (&test.expected_body, &result.response_body)
                        && expected_body != actual_body {
                        println!("  {}", "Body mismatch".red());
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

        self.display_compact_results(*skipped, context.total, context.verbose);
        if !self.no_fail_summary && !context.verbose {
            self.display_failure_details();
        }
    }

    fn display_compact_results(&self, skipped: usize, total: usize, verbose: bool) {
        // Only display compact results in non-verbose mode
        if verbose {
            return;
        }
        
        let formatter = TestSummaryFormatter::new(self.disable_color);
        let output = formatter.format_compact_results(&self.results, skipped, total);
        print!("{output}");
    }

    fn display_failure_details(&self) {
        let formatter = TestSummaryFormatter::new(self.disable_color);
        let output = formatter.format_failure_details(&self.results);
        print!("{output}");
    }
    
    pub const MAX_SUMMARY_BODY_BYTES: usize = 8192;
    
    #[must_use]
    pub fn format_response_body(&self, body: &Value) -> String {
        let formatter = TestSummaryFormatter::new(self.disable_color);
        formatter.format_response_body(body)
    }
}
