# Hooks (Before/After/Setup/Teardown)

Prepare data, generate tokens, and clean up using command hooks.

- Suite-level: `[[setup]]`, `[[teardown]]`
- Test-level: `[[tests.before]]`, `[[tests.after]]` (supports `on`)
- Safety: whitelist commands via `[config].allowed_commands`

Example
```toml
[config]
base_url = "https://api.example.com"
allowed_commands = ["bash", "sh", "echo"]

[[setup]]
run = "bash"
args = ["-lc", "./scripts/seed.sh"]

[[tests]]
name = "Create item"
method = "POST"
endpoint = "/items"
body = { name = "demo" }
expected_status = 201

[[tests.after]]
run = "bash"
args = ["-lc", "echo {\"id\": 42}"]
on = "success"
export = { item_id = "$.id" }
```

Learn more: ../how-to/command_hooks.md
