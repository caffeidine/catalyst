# Core Concepts

Understand the mental model behind Catalyst.

## Declarative tests (TOML)
Tests are defined in TOML, not code. This makes suites readable, reviewable, and easy to share.

## Configuration and defaults
`[config]` sets `base_url`, `default_headers`, and other defaults inherited by tests.

## Variables and chaining
Extract values from responses and cookies to chain requests. Use `store` to capture values and reference them with `{{name}}` later.

## Assertions
Validate responses via:
- `expected_status`: Required status
- `expected_body`: Exact body match
- `assertions`: Flexible checks: `contains`, `regex`, `path_regex`

## Performance checks
`max_response_time` enforces an upper bound (ms). Catalyst also stores `response_time_ms` for subsequent tests.

## Files and bodies
Inline `body` for small payloads; use `body_file` for larger JSON files. Variables can be used in both.

Learn details in: [Schema Reference](../reference/schema.md) and [Test Reference](../reference/references.md).
