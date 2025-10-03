# Tutorial: Your First API Test

This tutorial will guide you through writing with **Node.js** and running your first API tests with **Catalyst**
against a simple local web server.

## Step 1: Create a Sample Project (example with nodeJS)

First, let's create a new directory for our project and set up a very simple Rust web server to
test against.

### Folder Structure

```code
my-api-tests/
    ├── server.js    # nodejs server
    └── tests.toml   #
    └── ...
```

### 1. Create a new project directory:

    1. mkdir my-api-tests
    2. cd my-api-tests

### 2. Create a new file named server.js

Add the following code. This will be our server.

```javascript
// server.js
import express from "express";
import cors from "cors";
const PORT = process.env.PORT || 8000;

const app = express();

app.use(cors({ origin: "*" }));

app.get("/hello/:usename", (req, res) => {
  const { username } = req.params;
  res.status(200).send(`Hello ${username}`);
});

app.set("json spaces", 2);
app.listen(PORT, () => console.log("Server connected on Port : ", PORT));
```

## Step 2: Write Your Catalyst Tests

Now, let's write the tests for our API.

### Create a new test file and add the following:

```toml
    1     [config]
    2     base_url = "http://localhost:8080"
    3     default_headers = { "Content-Type" = "application/json" }
    4
    5     # test get a hello follow by the name
    6     [[tests]]
    7     name = "Greet"
    8     method = "GET"
    9     endpoint = "/hello/Ismael"
   10
   11     expected_status = 200
   12
```

**NB:** The file name can be any name. In our example we use `test.toml`

## Step 3: Run Your Tests

### 1. First, start your sample API server in a separate terminal:

```bash
   node server.js
```

### 2. In your project directory, run Catalyst:

```bash
    catalyst  run --file tests.toml
```

- More running tests commands here : [Running Tests](../getting_started/running_tests.md)

### 3. Expected Output:

**Output for success result**

```bash
✔ Loaded tests from tests.toml
✔ Base URL set to http://localhost:8000
✔ Default headers: { "Content-Type": "application/json" }

→ Running test: Greet
  GET http://localhost:8000/hello/ismael
  [PASS] Greet                        (200 Success)


Test Summary:
Total: 1, Passed: 1, Failed: 2
```

**Output for a failed result**

```bash
$ catalyst run --file tests.toml

✔ Loaded tests from tests.toml
✔ Base URL set to http://localhost:8080
✔ Default headers: { "Content-Type": "application/json" }

  Running API tests...
  GET http://localhost:8080/hello/
  [FAIL] Greet                        (404 Not Found) (expected 200)

Failed tests:
  - Greet

Test Summary:
Total: 1, Passed: 0, Failed: 1
```
