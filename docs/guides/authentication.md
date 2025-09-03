# Authentication

Common auth patterns with headers, cookies, and tokens.

## Bearer tokens
Add Authorization via headers (global or per-test):
```toml
[config]
base_url = "https://api.example.com"
default_headers = { Authorization = "Bearer ${{API_TOKEN}}" }
```

## Cookie session
```toml
[[tests]]
name = "Login"
method = "POST"
endpoint = "/auth/login"
body = { username = "demo", password = "secret" }
expected_status = 200
get_cookie = { session = "session_cookie" }

[[tests]]
name = "Use session"
method = "GET"
endpoint = "/me"
headers = { Cookie = "session={{session_cookie}}" }
expected_status = 200
```

## API keys
```toml
headers = { X-API-Key = "${{API_KEY}}" }
# or
query_params = { api_key = "${{API_KEY}}" }
```

See also: ../configuration/security.md
