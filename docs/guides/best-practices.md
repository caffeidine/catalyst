# Best Practices

Guidelines to keep suites fast, clear, and maintainable.

- Keep tests focused: one intent per test.
- Prefer `assertions` with Contains over exact bodies to reduce brittleness.
- Store only values you need; name variables clearly.
- Use `body_file` for large payloads; keep fixtures small and reusable.
- Centralize auth and headers in `[config].default_headers`.
- Add hooks sparingly; whitelist only necessary commands.
- Use `--filter` locally for fast feedback and `--debug` when stuck.
- Record performance budgets with `max_response_time` for critical endpoints.
- Separate suites per environment with dedicated test files.
