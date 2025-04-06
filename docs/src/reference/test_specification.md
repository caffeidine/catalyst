# Test Reference

This page serves as a technical reference for creating tests with Catalyst.

## Basic Structure

```toml
[config]
base_url = "https://api.example.com"
default_headers = { "Content-Type" = "application/json" }

[[tests]]
name = "Test Name"
method = "GET"
endpoint = "/path"
expected_status = 200
```

## Global Configuration Options

| Option            | Description                        | Required | Example                                           |
| ----------------- | ---------------------------------- | -------- | ------------------------------------------------- |
| `base_url`        | Base URL for all API requests      | Yes      | `"https://api.example.com"`                       |
| `default_headers` | Headers to include in all requests | No       | `{ "Content-Type" = "application/json" }`         |
| `auth_method`     | Authentication method              | No       | `"Bearer"` (options: "Bearer", "Basic", "Cookie") |
| `auth_token`      | Authentication token               | No       | `"your-token-here"`                               |

```toml
[config]
base_url = "https://api.example.com"
default_headers = {
  "Content-Type" = "application/json",
  "Accept" = "application/json"
}
auth_method = "Bearer"
auth_token = "your-token-here"
```

## Test Definition Options

| Option              | Description                                   | Required | Example                                                |
| ------------------- | --------------------------------------------- | -------- | ------------------------------------------------------ |
| `name`              | Name of the test                              | Yes      | `"Get User Profile"`                                   |
| `method`            | HTTP method                                   | Yes      | `"GET"` (options: GET, POST, PUT, DELETE, PATCH, etc.) |
| `endpoint`          | API endpoint (appended to base_url)           | Yes      | `"/users/1"`                                           |
| `query_params`      | Query parameters for the URL                  | No       | `{ "page" = "1", "limit" = "10" }`                     |
| `headers`           | Headers specific to this test                 | No       | `{ "X-Custom-Header" = "value" }`                      |
| `body`              | Request body (for POST, PUT, etc.)            | No       | `{ "name" = "John Doe" }`                              |
| `expected_status`   | Expected HTTP status code                     | Yes      | `200`                                                  |
| `expected_body`     | Expected response body (exact match)          | No       | `{ "success" = true }`                                 |
| `assertions`        | Advanced assertions for response validation   | No       | See Assertions section                                 |
| `expected_headers`  | Expected response headers                     | No       | `[["Content-Type", "application/json"]]`               |
| `store`             | JSON paths to extract and store as variables  | No       | `{ "$.token" = "auth_token" }`                         |
| `get_cookie`        | Cookies to extract and store as variables     | No       | `{ "session" = "session_id" }`                         |
| `max_response_time` | Maximum allowed response time in milliseconds | No       | `500`                                                  |

```toml
[[tests]]
name = "Create User"
method = "POST"
endpoint = "/users"
query_params = { "source" = "api" }
headers = { "X-Custom-Header" = "value" }
body = {
  "name" = "John Doe",
  "email" = "john@example.com",
  "roles" = ["user", "admin"]
}
expected_status = 201
expected_body = { "success" = true }
expected_headers = [["Content-Type", "application/json"]]
max_response_time = 500
```

## Response Validation

### Using expected_body vs. assertions

| Validation Method | Description                               | Use Case                                                        | Limitations                                         |
| ----------------- | ----------------------------------------- | --------------------------------------------------------------- | --------------------------------------------------- |
| `expected_body`   | Exact match validation                    | When you need to validate the entire response structure exactly | Cannot perform partial validations or regex matches |
| `assertions`      | Advanced validation with multiple methods | When you need more flexible validation options                  | Requires more configuration                         |

#### expected_body Example

```toml
# Using expected_body for exact match validation
[[tests]]
name = "Get User"
method = "GET"
endpoint = "/users/1"
expected_status = 200
expected_body = {
  "id" = 1,
  "name" = "John Doe",
  "email" = "john@example.com",
  "created_at" = "2023-01-01T00:00:00Z"
}
```

#### assertions Example

```toml
# Using assertions for more flexible validation
[[tests]]
name = "Get User with Assertions"
method = "GET"
endpoint = "/users/1"
expected_status = 200
assertions = [
  # Only validate specific fields we care about
  { type = "Contains", value = {
    "id" = 1,
    "name" = "John Doe"
  }},
  # Validate email format with regex
  { type = "PathRegex", value = ["$.email", "^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$"] }
]
```

## Assertion Types

