# Security Configuration

This section explains how to configure authentication and security-related aspects when testing APIs with Catalyst.

## Authentication Methods

Catalyst supports several authentication methods that can be configured in the global configuration section or per test.

### Cookie-based Authentication

The typical workflow for cookie-based authentication is:

1. Perform a login request
2. Extract the session cookie
3. Use the cookie in subsequent requests

Example:

```toml
[[tests]]
name = "Login"
method = "POST"
endpoint = "/auth/login"
body = { "username" = "test", "password" = "password" }
expected_status = 200
get_cookie = { "session_id" = "session_cookie" }

[[tests]]
name = "Access Protected Resource"
method = "GET"
endpoint = "/protected"
headers = { "Cookie" = "session_id={{session_cookie}}" }
expected_status = 200
```

### Bearer Token Authentication

Add an Authorization header via `default_headers` (global) or per-test `headers`:

```toml
[config]
base_url = "https://api.example.com"
default_headers = { 
  "Content-Type" = "application/json",
  "Authorization" = "Bearer {{API_TOKEN}}" 
}
```

You can pass `API_TOKEN` via `--var API_TOKEN=...` or use an environment variable with `${{API_TOKEN}}` in values.

## Handling API Keys

You can include API keys in headers or query parameters:

```toml
[[tests]]
name = "API Key in Header"
method = "GET"
endpoint = "/protected"
headers = { "X-API-Key" = "your-api-key" }
expected_status = 200

[[tests]]
name = "API Key in Query Parameter"
method = "GET"
endpoint = "/protected"
query_params = { "api_key" = "your-api-key" }
expected_status = 200
```

## Chaining Authentication

You can chain authentication by extracting tokens from responses:

```toml
[[tests]]
name = "Create User Token"
method = "POST"
endpoint = "/user/tokens"
headers = { "Cookie" = "{{session_cookie}}" }
expected_status = 200
store = { "data.token" = "token" }

[[tests]]
name = "Access Protected Resource"
method = "GET"
endpoint = "/protected"
headers = { "x-api-token" = "{{token}}" }
expected_status = 200
```

## Next Steps

For more information about test configuration and advanced features, see:

- [Schema Reference](../reference/schema.md)
- [Test Reference Index](../reference/references.md)
