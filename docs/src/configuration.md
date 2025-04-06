# Configuration

Catalyst uses a TOML configuration file to define your API tests. This section explains the structure and options available in the configuration file.

## Configuration File Location

By default, Catalyst looks for a configuration file at `.catalyst/tests.toml` in your project directory.

## Configuration Structure

The configuration file has two main sections:

1. **Global Configuration** - Settings that apply to all tests
2. **Test Definitions** - Individual test cases

## Global Configuration

The global configuration is defined in a `[config]` section at the top of the file:

```toml
[config]
base_url = "https://api.example.com"
auth_method = "Bearer"  # Optional
auth_token = "your-token"  # Optional
default_headers = { "Content-Type" = "application/json" }  # Optional
```

### Available Options

| Option            | Description                                         | Required |
| ----------------- | --------------------------------------------------- | -------- |
| `base_url`        | Base URL for all API requests                       | Yes      |
| `auth_method`     | Authentication method ("Bearer", "Basic", "Cookie") | No       |
| `auth_token`      | Authentication token (used with auth_method)        | No       |
| `default_headers` | Headers to include in all requests                  | No       |

## Test Definitions

Test definitions are specified using `[[tests]]` sections:

```toml
[[tests]]
name = "Get Users"
method = "GET"
endpoint = "/users"
query_params = { "limit" = "10" }  # Optional
headers = { "X-Custom-Header" = "value" }  # Optional
body = { "key" = "value" }  # Optional
expected_status = 200
expected_body = { "success" = true }  # Optional
expected_headers = [["Content-Type", "application/json"]]  # Optional
store = { "$.token" = "auth_token" }  # Optional
get_cookie = { "session" = "session_cookie" }  # Optional
```

### Available Test Options

| Option             | Description                                  | Required |
| ------------------ | -------------------------------------------- | -------- |
| `name`             | Name of the test                             | Yes      |
| `method`           | HTTP method (GET, POST, PUT, DELETE, etc.)   | Yes      |
| `endpoint`         | API endpoint (will be appended to base_url)  | Yes      |
| `query_params`     | Query parameters to include in the URL       | No       |
| `headers`          | Headers specific to this test                | No       |
| `body`             | Request body (for POST, PUT, etc.)           | No       |
| `expected_status`  | Expected HTTP status code                    | Yes      |
| `expected_body`    | Expected response body (partial match)       | No       |
| `expected_headers` | Expected response headers                    | No       |
| `store`            | JSON paths to extract and store as variables | No       |
| `get_cookie`       | Cookies to extract and store as variables    | No       |

## Next Steps

For more detailed information about specific aspects of configuration, see:

- [Test File Structure](./configuration/test_file_structure.md)
- [Security](./configuration/security.md)
