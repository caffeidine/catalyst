# Catalyst

[![Version](https://img.shields.io/badge/version-0.2.3-blue)](https://github.com/caffeidine/catalyst/releases)
[![License: MPL-2.0](https://img.shields.io/badge/License-MPL--2.0-brightgreen.svg)](https://opensource.org/licenses/MPL-2.0)
[![Crates.io](https://img.shields.io/badge/crates.io-catalyst-orange)](https://crates.io/crates/catalyst)
[![Documentation](https://img.shields.io/badge/docs-catalyst.caffeidine.com-informational)](https://catalyst.caffeidine.com)
![crates.io total download number](https://img.shields.io/crates/d/catalyst)

**A lightweight and powerful API testing tool**

[Documentation](https://catalyst.caffeidine.com) | [Installation](#installation) | [Examples](#examples)

## Overview

**Catalyst** is a lightweight and extensible HTTP API testing tool. It allows you to define and execute API tests through a declarative configuration file, without writing any code.

## Features

- **Declarative Testing**: Configure your API test scenarios using a simple TOML file
- **Variable Management**: Chain your tests by extracting and storing variables (cookies, JSON data, etc.)
- **Configuration Validation**: Syntax and semantic checks before execution
- **JSON Assertions**: Partial validation of JSON responses with regex support
- **Performance**: Measure and validate response times

## Installation

```sh
cargo install catalyst
```

## Examples

Create your test file in your project `.catalyst/tests.toml`

```toml
[config]
base_url = "http://localhost:8080"
default_headers = { "User-Agent" = "Catalyst", "Content-Type" = "application/json" }

[[tests]]
name = "Example Test"
method = "GET"
endpoint = "/api/example"
expected_status = 200
max_response_time = 500 # maximum time in ms
assertions = [
  { type = "contains", path = "data.status", value = "success" },
  { type = "regex", path = "data.id", pattern = "^[0-9a-f]{8}$" }
]
```

## Usage

Execute the tests from the command line:

```sh
# Run all tests
catalyst run

# List all tests
catalyst list --verbose

# Validate your configuration
catalyst validate
```

## Documentation

For complete documentation, visit [catalyst.caffeidine.com](https://catalyst.caffeidine.com).

## License

This project is licensed under the [MPL-2.0](https://opensource.org/licenses/MPL-2.0).
