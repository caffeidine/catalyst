# Request Bodies

Two options: inline `body` or `body_file`.

## Inline
```toml
body = { name = "{{username}}", content = "{{file:data/doc.txt}}" }
```

## File-based
```toml
body_file = "data/create-user.json"
```

Notes
- Paths are relative to the test file directory.
- `.json` files are parsed; others are sent as text.
- `body` and `body_file` are mutually exclusive.

See also: [File Bodies Reference](../reference/file-bodies)