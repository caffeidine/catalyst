use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Clone)]
pub struct CommandStep {
    pub run: String,
    pub args: Option<Vec<String>>,
    pub shell: Option<bool>,
    pub dir: Option<String>,
    pub env: Option<HashMap<String, String>>,
    pub timeout_ms: Option<u64>,
    pub ignore_error: Option<bool>,
    pub capture: Option<CaptureConfig>,
    pub export: Option<HashMap<String, String>>,
    pub when: Option<String>,
    pub on: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CaptureConfig {
    pub var: String,
}

impl CommandStep {
    pub fn should_ignore_error(&self) -> bool {
        self.ignore_error.unwrap_or(false)
    }

    pub fn should_use_shell(&self) -> bool {
        self.shell.unwrap_or(false)
    }

    pub fn get_timeout_ms(&self) -> u64 {
        self.timeout_ms.unwrap_or(30000)
    }

    pub fn get_on_condition(&self) -> &str {
        self.on.as_deref().unwrap_or("always")
    }
}