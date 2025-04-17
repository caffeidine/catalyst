use crate::engine::{test_validator::validate_test, variables::store_variables};
use crate::models::test::Test;
use reqwest::Response;
use serde_json::Value;
use std::collections::HashMap;
use std::time::Instant;

pub struct ExecutionResult {
    pub success: bool,
    pub expected_status: u16,
    pub actual_status: u16,
    pub response_time_ms: u64,
    pub response_body: Option<Value>,
    pub headers: HashMap<String, String>,
}

pub async fn execute_test_case(
    response: Response,
    test: &Test,
    variables: &mut HashMap<String, String>,
    start_time: Instant,
) -> ExecutionResult {
    let status = response.status().as_u16();
    let headers = response.headers().clone();
    let mut headers_map = HashMap::new();

    // Extract headers
    for (key, value) in headers.iter() {
        headers_map.insert(key.to_string(), value.to_str().unwrap_or("").to_string());
    }

    // Parse response body
    let body = parse_response_body(response).await;

    // Calculate response time
    let response_time_ms = start_time.elapsed().as_millis() as u64;

    // Validate test
    let validation_result = validate_test(test, status, &body, response_time_ms, variables);

    // Store variables if test passed
    if validation_result.success {
        store_variables(
            &body,
            test.store.as_ref().unwrap_or(&HashMap::new()),
            &headers_map,
            &test.get_cookie,
            response_time_ms,
            variables,
        );
    }

    ExecutionResult {
        success: validation_result.success,
        expected_status: validation_result.expected_status,
        actual_status: validation_result.actual_status,
        response_time_ms,
        response_body: Some(body),
        headers: headers_map,
    }
}

async fn parse_response_body(response: Response) -> Value {
    let content_type = response
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    if content_type.contains("application/json") {
        match response.json().await {
            Ok(json) => json,
            Err(err) => {
                println!("Error parsing response body as JSON: {}", err);
                Value::Null
            }
        }
    } else {
        match response.text().await {
            Ok(text) => match serde_json::from_str(&text) {
                Ok(json_value) => json_value,
                Err(_) => Value::String(text),
            },
            Err(err) => {
                println!("Error reading response body as text: {}", err);
                Value::Null
            }
        }
    }
}
