# Security Configuration

This section explains how to configure authentication and security-related aspects when testing APIs with Catalyst.

## Authentication Methods

Catalyst supports several authentication methods that can be configured in the global configuration section or per test.

### Cookie-based Authentication

For cookie-based authentication, you can use the `auth_method` set to "Cookie":

```toml
[config]
base_url = "https://api.example.com"
auth_method = "Cookie"
default_headers = { "Content-Type" = "application/json" }
```

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

To use Bearer token authentication:

```toml
[config]
base_url = "https://api.example.com"
auth_method = "Bearer"
auth_token = "your-jwt-token"
```

This will add an `Authorization: Bearer your-jwt-token` header to all requests.

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

- [Test File Structure](./test_file_structure.md) - Details on the overall structure of test files
- [Complete Reference](../reference/references.md) - Comprehensive reference with examples for all features, including advanced assertions and response time validation
