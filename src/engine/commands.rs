use crate::utils::string::replace_variables;
use crate::models::command::CommandStep;
use std::collections::HashMap;
use std::path::Path;
use std::process::Stdio;
use std::time::Duration;
use tokio::io::{AsyncReadExt, BufReader};
use tokio::process::Command;
use tokio::time::timeout;

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
    // Check conditional execution
    if let Some(when_condition) = &step.when {
        let condition = replace_variables(when_condition, variables);
        if !evaluate_condition(&condition) {
            return Ok(CommandResult {
                success: true,
                stdout: String::new(),
                stderr: String::new(),
                exit_code: Some(0),
                captured_vars: HashMap::new(),
            });
        }
    }

    // Validate allowed commands
    if let Some(allowed) = allowed_commands {
        let command_to_check = if step.should_use_shell() {
            step.args
                .as_ref()
                .and_then(|args| args.get(1))
                .unwrap_or(&step.run)
        } else {
            &step.run
        };
        
        let command_name = command_to_check.split_whitespace().next().unwrap_or(command_to_check);
        if !allowed.contains(&command_name.to_string()) {
            return Err(CommandError::Validation(format!(
                "Command '{command_name}' is not in allowed_commands list"
            )));
        }
    }

    // Substitute variables in command components
    let run_cmd = replace_variables(&step.run, variables);
    let args = step
        .args
        .as_ref()
        .map(|a| a.iter().map(|arg| replace_variables(arg, variables)).collect::<Vec<_>>());

    // Prepare working directory
    let dir_path = step.dir.as_ref().map(|dir| replace_variables(dir, variables));
    let exec_dir = dir_path.as_deref().map_or(work_dir, Path::new);

    // Build command
    let mut cmd = if step.should_use_shell() {
        let mut shell_cmd = if cfg!(unix) {
            let mut c = Command::new("sh");
            c.arg("-lc");
            c
        } else {
            let mut c = Command::new("cmd");
            c.arg("/C");
            c
        };
        
        if let Some(args) = &args {
            shell_cmd.args(args);
        } else {
            shell_cmd.arg(&run_cmd);
        }
        shell_cmd
    } else {
        let mut direct_cmd = Command::new(&run_cmd);
        if let Some(args) = &args {
            direct_cmd.args(args);
        }
        direct_cmd
    };

    cmd.current_dir(exec_dir);
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());

    // Set environment variables
    if let Some(env_vars) = &step.env {
        for (key, value) in env_vars {
            let substituted_value = replace_variables(value, variables);
            // Redact sensitive environment variables in debug output
            if key.to_lowercase().contains("token") 
                || key.to_lowercase().contains("password") 
                || key.to_lowercase().contains("secret") 
                || key.to_lowercase().contains("key") {
                crate::debug!("Setting env var {}: [REDACTED]", key);
            } else {
                crate::debug!("Setting env var {}: {}", key, substituted_value);
            }
            cmd.env(key, substituted_value);
        }
    }

    // Execute with timeout
    let timeout_duration = Duration::from_millis(step.get_timeout_ms());
    
    let child_result = timeout(timeout_duration, async {
        let mut child = cmd.spawn().map_err(|e| {
            CommandError::ExecutionFailed(format!("Failed to spawn process: {e}"))
        })?;

        let stdout = child.stdout.take().unwrap();
        let stderr = child.stderr.take().unwrap();

        let mut stdout_reader = BufReader::new(stdout);
        let mut stderr_reader = BufReader::new(stderr);

        let mut stdout_content = String::new();
        let mut stderr_content = String::new();

        let read_stdout = stdout_reader.read_to_string(&mut stdout_content);
        let read_stderr = stderr_reader.read_to_string(&mut stderr_content);
        let wait_child = child.wait();

        let (stdout_res, stderr_res, exit_status) = tokio::join!(read_stdout, read_stderr, wait_child);

        stdout_res.map_err(|e| CommandError::ExecutionFailed(format!("Failed to read stdout: {e}")))?;
        stderr_res.map_err(|e| CommandError::ExecutionFailed(format!("Failed to read stderr: {e}")))?;
        let status = exit_status.map_err(|e| CommandError::ExecutionFailed(format!("Failed to wait for process: {e}")))?;

        Ok::<_, CommandError>((stdout_content, stderr_content, status.code()))
    }).await;

    let (stdout, stderr, exit_code) = match child_result {
        Ok(result) => result?,
        Err(_) => return Err(CommandError::Timeout),
    };

    let success = exit_code.unwrap_or(-1) == 0;
    let mut captured_vars = HashMap::new();

    // Handle capture
    if let Some(capture) = &step.capture {
        captured_vars.insert(capture.var.clone(), stdout.clone());
        captured_vars.insert(format!("{}_stderr", capture.var), stderr.clone());
    }

    // Handle export (JSON parsing)
    if let Some(exports) = &step.export
        && !stdout.trim().is_empty() {
        match serde_json::from_str::<serde_json::Value>(&stdout) {
            Ok(json_value) => {
                for (var_name, json_path) in exports {
                    if let Some(extracted) = extract_json_path(&json_value, json_path) {
                        captured_vars.insert(var_name.clone(), extracted);
                    }
                }
            }
            Err(e) if !step.should_ignore_error() => {
                return Err(CommandError::JsonParsingFailed(format!(
                    "Failed to parse stdout as JSON: {e}"
                )));
            }
            _ => {} // Ignore JSON parsing errors if ignore_error is true
        }
    }

    // Update variables with captured values
    for (key, value) in &captured_vars {
        variables.insert(key.clone(), value.clone());
    }

    Ok(CommandResult {
        success,
        stdout,
        stderr,
        exit_code,
        captured_vars,
    })
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

fn extract_json_path(json: &serde_json::Value, path: &str) -> Option<String> {
    // Simple JSONPath implementation for basic paths like "$.field" or "$.nested.field"
    let path = path.strip_prefix("$.").unwrap_or(path);
    let parts: Vec<&str> = path.split('.').collect();
    
    let mut current = json;
    for part in parts {
        current = current.get(part)?;
    }
    
    Some(match current {
        serde_json::Value::String(s) => s.clone(),
        other => other.to_string().trim_matches('"').to_string(),
    })
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