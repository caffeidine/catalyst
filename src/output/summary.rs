use crate::core::runner::TestResult;
use colored::Colorize;
use serde_json::Value;
use std::env;

/// Handles formatting of test result summaries and failure details
pub struct TestSummaryFormatter {
    pub disable_color: bool,
}

impl TestSummaryFormatter {
    pub fn new(disable_color: bool) -> Self {
        Self { disable_color }
    }

    /// Format compact results list (non-verbose mode only)
    pub fn format_compact_results(&self, results: &[TestResult], skipped: usize, total: usize) -> String {
        let mut output = String::new();

        if !self.disable_color {
            output.push_str(&format!("\n{}\n", "━".repeat(get_terminal_width()).blue()));
            output.push_str("\nResults:\n");
            
            for result in results {
                let status_indicator = if result.success {
                    "✓".green()
                } else {
                    "✗".red()
                };
                
                if result.success {
                    output.push_str(&format!("  {} {}\n", status_indicator, result.name));
                } else {
                    output.push_str(&format!(
                        "  {} {} (expected {}, got {})\n", 
                        status_indicator, 
                        result.name,
                        result.expected_status,
                        result.actual_status.to_string().red()
                    ));
                }
            }
            
            output.push('\n');
            let success_count = results.iter().filter(|r| r.success).count();
            let fail_count = results.len() - success_count;

            if success_count > 0 {
                output.push_str(&format!("{} passed", format!("{success_count} tests").green()));
            }
            if fail_count > 0 {
                if success_count > 0 {
                    output.push_str(", ");
                }
                output.push_str(&format!("{} failed", format!("{fail_count} tests").red()));
            }
            if skipped > 0 {
                if success_count > 0 || fail_count > 0 {
                    output.push_str(", ");
                }
                output.push_str(&format!("{} skipped", format!("{skipped} tests").yellow()));
            }
            output.push_str(&format!(" (total: {total})\n"));
        } else {
            output.push_str("\nResults:\n");
            for result in results {
                if result.success {
                    output.push_str(&format!("  ✓ {}\n", result.name));
                } else {
                    output.push_str(&format!(
                        "  ✗ {} (expected {}, got {})\n", 
                        result.name, result.expected_status, result.actual_status
                    ));
                }
            }
            
            let success_count = results.iter().filter(|r| r.success).count();
            let fail_count = results.len() - success_count;
            
            output.push_str(&format!("\n{success_count} tests passed"));
            if fail_count > 0 {
                output.push_str(&format!(", {fail_count} tests failed"));
            }
            if skipped > 0 {
                output.push_str(&format!(", {skipped} tests skipped"));
            }
            output.push_str(&format!(" (total: {total})\n"));
        }

        output
    }

    /// Format failure details section
    pub fn format_failure_details(&self, results: &[TestResult]) -> String {
        let failed_results: Vec<_> = results.iter().filter(|r| !r.success).collect();
        
        if failed_results.is_empty() {
            return String::new();
        }
        
        let mut output = String::new();
        
        if !self.disable_color {
            output.push_str(&format!("\n{}\n", "━".repeat(get_terminal_width()).blue()));
            output.push_str(&format!("\n{}\n", "Failures:".red().bold()));
        } else {
            output.push_str("\nFailures:\n");
        }
        
        for (i, result) in failed_results.iter().enumerate() {
            if i > 0 {
                output.push('\n');
            }
            
            if !self.disable_color {
                output.push_str("---\n");
                output.push_str(&format!("Test: {}\n", result.name.bold()));
                output.push_str(&format!(
                    "Endpoint: {} {}\n", 
                    result.method.yellow(), 
                    result.endpoint.yellow()
                ));
                output.push_str(&format!(
                    "Status: {} (expected {})\n", 
                    result.actual_status.to_string().red(),
                    result.expected_status.to_string().bold()
                ));
            } else {
                output.push_str("---\n");
                output.push_str(&format!("Test: {}\n", result.name));
                output.push_str(&format!("Endpoint: {} {}\n", result.method, result.endpoint));
                output.push_str(&format!("Status: {} (expected {})\n", result.actual_status, result.expected_status));
            }
            
            if !result.messages.is_empty() {
                output.push_str("Messages:\n");
                for msg in &result.messages {
                    if !self.disable_color {
                        output.push_str(&format!("  - {}\n", msg.red()));
                    } else {
                        output.push_str(&format!("  - {msg}\n"));
                    }
                }
            }
            
            if let Some(body) = &result.response_body {
                output.push_str("Response Body:\n");
                let body_str = self.format_response_body(body);
                output.push_str(&format!("{body_str}\n"));
            } else {
                output.push_str("Response Body: <none>\n");
            }
        }

        output
    }
    
    /// Maximum bytes to display in failure summary
    pub const MAX_SUMMARY_BODY_BYTES: usize = 8192;
    
    /// Format response body with truncation for failure summary
    pub fn format_response_body(&self, body: &Value) -> String {
        let formatted = if body.is_object() || body.is_array() {
            serde_json::to_string_pretty(body).unwrap_or_else(|_| body.to_string())
        } else {
            body.to_string()
        };
        
        if formatted.len() > Self::MAX_SUMMARY_BODY_BYTES {
            let truncated = &formatted[..Self::MAX_SUMMARY_BODY_BYTES];
            format!("{truncated}\n... (truncated)")
        } else {
            formatted
        }
    }
}

fn get_terminal_width() -> usize {
    env::var("COLUMNS")
        .ok()
        .and_then(|cols| cols.parse().ok())
        .unwrap_or(80)
}