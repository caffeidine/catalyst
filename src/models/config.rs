use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub base_url: String,
    pub auth_method: Option<String>,
    pub auth_token: Option<String>,
    pub default_headers: Option<HashMap<String, String>>,
}
