# Introduction to Catalyst

**Catalyst** is a lightweight API testing tool. It allows you to define and execute HTTP API tests through a TOML configuration file.

## How Catalyst Works

Catalyst operates in a simple way:

1. You define your tests in a `.catalyst/tests.toml` file
2. You run the tests using the command-line interface
3. Catalyst executes the tests and reports the results

## Current Features

Catalyst currently supports:

- **HTTP Methods**: GET, POST, PUT, DELETE, PATCH, HEAD, OPTIONS
- **Request Configuration**: Headers, query parameters, and JSON bodies
- **Response Validation**: Status code verification
- **Variable Storage**: Extract values from JSON responses using path notation
- **Cookie Extraction**: Store cookies from responses for use in subsequent requests
- **Variable Substitution**: Use stored variables in endpoints, headers, and request bodies
- **Test Filtering**: Run specific tests by name

## Command-Line Interface

Catalyst provides a simple command-line interface with three main commands:

- `catalyst run` - Execute tests
- `catalyst validate` - Validate the test configuration file
- `catalyst list` - List available tests

## Configuration File

The configuration file (`.catalyst/tests.toml`) consists of:

- A global `[config]` section with settings that apply to all tests
- Multiple `[[tests]]` sections, each defining a single test

In the following sections, we'll guide you through installing Catalyst and creating your first test configuration.
