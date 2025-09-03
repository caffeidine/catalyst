# Quickstart

This tutorial gets you from zero to a passing test in under five minutes.

## 1) Install Catalyst

```bash
cargo install catalyst --locked
```

Verify the install:

```bash
catalyst --help
```

## 2) Create the project structure

In your project folder, create the `.catalyst` directory and test file:

```bash
mkdir -p .catalyst
touch .catalyst/tests.toml
```

## 3) Add a minimal test

Paste the following into `.catalyst/tests.toml`:

```toml
[config]
base_url = "https://jsonplaceholder.typicode.com"

[[tests]]
name = "Fetch a post"
method = "GET"
endpoint = "/posts/1"
expected_status = 200
assertions = [
  { type = "contains", path = "userId", value = 1 },
  { type = "contains", path = "id", value = 1 }
]
```

Tip: Replace `base_url` with your API when ready.

## 4) Run the test

```bash
catalyst run
```

You should see a passing result. Use `-v` for verbose output.

## 5) Next steps

- Learn how to structure tests: [Your First Test](../getting-started/first_test)
- Explore CLI flags: [Run and Filter Tests](../getting-started/running_tests)
- See all options: [Schema Reference](../reference/schema)