| Type        | Description                                                    | Example                                                           | When to Use                                                       |
| ----------- | -------------------------------------------------------------- | ----------------------------------------------------------------- | ----------------------------------------------------------------- |
| `Exact`     | Validates that the response exactly matches the expected value | `{ type = "Exact", value = { "id" = 1 } }`                        | When you need to validate the entire response structure           |
| `Contains`  | Validates that the response contains all specified fields      | `{ type = "Contains", value = { "success" = true } }`             | When you only care about specific fields, not the entire response |
| `Regex`     | Validates the response against a regex pattern                 | `{ type = "Regex", value = ".*\"id\":\\s*1.*" }`                  | When you need to validate the entire response with a pattern      |
| `PathRegex` | Validates a specific JSON path against a regex pattern         | `{ type = "PathRegex", value = ["$.email", ".*@example\\.com"] }` | When you need to validate a specific field with a pattern         |

### Assertion Examples

```toml
[[tests]]
name = "Advanced Assertions Example"
method = "GET"
endpoint = "/users/1"
expected_status = 200

assertions = [
  # Example 1: Exact match (equivalent to expected_body)
  { type = "Exact", value = {
    "id" = 1,
    "name" = "John Doe",
    "email" = "john@example.com"
  }},

  # Example 2: Contains match (partial validation)
  { type = "Contains", value = {
    "id" = 1,
    "roles" = ["user"]
  }},

  # Example 3: Regex match on entire response
  { type = "Regex", value = ".*\"email\":\\s*\"john@example.com\".*" },

  # Example 4: PathRegex match on specific field
  { type = "PathRegex", value = ["$.email", ".*@example\\.com"] },

  # Example 5: PathRegex for numeric validation
  { type = "PathRegex", value = ["$.id", "^[0-9]+$"] }
]
```

## Variable Storage and Usage

### Storing Variables

```toml
[[tests]]
name = "Extract and Store Values"
method = "POST"
endpoint = "/auth/login"
body = { "username" = "user", "password" = "pass" }
expected_status = 200

# Extract and store values from JSON body
store = {
  "$.token" = "auth_token",        # Stores value at $.token in auth_token
  "$.user.id" = "user_id",         # Stores value at $.user.id in user_id
  "$.expires_at" = "token_expiry"  # Stores value at $.expires_at in token_expiry
}

# Extract and store cookies
get_cookie = {
  "session" = "session_id",        # Stores session cookie value in session_id
  "XSRF-TOKEN" = "csrf_token"      # Stores XSRF-TOKEN cookie value in csrf_token
}
```

### Using Stored Variables

```toml
[[tests]]
name = "Use Stored Variables"
method = "GET"
endpoint = "/users/{{user_id}}"  # Uses user_id variable in URL
headers = {
  "Authorization" = "Bearer {{auth_token}}",  # Uses auth_token in header
  "X-CSRF-Token" = "{{csrf_token}}"           # Uses csrf_token in header
}
expected_status = 200

# Variables can be used in any part of the test
body = {
  "token" = "{{auth_token}}",
  "session" = "{{session_id}}"
}
```

## Response Time Validation

Catalyst automatically measures the response time for each test and makes it available as a variable.

```toml
[[tests]]
name = "Response Time Validation"
method = "GET"
endpoint = "/fast-endpoint"
expected_status = 200
max_response_time = 100  # Response must be received in less than 100ms

# The response_time_ms variable is automatically created after each test
# and can be used in subsequent tests
[[tests]]
name = "Log Response Time"
method = "POST"
endpoint = "/metrics/log"
body = { "previous_response_time" = "{{response_time_ms}}" }
expected_status = 200
```

## Test Chaining Example

```toml
# Test 1: Create a resource
[[tests]]
name = "Create Resource"
method = "POST"
endpoint = "/resources"
body = { "name" = "New Resource" }
expected_status = 201
store = { "$.id" = "resource_id" }  # Store ID for next test

# Test 2: Get the created resource
[[tests]]
name = "Get Created Resource"
method = "GET"
endpoint = "/resources/{{resource_id}}"  # Use stored ID
expected_status = 200
assertions = [
  { type = "Contains", value = { "name" = "New Resource" } }
]

# Test 3: Update the resource
[[tests]]
name = "Update Resource"
method = "PUT"
endpoint = "/resources/{{resource_id}}"
body = { "name" = "Updated Resource" }
expected_status = 200

# Test 4: Verify the update
[[tests]]
name = "Verify Update"
method = "GET"
endpoint = "/resources/{{resource_id}}"
expected_status = 200
assertions = [
  { type = "Contains", value = { "name" = "Updated Resource" } }
]

# Test 5: Delete the resource
[[tests]]
name = "Delete Resource"
method = "DELETE"
endpoint = "/resources/{{resource_id}}"
expected_status = 204
```
