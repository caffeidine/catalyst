# Assertions Reference

Flexible response validation options.

## Methods

- `Exact(value)`: response must equal the JSON value
- `Contains(value)`: response must contain the JSON subset
- `Regex(pattern)`: full response (as string) matches regex
- `PathRegex(path, pattern)`: value at JSON path matches regex

## Examples

```toml
[[tests]]
name = "Validate user"
method = "GET"
endpoint = "/users/1"
expected_status = 200

[[tests.assertions]]
type = "Contains"
value = { id = 1, name = "John" }

[[tests.assertions]]
type = "PathRegex"
value = ["$.email", "^[^@]+@example\\.com$"]

[[tests.assertions]]
type = "Regex"
value = ".*\"role\":\s*\"admin\".*"
```

Tip: Use variables in values and patterns, e.g. `"^{{user_id}}$"`.
