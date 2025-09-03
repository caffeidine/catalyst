use catalyst::models::command::{CommandStep, CaptureConfig};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[cfg(test)]
mod command_step_tests {
    use super::*;

    #[test]
    fn test_command_step_defaults() {
        let step = CommandStep {
            run: "echo".to_string(),
            args: None,
            shell: None,
            dir: None,
            env: None,
            timeout_ms: None,
            ignore_error: None,
            capture: None,
            export: None,
            when: None,
            on: None,
        };

        assert!(!step.should_ignore_error());
        assert!(!step.should_use_shell());
        assert_eq!(step.get_timeout_ms(), 30000);
        assert_eq!(step.get_on_condition(), "always");
    }

    #[test]
    fn test_command_step_with_values() {
        let step = CommandStep {
            run: "echo".to_string(),
            args: Some(vec!["hello".to_string()]),
            shell: Some(true),
            dir: Some("./test".to_string()),
            env: None,
            timeout_ms: Some(5000),
            ignore_error: Some(true),
            capture: Some(CaptureConfig { var: "output".to_string() }),
            export: None,
            when: Some("{{condition}} == \"true\"".to_string()),
            on: Some("success".to_string()),
        };

        assert!(step.should_ignore_error());
        assert!(step.should_use_shell());
        assert_eq!(step.get_timeout_ms(), 5000);
        assert_eq!(step.get_on_condition(), "success");
    }
}

#[cfg(test)]
mod command_execution_tests {
    use super::*;
    use catalyst::engine::commands::{execute_command_step, evaluate_condition};

    #[tokio::test]
    async fn test_simple_echo_command() {
        let step = CommandStep {
            run: "echo".to_string(),
            args: Some(vec!["hello".to_string(), "world".to_string()]),
            shell: None,
            dir: None,
            env: None,
            timeout_ms: Some(5000),
            ignore_error: None,
            capture: None,
            export: None,
            when: None,
            on: None,
        };

        let mut variables = HashMap::new();
        let work_dir = Path::new(".");
        let allowed_commands: Option<&[String]> = None;

        let result = execute_command_step(&step, &mut variables, work_dir, allowed_commands).await;
        
        assert!(result.is_ok());
        let cmd_result = result.unwrap();
        assert!(cmd_result.success);
        assert!(cmd_result.stdout.trim().contains("hello world"));
    }

    #[tokio::test]
    async fn test_command_with_capture() {
        let step = CommandStep {
            run: "echo".to_string(),
            args: Some(vec!["test_output".to_string()]),
            shell: None,
            dir: None,
            env: None,
            timeout_ms: Some(5000),
            ignore_error: None,
            capture: Some(CaptureConfig { var: "result".to_string() }),
            export: None,
            when: None,
            on: None,
        };

        let mut variables = HashMap::new();
        let work_dir = Path::new(".");
        let allowed_commands: Option<&[String]> = None;

        let result = execute_command_step(&step, &mut variables, work_dir, allowed_commands).await;
        
        assert!(result.is_ok());
        let cmd_result = result.unwrap();
        assert!(cmd_result.success);
        assert!(variables.contains_key("result"));
        assert!(variables.get("result").unwrap().contains("test_output"));
        assert!(variables.contains_key("result_stderr"));
    }

    #[tokio::test]
    async fn test_command_with_environment() {
        let mut env_vars = HashMap::new();
        env_vars.insert("TEST_VAR".to_string(), "test_value".to_string());

        let step = CommandStep {
            run: if cfg!(unix) { "sh".to_string() } else { "cmd".to_string() },
            args: Some(if cfg!(unix) { 
                vec!["-c".to_string(), "echo $TEST_VAR".to_string()]
            } else { 
                vec!["/C".to_string(), "echo %TEST_VAR%".to_string()]
            }),
            shell: None,
            dir: None,
            env: Some(env_vars),
            timeout_ms: Some(5000),
            ignore_error: None,
            capture: Some(CaptureConfig { var: "env_result".to_string() }),
            export: None,
            when: None,
            on: None,
        };

        let mut variables = HashMap::new();
        let work_dir = Path::new(".");
        let allowed_commands: Option<&[String]> = None;

        let result = execute_command_step(&step, &mut variables, work_dir, allowed_commands).await;
        
        assert!(result.is_ok());
        let cmd_result = result.unwrap();
        assert!(cmd_result.success);
        assert!(variables.contains_key("env_result"));
        assert!(variables.get("env_result").unwrap().contains("test_value"));
    }

