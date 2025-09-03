# Variables & Chaining

Chain tests by extracting values and reusing them.

## Sources
- CLI: `--var key=value`
- Store: `store = { "$.path" = "var" }`
- Cookies: `get_cookie = { "session" = "session_id" }`
- Headers: auto `header_<name>` variables (lowercased)
- Environment: `${{NAME}}` in strings

## Example
```toml
[[tests]]
name = "Login"
method = "POST"
endpoint = "/auth/login"
body = { username = "demo", password = "secret" }
expected_status = 200
store = { "$.user.id" = "user_id" }
get_cookie = { session = "session_cookie" }

[[tests]]
name = "Get profile"
method = "GET"
endpoint = "/users/{{user_id}}"
headers = { Cookie = "session={{session_cookie}}" }
expected_status = 200
```

Tips
- Use `--debug` to see substitutions at runtime.
- Use `$.field[0].id` for simple array access in JSON paths.
