use catalyst::engine::variables::resolve_variable_value;
use std::env;

#[test]
fn test_env_var_resolution() {
    unsafe {
        env::set_var("CATALYST_ENV_VAR_TEST", "a great value");
        let input = "env_var(\"CATALYST_ENV_VAR_TEST\")";
        let value = resolve_variable_value(input);
        assert_eq!(value, Some("a great value".to_string()));
    };
}
