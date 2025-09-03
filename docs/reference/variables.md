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