Needs Environment Variable
==========================

[![Crates.io Version](https://img.shields.io/crates/v/needs_env_var)](https://crates.io/crates/needs_env_var)

Skip code (such as a test) **at compile time** if an environment variable is undefined.

# Example

`some_test` will not be compiled if `SOMEENVIRONMENTVARIABLE` is not defined:

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

❗**Note:** As `needs_env_var` is evaluated at compile time you need to force a re-compilation an environment variable
is defined after compilation, e.g. with `cargo clean`❗ 