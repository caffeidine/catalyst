use crate::models::command::CommandStep;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum JsonAssertion {
    Exact(serde_json::Value),

    Contains(serde_json::Value),

    Regex(String),

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
    pub body_file: Option<String>,
    pub expected_status: u16,

    pub expected_body: Option<serde_json::Value>,

    pub assertions: Option<Vec<JsonAssertion>>,
    pub expected_headers: Option<Vec<(String, String)>>,
    pub store: Option<HashMap<String, String>>,
    pub get_cookie: Option<HashMap<String, String>>,

    pub max_response_time: Option<u64>,

    pub before: Option<Vec<CommandStep>>,
    pub after: Option<Vec<CommandStep>>,
}
