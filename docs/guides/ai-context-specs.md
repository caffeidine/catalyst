# AI Context Specs (for LLMs)

Help large language models (LLMs) reliably generate high‑quality Catalyst tests by giving them a consistent, compact context. This page provides what to include, a copy‑paste prompt template, and tips to improve results.

## What to include

- Base URL: the API root all endpoints use.
- Target endpoints: methods, paths, and brief intent for each test you want.
- Auth model: headers, tokens, cookies, or steps to obtain them.
- Required headers: defaults and per‑request overrides.
- Data and fixtures: example payloads or files, and how to create/delete them.
- Assertions: what must be validated (status, fields, shapes, JSONPath checks).
- Variables and chaining: values to capture and reuse between tests.
- Hooks: commands to run before/after tests (whitelist in allowed_commands).
- Performance budgets: `max_response_time` if relevant.
- Output rules: where files should go and any naming conventions.

Link the model to reference pages when possible for accuracy:
- Schema → ../reference/schema.md
- Assertions → ../reference/assertions.md
- Variables → ./variables-chaining.md
- Hooks → ./hooks.md
- Request bodies → ../reference/file-bodies.md

## Prompt template

Copy, adjust, and paste this into your AI chat. Replace ALL_CAPS placeholders.

```text
You are generating Catalyst API tests in TOML.
Follow the Schema and features from: https://caffeidine.github.io/catalyst/reference/schema

Project Context
- Base URL: BASE_URL
- Default headers (optional): DEFAULT_HEADERS_JSON
- Auth: describe how to authenticate (e.g., static bearer, cookie, OAuth step).
- Allowed commands (for hooks): ["bash", "sh", "echo", "jq"] (edit as needed)

Targets (what to test)
- Brief list of endpoints with intent. Example:
  - GET /users/{id} → fetch existing user
  - POST /users → create user
  - DELETE /users/{id} → remove user

Data & Fixtures
- Provide any static example payloads or file paths the tests can use.
- Note any setup/teardown required to seed/clean data.

Assertions & Performance
- Required checks (status codes, fields, shapes)
- Optional max_response_time (ms): e.g., 800

Variables & Chaining
- Which values to capture and reuse across tests (e.g., user id via $.id)

Output Expectations
- Produce a single TOML file named: OUTPUT_PATH (e.g., tests/users-smoke.toml)
- Only output a TOML code block, no commentary.
- Use these features as needed: `store`, `get_cookie`, `assertions`, `before`/`after`, `body_file`.

Example Capabilities Reminder
- Body can be inline JSON (`body = { ... }`) or `body_file = "path.json"`.
- Interpolate env vars: ${{NAME}}
- Extract with JSONPath: `store = { id = "$.id" }`
- Add performance budget: `max_response_time = 800`

Now generate the TOML file.
```

## Minimal example

Input prompt (to the AI):
```text
Base URL: https://api.example.com
Auth: Bearer token from env var API_TOKEN
Targets:
- POST /users → create user
- GET /users/{id} → verify created user
Assertions: status codes; `GET` body contains { id, email }
Output file: tests/users-smoke.toml
```

Expected output (from the AI):
```toml
[config]
base_url = "https://api.example.com"
default_headers = { "Content-Type" = "application/json", "Authorization" = "Bearer ${{API_TOKEN}}" }
allowed_commands = ["bash", "sh", "echo", "jq"]

[[tests]]
name = "Create user"
method = "POST"
endpoint = "/users"
body = { email = "demo@example.com", name = "Demo" }
expected_status = 201
store = { user_id = "$.id" }
max_response_time = 800

[[tests]]
name = "Get user"
method = "GET"
endpoint = "/users/{{user_id}}"
expected_status = 200

[[tests.assertions]]
type = "Contains"
value = { id = "{{user_id}}", email = "demo@example.com" }
```

## Tips for better results

- Be explicit: give concrete payloads, example responses, and field names.
- State invariants: what must always be true (status, shapes, keys, ranges).
- Guide naming: suggest test names and output file path.
- Prefer JSONPath: use `store` to capture ids or tokens for later steps.
- Keep secrets out of prompts: pass tokens via env vars and reference them.
- Use hooks sparingly: only when necessary, and whitelist in `allowed_commands`.

## Running the generated tests

- Save the AI’s TOML output to your repo, e.g., `tests/users-smoke.toml`.
- Run: `catalyst run tests/users-smoke.toml`
- Or run a whole folder: `catalyst run tests/`

Next
- Writing Tests → ./writing-tests.md
- Variables & Chaining → ./variables-chaining.md
- Assertions → ../reference/assertions.md
- Hooks → ./hooks.md
- Schema → ../reference/schema.md
