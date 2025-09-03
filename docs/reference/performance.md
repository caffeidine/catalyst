# Performance Reference

Enforce response latency and reuse timings in later tests.

## max_response_time

Fail a test if it exceeds the threshold (ms):

```toml
max_response_time = 200
```

## response_time_ms variable

After each test, Catalyst sets `response_time_ms` which can be used in subsequent requests or logs:

```toml
body = { "prev_ms" = "{{response_time_ms}}" }
```
