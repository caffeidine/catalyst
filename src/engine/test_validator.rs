use crate::models::test::{JsonAssertion, Test};
use serde_json::Value;
use std::collections::HashMap;

pub struct TestValidationResult {
    pub success: bool,
    pub expected_status: u16,
    pub actual_status: u16,
    pub response_time_ms: u64,
}

pub fn validate_test(
    test: &Test,
    status: u16,
    body: &Value,
    response_time_ms: u64,
    variables: &HashMap<String, String>,
) -> TestValidationResult {
    let mut success = status == test.expected_status;

    // Validate response time if specified
    if let Some(max_time) = test.max_response_time {
        if response_time_ms > max_time {
            println!(
                "Test '{}' failed: Response time {} ms exceeds maximum allowed {} ms",
                test.name, response_time_ms, max_time
            );
            success = false;
        }
    }

    // Validate body if expected_body is specified (backward compatibility)
    if success && test.expected_body.is_some() {
        let expected = test.expected_body.as_ref().unwrap();
        let processed_expected = super::variables::replace_variables_in_json(expected, variables);
        if !super::assertions::body_matches(&processed_expected, body) {
            println!(
                "Test '{}' failed: Response body does not match expected body",
                test.name
            );
            success = false;
        }
    }

    // Validate advanced assertions if specified
    if success && test.assertions.is_some() {
        for assertion in test.assertions.as_ref().unwrap() {
            let processed_assertion = process_assertion_with_variables(assertion, variables);
            if !super::assertions::validate_assertion(&processed_assertion, body) {
                println!(
                    "Test '{}' failed: Assertion failed: {:?}",
                    test.name, processed_assertion
                );
                success = false;
                break;
            }
        }
    }

    TestValidationResult {
        success,
        expected_status: test.expected_status,
        actual_status: status,
        response_time_ms,
    }
}

pub fn process_assertion_with_variables(
    assertion: &JsonAssertion,
    variables: &HashMap<String, String>,
) -> JsonAssertion {
    match assertion {
        JsonAssertion::Exact(value) => JsonAssertion::Exact(
            super::variables::replace_variables_in_json(value, variables),
        ),
        JsonAssertion::Contains(value) => JsonAssertion::Contains(
            super::variables::replace_variables_in_json(value, variables),
        ),
        JsonAssertion::Regex(pattern) => {
            let mut processed_pattern = pattern.clone();
            for (key, value) in variables {
                let var_pattern = format!("{{{{{}}}}}", key);
                processed_pattern = processed_pattern.replace(&var_pattern, value);
            }
            JsonAssertion::Regex(processed_pattern)
        }
        JsonAssertion::PathRegex(path, pattern) => {
            let mut processed_path = path.clone();
            let mut processed_pattern = pattern.clone();

            for (key, value) in variables {
                let var_pattern = format!("{{{{{}}}}}", key);
                processed_path = processed_path.replace(&var_pattern, value);
                processed_pattern = processed_pattern.replace(&var_pattern, value);
            }

            JsonAssertion::PathRegex(processed_path, processed_pattern)
        }
    }
}
