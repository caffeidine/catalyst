use serde::Deserialize;
use crate::models::{Config, Test};

#[derive(Debug, Deserialize)]
pub struct TestSuite {
    pub config: Config,
    pub tests: Vec<Test>,
}
