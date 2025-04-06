# Getting Started with Catalyst

This section will guide you through the basics of using Catalyst to test your APIs. We'll cover how to create a simple test configuration file, run your tests, and interpret the results.

## Overview

Using Catalyst involves three main steps:

1. **Create a test configuration file** (`.catalyst/tests.toml`) that defines your API tests
2. **Run the tests** using the Catalyst command-line interface
3. **Review the results** to identify any issues with your API

## Directory Structure

Catalyst expects your test configuration to be in a `.catalyst` directory in your project root:

```
your-project/
├── .catalyst/
│   └── tests.toml    # Your test configuration file
└── ...
```

## Basic Example

Here's a simple example of a test configuration file that tests a REST API:

```toml
[config]
base_url = "https://api.example.com"
default_headers = { "Content-Type" = "application/json" }

[[tests]]
name = "Get Users"
method = "GET"
endpoint = "/users"
expected_status = 200

[[tests]]
name = "Create User"
method = "POST"
endpoint = "/users"
body = { "name" = "John Doe", "email" = "john@example.com" }
expected_status = 201
```

This configuration defines two tests:

1. A GET request to `/users` that should return a 200 status code
2. A POST request to `/users` with a JSON body that should return a 201 status code

## Next Steps

In the following sections, we'll explore:

- [Creating Your First Test](./getting_started/first_test.md) - A step-by-step guide to creating your first test
- [Running Tests](./getting_started/running_tests.md) - How to run your tests and interpret the results
