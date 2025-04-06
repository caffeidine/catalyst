# Creating Your First Test

Creating your first test with Catalyst is straightforward. This guide will walk you through the process step by step.

## 1. Create the Directory Structure

First, create a `.catalyst` directory in your project root:

```bash
mkdir -p .catalyst
```

## 2. Create the Test File

Create a file named `tests.toml` inside the `.catalyst` directory:

```bash
touch .catalyst/tests.toml
```

## 3. Define the Global Configuration

Open the `tests.toml` file and add the global configuration:

```toml
[config]
base_url = "https://api.example.com"  # Replace with your API base URL
default_headers = { "Content-Type" = "application/json" }
```

## 4. Add Your First Test

Add a test definition to the file:

```toml
[[tests]]
name = "Simple GET Request"
method = "GET"
endpoint = "/status"
expected_status = 200
```

This test will make a GET request to `https://api.example.com/status` and expect a 200 status code in response.

## 5. Complete Example

Your complete `tests.toml` file should look like this:

```toml
[config]
base_url = "https://api.example.com"
default_headers = { "Content-Type" = "application/json" }

[[tests]]
name = "Simple GET Request"
method = "GET"
endpoint = "/status"
expected_status = 200
```

## Next Steps

Now that you've created your first test, learn how to [run it](./running_tests.md).
