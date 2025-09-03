# Request Bodies Reference

Options for defining request payloads.

## Inline `body`

Use JSON directly in the test. Strings support variables and file inclusion.

```toml
body = { "name" = "{{username}}", "content" = "{{file:data/doc.txt}}" }
```

## `body_file`

Load from a relative file path. `.json` files are parsed as JSON; others are sent as text.

```toml
body_file = "data/create-user.json"
```

Security
- Paths are resolved relative to the test file directory
- No `..` traversal or absolute paths

Note: `body` and `body_file` are mutually exclusive.