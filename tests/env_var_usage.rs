use catalyst::engine::variables::load_env_files;
use catalyst::utils::string::replace_variables;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Write;

#[test]
fn test_env_var_resolution() {
    unsafe {
        env::set_var("CATALYST_ENV_VAR_TEST", "test_value");
        let input = "${{CATALYST_ENV_VAR_TEST}}";
        let result = replace_variables(input, &HashMap::new());
        assert_eq!(result, "test_value");

        let input = "${{NON_EXISTENT_VAR}}";
        let result = replace_variables(input, &HashMap::new());
        assert_eq!(result, "${{NON_EXISTENT_VAR}}");

        let input = "normal_string";
        let result = replace_variables(input, &HashMap::new());
        assert_eq!(result, "normal_string");

        env::remove_var("CATALYST_ENV_VAR_TEST");
    }
}

#[test]
fn test_env_file_loading() {
    let mut file = File::create(".env").unwrap();
    writeln!(file, "TEST_ENV_VAR=env_file_value").unwrap();

    load_env_files();

    assert_eq!(env::var("TEST_ENV_VAR").unwrap(), "env_file_value");

    std::fs::remove_file(".env").unwrap();
}
