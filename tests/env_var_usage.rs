use catalyst::engine::variables::{load_env_files, resolve_env_var};
use std::env;
use std::fs::File;
use std::io::Write;

#[test]
fn test_env_var_resolution() {
    unsafe {
        env::set_var("CATALYST_ENV_VAR_TEST", "test_value");
        let input = "${{CATALYST_ENV_VAR_TEST}}";
        let value = resolve_env_var(input);
        assert_eq!(value, "test_value");

        // Test non-existent variable returns empty string
        let input = "${{NON_EXISTENT_VAR}}";
        let value = resolve_env_var(input);
        assert_eq!(value, "");
    }
}

#[test]
fn test_env_file_loading() {
    // Create temporary .env file
    let mut file = File::create(".env").unwrap();
    writeln!(file, "TEST_ENV_VAR=env_file_value").unwrap();

    // Load environment files
    load_env_files();

    // Check if variable was loaded
    assert_eq!(env::var("TEST_ENV_VAR").unwrap(), "env_file_value");

    // Clean up
    std::fs::remove_file(".env").unwrap();
}
