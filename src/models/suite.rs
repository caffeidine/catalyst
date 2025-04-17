use crate::models::config::Config;
use crate::models::test::Test;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TestSuite {
    pub config: Config,
    pub tests: Vec<Test>,
}
