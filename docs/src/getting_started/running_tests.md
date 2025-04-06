# Running Tests

Once you've created your test configuration file, you can run your tests using the Catalyst command-line interface. This guide explains how to run tests and interpret the results.

## Basic Test Execution

To run all tests defined in your `.catalyst/tests.toml` file, navigate to your project directory and run:

```bash
catalyst run
```

This will execute all tests in the order they are defined in the file.

## Filtering Tests

You can run specific tests by using the `--filter` option:

```bash
catalyst run --filter "Login"
```

This will only run tests whose names contain the string "Login".

## Verbose Output

For more detailed output, use the `--verbose` (or `-v`) flag:

```bash
catalyst run --verbose
```

This will show additional information such as response bodies and headers.

## Disabling Colored Output

If you're running tests in an environment that doesn't support colored output, you can disable it:

```bash
catalyst run --disable-color
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
