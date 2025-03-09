use crate::dsl::parser::parse_tests;

pub fn validate() {
    match parse_tests() {
        Ok(test_suite) => {
            if test_suite.tests.is_empty() {
                println!("Validation failed: No tests found in `tests.toml`.");
            } else {
                println!(
                    "Validation successful: Found {} tests.",
                    test_suite.tests.len()
                );
            }

            for test in &test_suite.tests {
                if test.name.is_empty() {
                    println!("Error: A test is missing a name.");
                }
                if test.method.is_empty() {
                    println!("Error: Test `{}` is missing an HTTP method.", test.name);
                }
                if test.endpoint.is_empty() {
                    println!("Error: Test `{}` is missing an endpoint.", test.name);
                }
                if !["GET", "POST", "PUT", "DELETE"].contains(&test.method.as_str()) {
                    println!(
                        "Error: Test `{}` has an invalid HTTP method `{}`.",
                        test.name, test.method
                    );
                }
            }
        }
        Err(err) => println!("Validation failed: {}", err),
    }
}
