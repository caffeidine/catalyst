use crate::models::command::CommandStep;
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug)]
pub struct CommandResult {
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
    pub exit_code: Option<i32>,
    pub captured_vars: HashMap<String, String>,
}

#[derive(Debug)]
pub enum CommandError {
    Timeout,
    ExecutionFailed(String),
    JsonParsingFailed(String),
    Validation(String),
}

impl std::fmt::Display for CommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommandError::Timeout => write!(f, "Command timed out"),
            CommandError::ExecutionFailed(msg) => write!(f, "Command execution failed: {msg}"),
            CommandError::JsonParsingFailed(msg) => write!(f, "JSON parsing failed: {msg}"),
            CommandError::Validation(msg) => write!(f, "Validation error: {msg}"),
        }
    }
}

impl std::error::Error for CommandError {}

/// Executes a command step with variable substitution and captures output
/// 
/// # Errors
/// Returns `CommandError` if:
/// - Command execution times out
/// - Command fails and `ignore_error` is false
/// - Command is not in the allowed commands list
/// - JSON parsing fails for export operations
/// 
/// # Panics
/// May panic if process spawning fails in unexpected ways
pub async fn execute_command_step(
    step: &CommandStep,
    variables: &mut HashMap<String, String>,
    work_dir: &Path,
    allowed_commands: Option<&[String]>,
) -> Result<CommandResult, CommandError> {
    let executor = super::executor::CommandExecutor::new(step, variables, work_dir, allowed_commands);
    executor.execute().await
}

#[must_use]
pub fn evaluate_condition(condition: &str) -> bool {
    // Simple string comparison evaluation
    // Supports formats like: "{{var}}" == "value" or "{{var}}" != "value"
    if let Some(eq_pos) = condition.find("==") {
        let left = condition[..eq_pos].trim().trim_matches('"');
        let right = condition[eq_pos + 2..].trim().trim_matches('"');
        return left == right;
    } else if let Some(ne_pos) = condition.find("!=") {
        let left = condition[..ne_pos].trim().trim_matches('"');
        let right = condition[ne_pos + 2..].trim().trim_matches('"');
        return left != right;
    }
    
    // Default: treat non-empty strings as true
    !condition.trim().is_empty() && condition.trim() != "false"
}

#[must_use]
pub fn extract_json_path(json: &serde_json::Value, path: &str) -> Option<String> {
    crate::utils::json_path::extract_json_path(json, path)
}

/// Executes multiple command steps in sequence
/// 
/// # Errors
/// Returns `CommandError` if any step fails and has `ignore_error = false`
pub async fn execute_command_steps(
    steps: &[CommandStep],
    variables: &mut HashMap<String, String>,
    work_dir: &Path,
    allowed_commands: Option<&[String]>,
    phase_name: &str,
) -> Result<(), CommandError> {
    for (i, step) in steps.iter().enumerate() {
        crate::debug!("Executing {} step {}: {:?}", phase_name, i + 1, step.run);
        
        match execute_command_step(step, variables, work_dir, allowed_commands).await {
            Ok(result) => {
                if !result.success && !step.should_ignore_error() {
                    return Err(CommandError::ExecutionFailed(format!(
                        "{phase_name} step {} failed with exit code {:?}: {}",
                        i + 1,
                        result.exit_code,
                        result.stderr
                    )));
                }
                crate::debug!(
                    "{phase_name} step {} completed successfully",
                    i + 1
                );
            }
            Err(e) if step.should_ignore_error() => {
                crate::debug!(
                    "{phase_name} step {} failed but ignored: {e}",
                    i + 1
                );
            }
            Err(e) => return Err(e),
        }
    }
    Ok(())
}