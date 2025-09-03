# Project Layout

Keep tests close to your codebase while remaining portable and CI‑ready.

Recommended structure
```
your-project/
├── .catalyst/
│   ├── tests.toml            # default test suite
│   └── data/                 # request bodies and fixtures
│       ├── create-user.json
│       └── payloads/
├── .env.local                # local env vars (optional)
└── ...
```

Conventions
- Default file: `.catalyst/tests.toml` (override with `--file`).
- Relative file paths resolve from the test file directory.
- Use `data/` for request bodies; prefer `body_file` for large payloads.
- Keep secrets in env vars; reference with `${{NAME}}`.

Multiple suites
- Create additional files: `.catalyst/staging.toml`, `.catalyst/perf.toml`.
- Select with `--file .catalyst/staging.toml`.

