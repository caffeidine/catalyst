use serde::Deserialize;
use std::collections::HashMap;

/// Represents the type of assertion to be performed on a JSON value
#[derive(Debug, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum JsonAssertion {
    /// Exact match with the provided JSON value
    Exact(serde_json::Value),
    /// Partial match - checks if the response contains all the fields in the provided JSON
    Contains(serde_json::Value),
    /// Regex match - checks if the string representation matches the provided regex pattern
    Regex(String),
    /// JSON path assertion - checks a specific path in the JSON using a regex pattern
    /// Format: (json_path, regex_pattern)
    PathRegex(String, String),
}

#[derive(Debug, Deserialize)]
pub struct Test {
    pub name: String,
    pub method: String,
    pub endpoint: String,
    pub query_params: Option<HashMap<String, String>>,
    pub headers: Option<HashMap<String, String>>,
    pub body: Option<serde_json::Value>,
    pub expected_status: u16,
    /// Expected response body - can be a direct JSON value for backward compatibility
    pub expected_body: Option<serde_json::Value>,
    /// Advanced assertions for the response body
    pub assertions: Option<Vec<JsonAssertion>>,
    pub expected_headers: Option<Vec<(String, String)>>,
    pub store: Option<HashMap<String, String>>,
    pub get_cookie: Option<HashMap<String, String>>,
    /// Maximum allowed response time in milliseconds
    pub max_response_time: Option<u64>,
}
