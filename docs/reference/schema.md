# Schema Reference

Exhaustive specification of the test file format.

## Global `[config]`

| Key               | Type                       | Required | Notes |
|-------------------|----------------------------|----------|-------|
| `base_url`        | string                      | yes      | Base URL for all requests |
| `default_headers` | map&lt;string,string&gt;          | no       | Added to every request |
| `insecure`        | bool                        | no       | Accept invalid TLS certs |
| `allowed_commands`| array&lt;string&gt;               | no       | Whitelist for command hooks |

Environment variables can be interpolated in any string via <code v-pre>${{NAME}}</code>.

```toml
[config]
base_url = "https://api.example.com"
default_headers = { "Content-Type" = "application/json" }
insecure = false
allowed_commands = ["bash", "sh", "echo", "jq"]
```

## `[[tests]]` entries

| Key                 | Type                      | Required | Notes |
|---------------------|---------------------------|----------|-------|
| `name`              | string                    | yes      | Display name |
| `method`            | string                    | yes      | GET, POST, PUT, DELETE, PATCH, HEAD, OPTIONS |
| `endpoint`          | string                    | yes      | Appended to `base_url`; supports <code v-pre>{{vars}}</code> |
| `query_params`      | map&lt;string,string&gt;        | no       | Each value supports <code v-pre>{{vars}}</code> and <code v-pre>${{ENV}}</code> |
| `headers`           | map&lt;string,string&gt;        | no       | Per-test headers |
| `body`              | JSON value                | no       | Inline JSON; strings support <code v-pre>{{vars}}</code> and <code v-pre>{{file:path}}</code> |
| `body_file`         | string (relative path)    | no       | Load body from file; `.json` parsed as JSON |
| `expected_status`   | number                    | yes      | HTTP status code |
| `expected_body`     | JSON value                | no       | Exact match; use `assertions` for flexible checks |
| `assertions`        | `array&lt;Assertion&gt;`          | no       | See Assertions Reference |
| `store`             | map&lt;string,string&gt;        | no       | JSONPath (like `$.id`) → variable name |
| `get_cookie`        | map&lt;string,string&gt;        | no       | Cookie name → variable name |
| `max_response_time` | number (ms)               | no       | Fails if exceeded |
| `before`            | `array&lt;CommandStep&gt;`        | no       | Run before HTTP call |
| `after`             | `array&lt;CommandStep&gt;`        | no       | Run after; supports `on` condition |

Mutual exclusivity: `body` and `body_file` cannot be used together.

### CommandStep

| Key          | Type                | Required | Notes |
|--------------|---------------------|----------|-------|
| `run`        | string              | yes      | Command binary or script |
| `args`       | array&lt;string&gt;       | no       | Arguments array |
| `shell`      | bool                | no       | Run via shell (`sh -lc`) |
| `dir`        | string              | no       | Working directory |
| `env`        | map&lt;string,string&gt;  | no       | Step environment variables |
| `timeout_ms` | number              | no       | Default 30000 |
| `ignore_error`| bool               | no       | Don’t fail on non-zero exit |
| `capture`    | `{ var: string }`   | no       | Save stdout→`{{var}}`, stderr→`{{var}}_stderr` |
| `export`     | map&lt;string,string&gt;  | no       | JSONPath from stdout → variable |
| `when`       | string              | no       | Simple `==`/`!=` condition after substitution |
| `on`         | string              | no       | Only for `after`: `success`/`failure`/`always` |

Security: every step is validated against `[config].allowed_commands`.
