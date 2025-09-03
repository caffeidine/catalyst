# Run, Filter, and Debug

Run the suite, target specific tests, and print debug logs.

## Run all
```bash
catalyst run
```

## Filter by name
```bash
catalyst run --filter "Login"
```

## Custom file and variables
```bash
catalyst run --file .catalyst/staging.toml --var base_url=$BASE,token=$TOKEN
```

## Verbose and no color
```bash
catalyst run -v --disable-color
```

## Debug internals
```bash
catalyst run --debug
```

More: ../reference/cli.md and ../getting-started/running_tests.md
