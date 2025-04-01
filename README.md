Needs Environment Variable
==========================

[![Crates.io Version](https://img.shields.io/crates/v/needs_env_var)](https://crates.io/crates/needs_env_var)

Early return from a test (or method in general) if an environment variable is undefined or does not match a specific value.

# Example

`some_test` will be skipped if `SOMEENVIRONMENTVARIABLE` is not defined or does not match `1` (examples 2 & 3).

```rust
use needs_env_var::*;

#[needs_env_var(SOMEENVIRONMENTVARIABLE)]
#[test]
fn some_test() {
    assert!(1 == 1);
}

// or if its specified value not matched
#[needs_env_var(SOMEENVIRONMENTVARIABLE = 1)]
#[test]
fn some_test() {
    assert!(1 == 1);
}


#[needs_env_var(SOMEENVIRONMENTVARIABLE=1)]
#[test]
fn some_test() {
    assert!(1 == 1);
}
```
