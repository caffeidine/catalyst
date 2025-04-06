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
max_response_time = 500  # Maximum response time in milliseconds (Optional)

# Advanced assertions (Optional)
assertions = [
  # Exact match (same as expected_body)
  { type = "Exact", value = { "success" = true } },

  # Partial match - checks if response contains these fields
  { type = "Contains", value = { "data" = { "users" = [] } } },

  # Regex match on the entire response body
  { type = "Regex", value = "\\{.*\"success\"\\s*:\\s*true.*\\}" },

  # Regex match on a specific JSON path
  { type = "PathRegex", value = ["$.user.email", ".*@example\\.com"] }
]
```

### Available Test Options

| Option              | Description                                   | Required |
| ------------------- | --------------------------------------------- | -------- |
| `name`              | Name of the test                              | Yes      |
| `method`            | HTTP method (GET, POST, PUT, DELETE, etc.)    | Yes      |
| `endpoint`          | API endpoint (will be appended to base_url)   | Yes      |
| `query_params`      | Query parameters to include in the URL        | No       |
| `headers`           | Headers specific to this test                 | No       |
| `body`              | Request body (for POST, PUT, etc.)            | No       |
| `expected_status`   | Expected HTTP status code                     | Yes      |
| `expected_body`     | Expected response body (exact match)          | No       |
| `assertions`        | Advanced assertions for response validation   | No       |
| `expected_headers`  | Expected response headers                     | No       |
| `store`             | JSON paths to extract and store as variables  | No       |
| `get_cookie`        | Cookies to extract and store as variables     | No       |
| `max_response_time` | Maximum allowed response time in milliseconds | No       |

## Advanced Assertions

Catalyst 0.2.1 introduces advanced assertions for more flexible response validation. You can use the `assertions` field to define multiple validation rules for a single test.

### Types of Assertions

#### Exact Match

Validates that the response body exactly matches the expected value. This is equivalent to using `expected_body`.

```toml
assertions = [
  { type = "Exact", value = { "success" = true, "data" = { "id" = 1 } } }
]
```

#### Contains Match

Validates that the response body contains all the fields specified in the expected value, but may contain additional fields.

```toml
assertions = [
  { type = "Contains", value = { "success" = true } }
]
```

This will pass if the response contains `{"success": true, "data": {...}}` or any other JSON that includes the `success` field with a value of `true`.

#### Regex Match

Validates that the string representation of the response body matches the specified regular expression pattern.

```toml
assertions = [
  { type = "Regex", value = "\\{.*\"success\"\\s*:\\s*true.*\\}" }
]
```

#### Path Regex Match

Validates that a specific value in the response body, identified by a JSON path, matches the specified regular expression pattern.

```toml
assertions = [
  { type = "PathRegex", value = ["$.user.email", ".*@example\\.com"] }
]
```

This will pass if the value at `$.user.email` in the response matches the pattern `.*@example\.com`.

### Combining Assertions

You can combine multiple assertions for a single test:

```toml
assertions = [
  { type = "Contains", value = { "success" = true } },
  { type = "PathRegex", value = ["$.data.id", "\\d+"] }
]
```

This test will pass only if both assertions are satisfied.

## Response Time Validation

Catalyst 0.2.1 introduces response time validation and tracking. This feature allows you to:

1. Set maximum allowed response times for your API endpoints
2. Access and use the measured response times in subsequent tests

### Setting Maximum Response Time

You can use the `max_response_time` field to specify the maximum allowed response time in milliseconds:

```toml
[[tests]]
name = "Fast API Response"
method = "GET"
endpoint = "/api/fast-endpoint"
expected_status = 200
max_response_time = 100  # Must respond within 100ms
```

If the API response takes longer than the specified time, the test will fail with a message indicating that the response time exceeded the maximum allowed time.

### Automatic Response Time Tracking

**Important:** After each test execution, Catalyst automatically measures and stores the response time. This value is stored in a special variable named `response_time_ms` that becomes available to all subsequent tests.

This happens automatically for every test, whether or not you've specified a `max_response_time` value.

### Using the Response Time Variable

You can reference the stored response time using the standard variable syntax `{{response_time_ms}}` in any subsequent test:

```toml
# First test - response time will be measured
[[tests]]
name = "Get User Profile"
method = "GET"
endpoint = "/users/profile"
expected_status = 200

# Second test - uses the response time from the first test
[[tests]]
name = "Log Response Time"
method = "POST"
endpoint = "/metrics/log"
body = {
  "endpoint" = "/users/profile",
  "response_time_ms" = "{{response_time_ms}}",
  "timestamp" = "{{current_timestamp}}"
}
expected_status = 200
```

This feature is particularly useful for:

- Performance logging and monitoring
- Creating tests that validate performance metrics
- Debugging performance issues across different API endpoints

## Next Steps

For more detailed information about specific aspects of configuration, see:

- [Test File Structure](./configuration/test_file_structure.md)
- [Security](./configuration/security.md)
- [Complete Reference](./reference/test_specification.md) - Comprehensive reference with examples for all features
