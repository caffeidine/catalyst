# CLI Reference

Use `catalyst help` or `catalyst <command> --help` to see flags.

## Commands

- `run`: Execute tests
  - `-f, --filter <FILTER>`: Run tests with names containing this string
  - `--file <FILE>`: Use a specific test file (defaults to `.catalyst/tests.toml`)
  - `--var <VAR>`: Set variables as `key=value` pairs (comma-separated)
  - `-v, --verbose`: Show detailed output
  - `--disable-color`: Disable colored output
  - `-d, --debug`: Print debug logs (variable substitution, hooks, env)

- `validate`: Validate tests configuration
  - `--file <FILE>`: Use a specific test file
  - `--var <VAR>`: Set variables as `key=value` pairs (comma-separated)

- `list`: List available tests
  - `-v, --verbose`: Show detailed information
  - `--file <FILE>`: Use a specific test file

## Examples

```bash
catalyst run --verbose
catalyst run --file .catalyst/staging.toml
catalyst run --filter "Login" --var token=$TOKEN,base_url=$BASE
catalyst validate --file .catalyst/tests.toml
catalyst list -v
```

See also: [Run and Filter Tests](../getting-started/running_tests.md)