    #[tokio::test]
    async fn test_command_with_json_export() {
        let step = CommandStep {
            run: "echo".to_string(),
            args: Some(vec![r#"{"name": "test", "id": 123}"#.to_string()]),
            shell: None,
            dir: None,
            env: None,
            timeout_ms: Some(5000),
            ignore_error: None,
            capture: Some(CaptureConfig { var: "json_output".to_string() }),
            export: Some({
                let mut exports = HashMap::new();
                exports.insert("extracted_name".to_string(), "$.name".to_string());
                exports.insert("extracted_id".to_string(), "$.id".to_string());
                exports
            }),
            when: None,
            on: None,
        };

        let mut variables = HashMap::new();
        let work_dir = Path::new(".");
        let allowed_commands: Option<&[String]> = None;

        let result = execute_command_step(&step, &mut variables, work_dir, allowed_commands).await;
        
        assert!(result.is_ok());
        let cmd_result = result.unwrap();
        assert!(cmd_result.success);
        assert_eq!(variables.get("extracted_name").unwrap(), "test");
        assert_eq!(variables.get("extracted_id").unwrap(), "123");
    }

    #[tokio::test]
    async fn test_conditional_execution() {
        let step = CommandStep {
            run: "echo".to_string(),
            args: Some(vec!["conditional".to_string()]),
            shell: None,
            dir: None,
            env: None,
            timeout_ms: Some(5000),
            ignore_error: None,
            capture: None,
            export: None,
            when: Some("{{run_test}} == \"true\"".to_string()),
            on: None,
        };

        // Test when condition is false - should skip execution
        let mut variables = HashMap::new();
        variables.insert("run_test".to_string(), "false".to_string());
        let work_dir = Path::new(".");
        let allowed_commands: Option<&[String]> = None;

        let result = execute_command_step(&step, &mut variables, work_dir, allowed_commands).await;
        assert!(result.is_ok());
        let cmd_result = result.unwrap();
        assert!(cmd_result.success);
        assert!(cmd_result.stdout.is_empty()); // Should not have run

        // Test when condition is true - should execute
        variables.insert("run_test".to_string(), "true".to_string());
        let result = execute_command_step(&step, &mut variables, work_dir, allowed_commands).await;
        assert!(result.is_ok());
        let cmd_result = result.unwrap();
        assert!(cmd_result.success);
        assert!(cmd_result.stdout.contains("conditional"));
    }

    #[tokio::test]
    async fn test_allowed_commands_validation() {
        let step = CommandStep {
            run: "forbidden_command".to_string(),
            args: None,
            shell: None,
            dir: None,
            env: None,
            timeout_ms: Some(5000),
            ignore_error: None,
            capture: None,
            export: None,
            when: None,
            on: None,
        };

        let mut variables = HashMap::new();
        let work_dir = Path::new(".");
        let allowed_commands = Some(vec!["echo".to_string(), "ls".to_string()]);

        let result = execute_command_step(&step, &mut variables, work_dir, allowed_commands.as_deref()).await;
        
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not in allowed_commands list"));
    }

    #[test]
    fn test_condition_evaluation() {
        assert!(evaluate_condition("\"true\" == \"true\""));
        assert!(!evaluate_condition("\"true\" == \"false\""));
        assert!(!evaluate_condition("\"false\" == \"true\""));
        assert!(evaluate_condition("\"test\" != \"other\""));
        assert!(!evaluate_condition("\"test\" != \"test\""));
        assert!(evaluate_condition("non-empty-string"));
        assert!(!evaluate_condition(""));
        assert!(!evaluate_condition("false"));
    }
}

#[cfg(test)]
mod validator_tests {
    use super::*;

    #[test]
    fn test_validation_with_command_hooks() {
        // Create a temporary test file
        let test_file = ".catalyst_test_hooks.toml";
        
        let test_content = r#"
[config]
base_url = "http://localhost:8080"
allowed_commands = ["echo", "ls"]

[[setup]]
run = "echo"
args = ["Setting up tests"]

[[teardown]]
run = "echo"
args = ["Cleaning up"]
ignore_error = true

[[tests]]
name = "Test with hooks"
method = "GET"
endpoint = "/test"
expected_status = 200

  [[tests.before]]
  run = "echo"
  args = ["Before test"]
  
  [[tests.after]]
  run = "echo" 
  args = ["After test"]
  on = "always"
"#;
        
        fs::write(test_file, test_content).unwrap();
        
        // This should not panic and should validate successfully
        catalyst::checker::validator::validate(Some(test_file));
        
        // Clean up
        if Path::new(test_file).exists() {
            fs::remove_file(test_file).unwrap();
        }
    }
}