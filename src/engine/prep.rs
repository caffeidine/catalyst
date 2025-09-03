// This module is currently unused as body resolution logic
// has been moved to execution.rs to properly handle test file directories.
// Keeping for potential future use.

use crate::{
    models::{suite::TestSuite, test::Test},
    utils::{file::load_body_from_file, string::replace_variables},
};
use serde_json::Value;
use std::collections::HashMap;
use std::path::Path;

pub struct Request {
    pub url: String,
    pub headers: Option<Vec<(String, String)>>,
    pub params: Option<Vec<(String, String)>>,
    pub body: Option<Value>,
}

pub fn build(test: &Test, suite: &TestSuite, vars: &HashMap<String, String>, test_file_dir: &Path) -> Request {
    Request {
        url: format!(
            "{}{}",
            suite.config.base_url,
            replace_variables(&test.endpoint, vars)
        ),
        headers: build_headers(test, suite, vars),
        params: test.query_params.as_ref().map(|p| {
            p.iter()
                .map(|(k, v)| (k.clone(), replace_variables(v, vars)))
                .collect()
        }),
        body: resolve_body(test, vars, test_file_dir),
    }
}

fn build_headers(
    test: &Test,
    suite: &TestSuite,
    vars: &HashMap<String, String>,
) -> Option<Vec<(String, String)>> {
    let mut headers = Vec::new();

    if let Some(default_headers) = &suite.config.default_headers {
        headers.extend(
            default_headers
                .iter()
                .map(|(k, v)| (k.clone(), replace_variables(v, vars))),
        );
    }

    if let Some(test_headers) = &test.headers {
        headers.extend(
            test_headers
                .iter()
                .map(|(k, v)| (k.clone(), replace_variables(v, vars))),
        );
    }

    if let (Some(method), Some(token)) = (&suite.config.auth_method, &suite.config.auth_token) {
        let processed_token = replace_variables(token, vars);
        let auth_value = match method.to_lowercase().as_str() {
            "bearer" => format!("Bearer {}", processed_token),
            "basic" => format!("Basic {}", processed_token),
            _ => processed_token,
        };
        headers.push(("Authorization".to_string(), auth_value));
    }

    if headers.is_empty() {
        None
    } else {
        Some(headers)
    }
}

fn resolve_body(test: &Test, vars: &HashMap<String, String>, test_file_dir: &Path) -> Option<Value> {
    if let Some(body) = &test.body {
        // Existing logic for inline body
        let body_str = serde_json::to_string(body).unwrap_or_default();
        Some(serde_json::from_str(&replace_variables(&body_str, vars)).unwrap_or(Value::Null))
    } else if let Some(body_file) = &test.body_file {
        // New: File-based body logic
        match load_body_from_file(body_file, test_file_dir, vars) {
            Ok(content) => Some(content),
            Err(e) => {
                eprintln!("Error loading body file '{}': {}", body_file, e);
                None
            }
        }
    } else {
        None
    }
}
