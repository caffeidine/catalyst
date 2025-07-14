# Running Tests

Once you've created your test configuration file, you can run your tests using the Catalyst command-line interface. This guide explains how to run tests and interpret the results.

## Basic Test Execution

To run all tests defined in your `.catalyst/tests.toml` file, navigate to your project directory and run:

```bash
catalyst run
```

This will execute all tests in the order they are defined in the file.

## CLI Options

Catalyst provides several command-line options to customize test execution:

### Specifying a Custom Test File

By default, Catalyst looks for tests in `.catalyst/tests.toml` in your current directory. You can specify a different file using the `--file` option:

```bash
catalyst run --file /path/to/custom/tests.toml
```

### Filtering Tests

You can run specific tests by using the `--filter` option:

```bash
catalyst run --filter "Login"
```

This will only run tests whose names contain the string "Login".

### Verbose Output

For more detailed output, use the `--verbose` (or `-v`) flag:

```bash
catalyst run --verbose
```

This will show additional information such as response bodies and headers.

### Disabling Colored Output

If you're running tests in an environment that doesn't support colored output, you can disable it:

```bash
catalyst run --disable-color
```

### Setting Variables from CLI

You can pass variables directly from the command line using the `--var` option. This is useful for setting environment-specific values or dynamic test data:

```bash
catalyst run --var user_id=123,api_token=secret_key
```

Variables are specified in `key=value` format and multiple variables are separated by commas. These variables can then be used in your test files using the `{{variable}}` syntax:

```toml
[[tests]]
name = "Get User"
method = "GET"
endpoint = "/users/{{user_id}}"
headers = { "Authorization" = "Bearer {{api_token}}" }
expected_status = 200
```

CLI variables work with both complex values and special characters:

```bash
catalyst run --var base_url=https://api.example.com,timeout=30,debug=true
```

### Complete CLI Reference

Here's a complete list of available commands and options:

```
CATALYST COMMANDS:
  run       Run API tests
    Options:
      -f, --filter <FILTER>    Filter by test name
      --disable-color          Disable colored output
      -v, --verbose            Enable verbose output
      --file <FILE>            Specify a custom test file path
      --var <VAR>              Set variables in key=value format (comma-separated)

  validate  Validate tests configuration
    Options:
      --file <FILE>            Specify a custom test file path
      --var <VAR>              Set variables in key=value format (comma-separated)

  list      List available tests
    Options:
      -v, --verbose            Enable detailed test information
      --file <FILE>            Specify a custom test file path

  help      Print this message or the help of the given subcommand(s)
```

## Understanding Test Results

Catalyst provides clear feedback about test execution:

- **[PASS]** - The test succeeded (actual status code matches expected status code)
- **[FAIL]** - The test failed (actual status code differs from expected status code)

At the end of the test run, Catalyst will display a summary showing the total number of tests, how many passed, and how many failed.

## Example Output

```
Running API tests...
[PASS] Simple GET Request      (200 Success)
[PASS] Create User             (201 Success)
[FAIL] Update User             (404 Not Found) (expected 200)

Failed tests:
- Update User

Test Summary:
Total: 3, Passed: 2, Failed: 1
```

## Next Steps

Now that you know how to run tests, you can explore more configuration options in the [Configuration](../configuration.md) section.
