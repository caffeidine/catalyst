# Variables Reference

Catalyst substitutes variables in any string using <code v-pre>{{name}}</code>. Environment variables use <code v-pre>${{NAME}}</code>.

## Sources

- CLI: `--var key=value,token=$TOKEN`
- Stored values: `store = { "$.path" = "var" }`
- Cookies: `get_cookie = { "cookieName" = "var" }`
- Environment: <code v-pre>${{ENV_NAME}}</code> in strings
- Automatic:
  - `response_time_ms`: set after each test
  - `header_<name>`: response headers (lowercased), e.g. <code v-pre>{{header_content-type}}</code>

## Usage

<div v-pre>

```toml
endpoint = "/users/{{user_id}}"
headers = { "Authorization" = "Bearer {{token}}" }
body = { "content" = "{{file:data/payload.txt}}" }
```

</div>

## Type Inference

When a placeholder is the entire JSON value, Catalyst inspects the resolved text and automatically parses native types:

<div v-pre>

```toml
body = { user_ids = ["{{admin_user_id}}"], active = "{{is_active}}" }
```

</div>

If `admin_user_id = "42"` and `is_active = "false"`, the request body becomes:

```json
{ "user_ids": [42], "active": false }
```

Lists, objects, numbers, booleans, and `null` all round-trip as long as the stored value is valid JSON.

## Explicit Casts

Add a suffix with `|<type>` to force a specific conversion. Casts work for both CLI variables and environment variables.

<div v-pre>

```toml
body = {
  user_ids = ["{{admin_user_id|int}}"],
  flags = "{{feature_flags|json}}",
  comment = "Mention {{admin_user_id|string}} user"
}
```

</div>

Supported casts:

- `|int`, `|integer`
- `|float`, `|double`, `|number`
- `|bool`, `|boolean`
- `|string`, `|str`
- `|json` (parses any JSON value)
- `|array`, `|list`
- `|object`, `|map`
- `|null`

If parsing fails, Catalyst falls back to the interpolated string so tests keep running.