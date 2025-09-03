use crate::core::runner::TestResult;
use colored::Colorize;
use std::env;

/// Handles formatting of individual test results
pub struct TestResultFormatter {
    pub disable_color: bool,
    pub verbose: bool,
}

impl TestResultFormatter {
    pub fn new(disable_color: bool, verbose: bool) -> Self {
        Self {
            disable_color,
            verbose,
        }
    }

    /// Format a single test result for verbose output
    pub fn format_verbose_result(&self, result: &TestResult) -> String {
        let mut output = String::new();
        
        let terminal_width = get_terminal_width();
        
        if !self.disable_color {
            output.push_str(&format!("\n{}\n", "━".repeat(terminal_width).blue()));
            output.push_str(&format!("Test: {}\n", result.name.bold()));
            output.push_str(&format!(
                "Endpoint: {} {}\n",
                result.method.yellow(),
                result.endpoint.yellow()
            ));
            output.push_str(&format!(
                "Success: {}\n",
                if result.success {
                    "Yes".green()
                } else {
                    "No".red()
                }
            ));
        } else {
            output.push_str(&format!("\n{}\n", "━".repeat(terminal_width)));
            output.push_str(&format!("Test: {}\n", result.name));
            output.push_str(&format!("Endpoint: {} {}\n", result.method, result.endpoint));
            output.push_str(&format!(
                "Success: {}\n",
                if result.success { "Yes" } else { "No" }
            ));
        }

        // Add status information
        let status_matches = result.expected_status == result.actual_status;
        if !self.disable_color {
            let status_display = format!(
                "Status: {} (expected {})",
                if status_matches {
                    result.actual_status.to_string().green()
                } else {
                    result.actual_status.to_string().red()
                },
                result.expected_status.to_string().bold()
            );
            output.push_str(&format!("{status_display}\n"));
        } else {
            output.push_str(&format!(
                "Status: {} (expected {})\n",
                result.actual_status, result.expected_status
            ));
        }

        // Add response body if available
        if let Some(body) = &result.response_body {
            let pretty_body = serde_json::to_string_pretty(body).unwrap_or(body.to_string());
            output.push_str(&format!("\nResponse Body: {pretty_body}\n"));
        }

        // Add body comparison for verbose mode
        if let Some(expected_body) = &result.response_body {
            if let Some(actual_body) = &result.response_body {
                output.push_str("\nBody comparison:\n");
                let body_matches = expected_body == actual_body;
                if !self.disable_color {
                    if body_matches {
                        output.push_str(&format!("  {}\n", "✓ Body matches expected value".green()));
                    } else {
                        output.push_str(&format!("  {}\n", "✗ Body differs from expected value".red()));
                        output.push_str(&format!(
                            "    Expected: {}\n",
                            serde_json::to_string_pretty(expected_body).unwrap().green()
                        ));
                        output.push_str(&format!(
                            "    Actual:   {}\n",
                            serde_json::to_string_pretty(actual_body).unwrap().red()
                        ));
                    }
                } else if body_matches {
                    output.push_str("  ✓ Body matches expected value\n");
                } else {
                    output.push_str("  ✗ Body differs from expected value\n");
                    output.push_str(&format!(
                        "    Expected: {}\n",
                        serde_json::to_string_pretty(expected_body).unwrap()
                    ));
                    output.push_str(&format!(
                        "    Actual:   {}\n",
                        serde_json::to_string_pretty(actual_body).unwrap()
                    ));
                }
            }
        }

        // Add error messages
        if !result.messages.is_empty() {
            output.push_str("\nMessages:\n");
            for msg in &result.messages {
                if !self.disable_color {
                    output.push_str(&format!("  {} {}\n", "-".bold(), msg.red()));
                } else {
                    output.push_str(&format!("  - {msg}\n"));
                }
            }
        }

        output
    }

    /// Format a single test result for non-verbose output
    pub fn format_compact_result(&self, result: &TestResult) -> String {
        if !self.disable_color {
            let status_indicator = if result.success {
                "✓".green()
            } else {
                "✗".red()
            };

            if result.success {
                format!("{} {}", status_indicator, result.name)
            } else {
                format!(
                    "{} {} (expected {}, got {})",
                    status_indicator,
                    result.name,
                    result.expected_status,
                    result.actual_status.to_string().red()
                )
            }
        } else if result.success {
            format!("✓ {}", result.name)
        } else {
            format!(
                "✗ {} (expected {}, got {})",
                result.name, result.expected_status, result.actual_status
            )
        }
    }

    /// Format error messages for a failed test
    pub fn format_error_messages(&self, result: &TestResult) -> Vec<String> {
        let mut messages = Vec::new();
        
        // Note: Body mismatch logic handled elsewhere in the codebase

        // Add error messages
        for msg in &result.messages {
            if !self.disable_color {
                messages.push(format!("  {} {}", "-".bold(), msg.red()));
            } else {
                messages.push(format!("  - {msg}"));
            }
        }

        messages
    }
}

fn get_terminal_width() -> usize {
    env::var("COLUMNS")
        .ok()
        .and_then(|cols| cols.parse().ok())
        .unwrap_or(80)
}