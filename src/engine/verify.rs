use crate::models::test::{JsonAssertion, Test};
use serde_json::Value;
use std::collections::HashMap;

pub struct Result {
    pub ok: bool,
    pub status: (u16, u16),
    pub time_ms: u64,
    pub errors: Vec<String>,
}

pub fn check(
    test: &Test,
    status: u16,
    body: &Value,
    time_ms: u64,
    vars: &HashMap<String, String>,
) -> Result {
    let mut errors = Vec::new();

    if status != test.expected_status {
        errors.push(format!(
            "Status {}, expected {}",
            status, test.expected_status
        ));
    }

    if let Some(max) = test.max_response_time {
        if time_ms > max {
            errors.push(format!("Time {}ms > {}ms", time_ms, max));
        }
    }

    if let Some(expected) = &test.expected_body {
        if !super::assertions::body_matches(
            &super::variables::replace_variables_in_json(expected, vars),
            body,
        ) {
            errors.push("Body mismatch".into());
        }
    }

    if let Some(assertions) = &test.assertions {
        for assertion in assertions {
            if !super::assertions::validate_assertion(&process_assertion(assertion, vars), body) {
                errors.push(format!("Failed: {:?}", assertion));
            }
        }
    }

    Result {
        ok: errors.is_empty(),
        status: (test.expected_status, status),
        time_ms,
        errors,
    }
}

fn process_assertion(assertion: &JsonAssertion, vars: &HashMap<String, String>) -> JsonAssertion {
    match assertion {
        JsonAssertion::Exact(v) | JsonAssertion::Contains(v) => {
            let processed = super::variables::replace_variables_in_json(v, vars);
            if matches!(assertion, JsonAssertion::Exact(_)) {
                JsonAssertion::Exact(processed)
            } else {
                JsonAssertion::Contains(processed)
            }
        }
        JsonAssertion::Regex(p) => JsonAssertion::Regex(replace_vars(p, vars)),
        JsonAssertion::PathRegex(path, pattern) => {
            JsonAssertion::PathRegex(replace_vars(path, vars), replace_vars(pattern, vars))
        }
    }
}

fn replace_vars(s: &str, vars: &HashMap<String, String>) -> String {
    let mut result = s.to_string();
    for (k, v) in vars {
        result = result.replace(&format!("{{{{{}}}}}", k), v);
    }
    result
}
