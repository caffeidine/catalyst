# Test File Structure

Catalyst uses a TOML configuration file to define your API tests. This file should be located at `.catalyst/tests.toml` in your project directory.

## Basic Structure

The test file has two main sections:

1. **Global configuration** (`[config]`) - Contains settings that apply to all tests
2. **Test definitions** (`[[tests]]`) - Contains individual test cases

Here's a simple example:

```toml
# Global settings
[config]
base_url = "https://api.example.com"
default_headers = { "Content-Type" = "application/json" }

# Test definitions
[[tests]]
name = "Get Users"
method = "GET"
endpoint = "/users"
expected_status = 200

[[tests]]
name = "Create User"
method = "POST"
endpoint = "/users"
body = { "name" = "John Doe", "email" = "john@example.com" }
expected_status = 201
```

## File Organization

You can organize your tests in any order within the file. Catalyst will execute the tests in the order they are defined, which is important when tests depend on each other (for example, when one test stores a variable that another test uses).

## Comments

You can add comments to your test file using the `#` character:

```toml
# This is a comment
[config]
base_url = "https://api.example.com"  # This is also a comment
```

Comments are useful for documenting your tests and explaining their purpose or any special considerations.

## Multiple Test Files

Currently, Catalyst supports a single test file (`.catalyst/tests.toml`). If you need to organize your tests into multiple files, you might consider using symbolic links or a build process that combines multiple TOML files into a single test file.

## Next Steps

For information on security-related configuration, see:

- [Security](./security.md) - Details on authentication and security configuration
