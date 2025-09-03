use crate::checker::parse_tests;
use std::path::Path;

pub fn validate(file_path: Option<&str>) {
    let test_file_path = file_path.unwrap_or(".catalyst/tests.toml");
    let test_file_dir = Path::new(test_file_path).parent().unwrap_or(Path::new("."));

    match parse_tests(file_path) {
        Ok(test_suite) => {
            if test_suite.tests.is_empty() {
                println!("Validation failed: No tests found in `tests.toml`.");
            } else {
                println!(
                    "Validation successful: Found {} tests.",
                    test_suite.tests.len()
                );
            }

            // Validate suite-level command hooks
            if let Some(setup_steps) = &test_suite.setup {
                validate_command_steps(setup_steps, "setup", test_suite.config.allowed_commands.as_deref());
            }
            if let Some(teardown_steps) = &test_suite.teardown {
                validate_command_steps(teardown_steps, "teardown", test_suite.config.allowed_commands.as_deref());
            }

            for test in &test_suite.tests {
                if test.name.is_empty() {
                    println!("Error: A test is missing a name.");
                }
                if test.method.is_empty() {
                    println!("Error: Test `{}` is missing an HTTP method.", test.name);
                }
                if test.endpoint.is_empty() {
                    println!("Error: Test `{}` is missing an endpoint.", test.name);
                }
                if !["GET", "POST", "PUT", "DELETE", "PATCH", "HEAD", "OPTIONS"]
                    .contains(&test.method.to_uppercase().as_str())
                {
                    println!(
                        "Error: Test `{}` has an invalid HTTP method `{}`.",
                        test.name, test.method
                    );
                }

                if test.body.is_some() && test.body_file.is_some() {
                    println!(
                        "Error: Test `{}` cannot have both `body` and `body_file` specified.",
                        test.name
                    );
                }

                if let Some(body_file) = &test.body_file {
                    validate_body_file(&test.name, body_file, test_file_dir);
                }

                // Validate test-level command hooks
                if let Some(before_steps) = &test.before {
                    validate_command_steps(before_steps, &format!("test '{}' before", test.name), test_suite.config.allowed_commands.as_deref());
                }
                if let Some(after_steps) = &test.after {
                    validate_command_steps(after_steps, &format!("test '{}' after", test.name), test_suite.config.allowed_commands.as_deref());
                    
                    // Validate 'on' field for after steps
                    for step in after_steps {
                        if let Some(on_condition) = &step.on
                            && !["success", "failure", "always"].contains(&on_condition.as_str()) {
                            println!(
                                "Error: Test `{}` after hook has invalid 'on' condition '{}'. Must be one of: success, failure, always",
                                test.name, on_condition
                            );
                        }
                    }
                }
            }
        }
        Err(err) => println!("Validation failed: {err}"),
    }
}

fn validate_body_file(test_name: &str, body_file: &str, test_file_dir: &Path) {
    if body_file.is_empty() {
        println!("Error: Test `{test_name}` has an empty `body_file` path.");
        return;
    }

    if body_file.contains("..") {
        println!(
            "Error: Test `{test_name}` has an invalid `body_file` path `{body_file}` - path traversal is not allowed."
        );
        return;
    }

    if Path::new(body_file).is_absolute() {
        println!(
            "Error: Test `{test_name}` has an invalid `body_file` path `{body_file}` - absolute paths are not allowed."
        );
        return;
    }

    let full_path = test_file_dir.join(body_file);

    if !full_path.exists() {
        println!(
            "Error: Test `{test_name}` references a non-existent `body_file`: `{body_file}`"
        );
        return;
    }

    if !full_path.is_file() {
        println!(
            "Error: Test `{test_name}` references a `body_file` that is not a file: `{body_file}`"
        );
        return;
    }

    if std::path::Path::new(body_file)
        .extension()
        .is_some_and(|ext| ext.eq_ignore_ascii_case("json"))
        && let Err(e) = validate_json_file(&full_path) {
        println!(
            "Error: Test `{test_name}` references an invalid JSON file `{body_file}`: {e}"
        );
    }
}

fn validate_json_file(file_path: &Path) -> Result<(), String> {
    use std::fs;

    let content = fs::read_to_string(file_path).map_err(|e| format!("Cannot read file: {e}"))?;

    serde_json::from_str::<serde_json::Value>(&content)
        .map_err(|e| format!("Invalid JSON syntax: {e}"))?;

    Ok(())
}

fn validate_command_steps(
    steps: &[crate::models::command::CommandStep],
    context: &str,
    allowed_commands: Option<&[String]>,
) {
    for (i, step) in steps.iter().enumerate() {
        let step_context = format!("{} step {}", context, i + 1);

        // Validate required fields
        if step.run.is_empty() {
            println!("Error: {step_context} has empty 'run' field.");
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
                println!(
                    "Error: {step_context} uses command '{command_name}' which is not in allowed_commands list."
                );
            }
        }

        // Validate directory paths
        if let Some(dir) = &step.dir {
            if dir.contains("..") {
                println!(
                    "Error: {step_context} has invalid 'dir' path '{dir}' - path traversal is not allowed."
                );
            }
            if Path::new(dir).is_absolute() {
                println!(
                    "Warning: {step_context} uses absolute path '{dir}' - consider using relative paths for portability."
                );
            }
        }

        // Validate timeout
        if let Some(timeout) = step.timeout_ms {
            if timeout == 0 {
                println!(
                    "Warning: {step_context} has timeout_ms set to 0, which may cause immediate timeout."
                );
            }
            if timeout > 600_000 { // 10 minutes
                println!(
                    "Warning: {step_context} has very large timeout_ms ({timeout}ms), consider reducing it."
                );
            }
        }

        // Validate export requires capture or stdout
        if step.export.is_some() && step.capture.is_none() {
            println!(
                "Warning: {step_context} uses 'export' without 'capture'. Export requires JSON stdout which may not be captured."
            );
        }

        // Validate 'on' field only valid for after hooks
        if step.on.is_some() && !context.contains("after") {
            println!(
                "Error: {step_context} uses 'on' field which is only valid for 'after' hooks."
            );
        }
    }
}
