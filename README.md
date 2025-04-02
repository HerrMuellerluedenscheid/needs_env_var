Needs Environment Variable
==========================

[![Crates.io Version](https://img.shields.io/crates/v/needs_env_var)](https://crates.io/crates/needs_env_var)

Early return from a test (or method in general) if an environment variable is undefined or does not match a specific value.

# Example

`some_test` will be skipped if `MY_ENVIRONMENT_VARIABLE` is not defined (example 1) or does not match `1` (examples 2).

```rust
use needs_env_var::*;

// Test will be skipped if `MY_ENVIRONMENT_VARIABLE` is not defined
#[needs_env_var(MY_ENVIRONMENT_VARIABLE)]
#[test]
fn some_test() {
    // ...
}

// or if does not match `1`
#[needs_env_var(MY_ENVIRONMENT_VARIABLE=1)]
#[test]
fn some_test() {
    // ...
}
```
