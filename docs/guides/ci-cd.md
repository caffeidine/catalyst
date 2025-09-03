# CI/CD Integration

Run Catalyst in pipelines to catch regressions.

## GitHub Actions
```yaml
name: Catalyst Tests
on: [push, pull_request]
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Install Catalyst
        run: cargo install catalyst --locked
      - name: Run API tests
        run: catalyst run --file .catalyst/tests.toml --verbose
```

## Variables and secrets
Pass secrets as env or via `--var` and reference in tests.

See also: ../reference/cli.md
