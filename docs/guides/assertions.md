# Assertions

Validate responses with flexible methods.

- Exact: full JSON equality
- Contains: subset match
- Regex: match whole response
- PathRegex: match a JSON path value

Example
```toml
[[tests.assertions]]
type = "Contains"
value = { id = 1 }

[[tests.assertions]]
type = "PathRegex"
value = ["$.email", "^[^@]+@example\\.com$"]
```

Reference: ../reference/assertions.md
