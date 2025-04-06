use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Test {
    pub name: String,
    pub method: String,
    pub endpoint: String,
    pub query_params: Option<HashMap<String, String>>,
    pub headers: Option<HashMap<String, String>>,
    pub body: Option<serde_json::Value>,
    pub expected_status: u16,
    pub expected_body: Option<serde_json::Value>,
    pub expected_headers: Option<Vec<(String, String)>>,
    pub store: Option<HashMap<String, String>>,
    pub get_cookie: Option<HashMap<String, String>>,
}
