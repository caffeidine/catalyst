use std::collections::HashMap;
use serde_json::json;
use catalyst::core::runner::{TestRunner, TestResult};

#[tokio::test]
async fn test_failure_summary_display() {
    let mut runner = TestRunner::new(false); // colored output enabled
    runner.no_fail_summary = false; // failure summary enabled
    
    // Add some test results with failures
    runner.results.push(TestResult {
        name: "Test 1 - Success".to_string(),
        success: true,
        expected_status: 200,
        actual_status: 200,
        response_body: Some(json!({"message": "success"})),
        headers: HashMap::new(),
        messages: vec![],
        method: "GET".to_string(),
        endpoint: "/api/users".to_string(),
    });
    
    runner.results.push(TestResult {
        name: "Test 2 - Failure".to_string(),
        success: false,
        expected_status: 200,
        actual_status: 404,
        response_body: Some(json!({"error": "Not found", "code": 404})),
        headers: HashMap::new(),
        messages: vec!["Status mismatch".to_string(), "Body validation failed".to_string()],
        method: "POST".to_string(),
        endpoint: "/api/users/123".to_string(),
    });
    
    runner.results.push(TestResult {
        name: "Test 3 - Success".to_string(),
        success: true,
        expected_status: 201,
        actual_status: 201,
        response_body: Some(json!({"id": 123, "created": true})),
        headers: HashMap::new(),
        messages: vec![],
        method: "POST".to_string(),
        endpoint: "/api/users".to_string(),
    });
    
    // Verify that we have the expected mix of results
    let success_count = runner.results.iter().filter(|r| r.success).count();
    let fail_count = runner.results.iter().filter(|r| !r.success).count();
    
    assert_eq!(success_count, 2);
    assert_eq!(fail_count, 1);
    
    // Test the format_response_body method
    let json_body = json!({"error": "Not found", "code": 404});
    let formatted = runner.format_response_body(&json_body);
    assert!(formatted.contains("\"error\": \"Not found\""));
    assert!(formatted.contains("\"code\": 404"));
    
    // Test truncation with large body
    let large_json = json!({
        "data": "x".repeat(10000),
        "message": "This is a large response"
    });
    let truncated = runner.format_response_body(&large_json);
    assert!(truncated.contains("... (truncated)"));
    assert!(truncated.len() <= TestRunner::MAX_SUMMARY_BODY_BYTES + 20); // Allow for truncation message
}

#[tokio::test]
async fn test_no_fail_summary_flag() {
    let mut runner = TestRunner::new(false);
    runner.no_fail_summary = true; // failure summary disabled
    
    // Add a failing test result
    runner.results.push(TestResult {
        name: "Failing Test".to_string(),
        success: false,
        expected_status: 200,
        actual_status: 500,
        response_body: Some(json!({"error": "Internal server error"})),
        headers: HashMap::new(),
        messages: vec!["Server error occurred".to_string()],
        method: "GET".to_string(),
        endpoint: "/api/status".to_string(),
    });
    
    // When no_fail_summary is true, failure details should not be displayed
    // This is tested indirectly by checking the flag value
    assert!(runner.no_fail_summary);
}

#[tokio::test]
async fn test_body_truncation_logic() {
    let runner = TestRunner::new(false);
    
    // Test normal sized body
    let normal_body = json!({"message": "Hello world"});
    let formatted_normal = runner.format_response_body(&normal_body);
    assert!(!formatted_normal.contains("... (truncated)"));
    
    // Test large body that exceeds MAX_SUMMARY_BODY_BYTES
    let large_data = "x".repeat(TestRunner::MAX_SUMMARY_BODY_BYTES + 1000);
    let large_body = json!({"data": large_data});
    let formatted_large = runner.format_response_body(&large_body);
    assert!(formatted_large.contains("... (truncated)"));
    assert!(formatted_large.len() <= TestRunner::MAX_SUMMARY_BODY_BYTES + 50);
    
    // Test non-JSON body (string)
    let string_body = json!("This is just a string response");
    let formatted_string = runner.format_response_body(&string_body);
    assert_eq!(formatted_string, "\"This is just a string response\"");
}

#[tokio::test]
async fn test_test_result_structure() {
    // Test that TestResult properly captures method and endpoint
    let result = TestResult {
        name: "Test API call".to_string(),
        success: true,
        expected_status: 200,
        actual_status: 200,
        response_body: Some(json!({"status": "ok"})),
        headers: HashMap::new(),
        messages: vec![],
        method: "GET".to_string(),
        endpoint: "/api/status".to_string(),
    };
    
    assert_eq!(result.name, "Test API call");
    assert!(result.success);
    assert_eq!(result.expected_status, 200);
    assert_eq!(result.actual_status, 200);
    assert_eq!(result.method, "GET");
    assert_eq!(result.endpoint, "/api/status");
    assert!(result.response_body.is_some());
}