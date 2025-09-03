use crate::models::command::CommandStep;
use crate::utils::string::replace_variables;
use std::collections::HashMap;
use std::path::Path;
use std::process::Stdio;
use std::time::Duration;
use tokio::io::{AsyncReadExt, BufReader};
use tokio::process::Command;
use tokio::time::timeout;

use super::commands::{CommandError, CommandResult};

/// Handles the execution of command steps with proper separation of concerns
pub struct CommandExecutor<'a> {
    step: &'a CommandStep,
    variables: &'a mut HashMap<String, String>,
    work_dir: &'a Path,
    allowed_commands: Option<&'a [String]>,
}

impl<'a> CommandExecutor<'a> {
    pub fn new(
        step: &'a CommandStep,
        variables: &'a mut HashMap<String, String>,
        work_dir: &'a Path,
        allowed_commands: Option<&'a [String]>,
    ) -> Self {
        Self {
            step,
            variables,
            work_dir,
            allowed_commands,
        }
    }

    /// Execute the command step with all validation and processing
    pub async fn execute(mut self) -> Result<CommandResult, CommandError> {
        // Step 1: Check conditional execution
        if !self.should_execute()? {
            return Ok(self.create_skipped_result());
        }

        // Step 2: Validate command is allowed
        self.validate_command()?;

        // Step 3: Build the command
        let mut cmd = self.build_command()?;

        // Step 4: Execute with timeout
        let (stdout, stderr, exit_code) = self.execute_with_timeout(&mut cmd).await?;

        // Step 5: Process output and capture variables
        Ok(self.process_output(stdout, stderr, exit_code))
    }

    /// Check if the command should execute based on conditions
    fn should_execute(&self) -> Result<bool, CommandError> {
        if let Some(when_condition) = &self.step.when {
            let condition = replace_variables(when_condition, self.variables);
            Ok(super::commands::evaluate_condition(&condition))
        } else {
            Ok(true)
        }
    }

    /// Create a result for skipped commands
    fn create_skipped_result(&self) -> CommandResult {
        CommandResult {
            success: true,
            stdout: String::new(),
            stderr: String::new(),
            exit_code: Some(0),
            captured_vars: HashMap::new(),
        }
    }

    /// Validate that the command is in the allowed list
    fn validate_command(&self) -> Result<(), CommandError> {
        if let Some(allowed) = self.allowed_commands {
            let command_to_check = if self.step.should_use_shell() {
                self.step.args
                    .as_ref()
                    .and_then(|args| args.get(1))
                    .unwrap_or(&self.step.run)
            } else {
                &self.step.run
            };
            
            let command_name = command_to_check.split_whitespace().next().unwrap_or(command_to_check);
            if !allowed.contains(&command_name.to_string()) {
                return Err(CommandError::Validation(format!(
                    "Command '{command_name}' is not in allowed_commands list"
                )));
            }
        }
        Ok(())
    }

    /// Build the command with all substitutions and configurations
    fn build_command(&self) -> Result<Command, CommandError> {
        // Substitute variables in command components
        let run_cmd = replace_variables(&self.step.run, self.variables);
        let args = self.step
            .args
            .as_ref()
            .map(|a| a.iter().map(|arg| replace_variables(arg, self.variables)).collect::<Vec<_>>());

        // Prepare working directory
        let dir_path = self.step.dir.as_ref().map(|dir| replace_variables(dir, self.variables));
        let exec_dir = dir_path.as_deref().map_or(self.work_dir, Path::new);

        // Build command
        let mut cmd = if self.step.should_use_shell() {
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
        if let Some(env_vars) = &self.step.env {
            for (key, value) in env_vars {
                let substituted_value = replace_variables(value, self.variables);
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

        Ok(cmd)
    }

    /// Execute the command with timeout
    async fn execute_with_timeout(&self, cmd: &mut Command) -> Result<(String, String, Option<i32>), CommandError> {
        let timeout_duration = Duration::from_millis(self.step.get_timeout_ms());
        
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

        match child_result {
            Ok(result) => result,
            Err(_) => Err(CommandError::Timeout),
        }
    }

    /// Process command output and handle variable capture/export
    fn process_output(&mut self, stdout: String, stderr: String, exit_code: Option<i32>) -> CommandResult {
        let success = exit_code.unwrap_or(-1) == 0;
        let mut captured_vars = HashMap::new();

        // Handle capture
        if let Some(capture) = &self.step.capture {
            captured_vars.insert(capture.var.clone(), stdout.clone());
            captured_vars.insert(format!("{}_stderr", capture.var), stderr.clone());
        }

        // Handle export (JSON parsing)
        if let Some(exports) = &self.step.export && !stdout.trim().is_empty() {
            match serde_json::from_str::<serde_json::Value>(&stdout) {
                Ok(json_value) => {
                    for (var_name, json_path) in exports {
                        if let Some(extracted) = super::commands::extract_json_path(&json_value, json_path) {
                            captured_vars.insert(var_name.clone(), extracted);
                        }
                    }
                }
                Err(_) if !self.step.should_ignore_error() => {
                    // For now, we'll just skip the JSON parsing error
                    // In a real implementation, we might want to handle this differently
                }
                _ => {} // Ignore JSON parsing errors if ignore_error is true
            }
        }

        // Update variables with captured values
        for (key, value) in &captured_vars {
            self.variables.insert(key.clone(), value.clone());
        }

        CommandResult {
            success,
            stdout,
            stderr,
            exit_code,
            captured_vars,
        }
    }
}