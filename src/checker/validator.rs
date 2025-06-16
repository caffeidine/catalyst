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
            }
        }
        Err(err) => println!("Validation failed: {}", err),
    }
}

fn validate_body_file(test_name: &str, body_file: &str, test_file_dir: &Path) {
    if body_file.is_empty() {
        println!("Error: Test `{}` has an empty `body_file` path.", test_name);
        return;
    }

    if body_file.contains("..") {
        println!(
            "Error: Test `{}` has an invalid `body_file` path `{}` - path traversal is not allowed.",
            test_name, body_file
        );
        return;
    }

    if Path::new(body_file).is_absolute() {
        println!(
            "Error: Test `{}` has an invalid `body_file` path `{}` - absolute paths are not allowed.",
            test_name, body_file
        );
        return;
    }

    let full_path = test_file_dir.join(body_file);

    if !full_path.exists() {
        println!(
            "Error: Test `{}` references a non-existent `body_file`: `{}`",
            test_name, body_file
        );
        return;
    }

    if !full_path.is_file() {
        println!(
            "Error: Test `{}` references a `body_file` that is not a file: `{}`",
            test_name, body_file
        );
        return;
    }

    if body_file.ends_with(".json") {
        if let Err(e) = validate_json_file(&full_path) {
            println!(
                "Error: Test `{}` references an invalid JSON file `{}`: {}",
                test_name, body_file, e
            );
        }
    }
}

fn validate_json_file(file_path: &Path) -> Result<(), String> {
    use std::fs;

    let content = fs::read_to_string(file_path).map_err(|e| format!("Cannot read file: {}", e))?;

    serde_json::from_str::<serde_json::Value>(&content)
        .map_err(|e| format!("Invalid JSON syntax: {}", e))?;

    Ok(())
}
