# Architecture

High-level overview for contributors and power users.

## Components

- CLI: Parses args, loads config, orchestrates execution and reporting.
- Loader: Reads TOML, merges `[config]` defaults with per-test overrides.
- Executor: Issues HTTP requests, manages cookies, tracks response times.
- Validator: Applies `expected_status`, `expected_body`, and `assertions`.
- Store: Extracts and persists variables (JSON paths, cookies) across tests.

## Data flow

1. CLI resolves test file and CLI `--var` overrides.
2. Config is parsed and validated (syntax + semantics).
3. Tests execute sequentially; variables and cookies accumulate.
4. Results aggregate into a summary; failures are reported with context.

## Extensibility

- New assertion types can be added in the Validator.
- Additional loaders (e.g., multiple files) can wrap the Loader.
- Output formats (e.g., JUnit) can extend reporting without changing tests.

See the source under `src/` for implementation details.

