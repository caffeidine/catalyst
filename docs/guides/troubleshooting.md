# Troubleshooting

Common issues and fixes.

## No tests found
- Ensure `.catalyst/tests.toml` exists or pass `--file`.
- Run from project root.

## Variables not substituted
- Use `--var key=value,token=$TOKEN`.
- Use `{{variable}}` in strings; env uses `${{NAME}}`.
- Env precedence: `.env.local` > `.env.dev` > `.env`.

## Auth (401/403)
- Verify header names and values.
- Extract cookies/tokens before using.

## Flaky response time checks
- Raise `max_response_time` for CI variance.
- Isolate perf-sensitive tests.

More: ../getting-started/running_tests.md
