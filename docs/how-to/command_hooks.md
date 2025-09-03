# Command Hooks

Run shell commands before/after tests and at suite setup/teardown. Use hooks to seed data, generate tokens, or clean up.

## Safety: allowed_commands

Whitelist commands in `[config]` to prevent arbitrary execution:

```toml
[config]
base_url = "https://api.example.com"
allowed_commands = ["bash", "sh", "echo", "jq", "curl"]
```

## Suite-level hooks

Run once before all tests and once after all tests:

```toml
[config]
base_url = "https://api.example.com"
allowed_commands = ["bash", "sh", "echo", "jq"]

[[setup]]
run = "bash"
args = ["-lc", "echo seeding; ./scripts/seed.sh"]
timeout_ms = 60000

[[teardown]]
run = "bash"
args = ["-lc", "./scripts/cleanup.sh"]
ignore_error = true
```

## Test-level hooks

Each test can define `before` and `after` steps. `after` supports `on` to run on `success`, `failure`, or `always`.

```toml
[[tests]]
name = "Create and verify"
method = "POST"
endpoint = "/items"
body = { name = "demo" }
expected_status = 201

[[tests.before]]
run = "bash"
args = ["-lc", "echo START $(date +%s)"]
capture = { var = "start_meta" }   # saves stdout in {{start_meta}}

[[tests.after]]
run = "bash"
args = ["-lc", "echo {\"id\": 42, \"status\": \"ok\"}"]
on = "success"                 # run only if HTTP test succeeded
export = { item_id = "$.id" } # parse stdout as JSON and export to {{item_id}}
```

## Step fields

- run: command binary or script name
- args: optional arguments array
- shell: run in a shell (`sh -lc` or `cmd /C`) and pass args as a single string
- dir: working directory for the command
- env: map of environment variables for the command (supports `{{vars}}` and `${{ENV}}`)
- timeout_ms: default 30000
- ignore_error: don’t fail the step if the command exits non‑zero
- capture: `{ var = "name" }` saves stdout to `{{name}}` and stderr to `{{name}}_stderr`
- export: `{ var = "$.json.path" }` parse stdout as JSON and extract values into variables
- when: simple condition string, e.g. `"{{flag}}" == "true"`
- on: only for `after` steps (`success`, `failure`, `always`)

Notes
- Conditions support `==` and `!=` comparisons after substitution.
- Hooks honor `allowed_commands`; non-whitelisted commands fail validation/execution.
- Use `--debug` to print hook execution and variable substitutions.

