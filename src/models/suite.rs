use crate::models::command::CommandStep;
use crate::models::config::Config;
use crate::models::test::Test;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TestSuite {
    pub config: Config,
    pub tests: Vec<Test>,
    pub setup: Option<Vec<CommandStep>>,
    pub teardown: Option<Vec<CommandStep>>,
}
