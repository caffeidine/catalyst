# Writing Tests

This guide shows how to author clear, maintainable tests step by step.

1) Minimal test
```toml
[config]
base_url = "https://api.example.com"

[[tests]]
name = "Get user"
method = "GET"
endpoint = "/users/1"
expected_status = 200
```

2) Add assertions
```toml
[[tests.assertions]]
type = "Contains"
value = { id = 1, active = true }
```

3) Use variables
```toml
[tests.headers]
Authorization = "Bearer ${{API_TOKEN}}"
```

4) Split request bodies into files
```toml
[[tests]]
name = "Create user"
method = "POST"
endpoint = "/users"
body_file = "data/create-user.json"
expected_status = 201
```

Next
- Variables & Chaining → ./variables-chaining.md
- Assertions → ../reference/assertions.md
- Request Bodies → ../reference/file-bodies.md
- Hooks → ./hooks.md
