# Test Reference

Authoritative, structured reference for Catalyst features. For step-by-step guides, see Tutorials and How‑To Guides.

Reference sections
- Schema: ./schema.md
- CLI: ./cli.md
- Assertions: ./assertions.md
- Variables: ./variables.md
- Request Bodies: ./file-bodies.md
- Performance: ./performance.md

Quick examples

Basic test structure:

```toml
[config]
base_url = "https://api.example.com"

[[tests]]
name = "Get user"
method = "GET"
endpoint = "/users/1"
expected_status = 200
```

Use variables and assertions:

```toml
[[tests]]
name = "Verify Email"
method = "GET"
endpoint = "/users/{{user_id}}"
expected_status = 200

[tests.headers]
Authorization = "Bearer ${{API_TOKEN}}"

[[tests.assertions]]
type = "PathRegex"
value = ["$.email", "^[^@]+@example\\.com$"]
```

