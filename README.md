# turbonone (no_std)

Tiny macro for calling functions with `Option<T>` arguments.

## The Problem

```rust
fn my_function<T>(arg: Option<T>) -> &'static str {
    "Works!"
}

my_function(None); // cannot infer type for type parameter `T` declared on the associated function `my_function`
my_function(Some("An argument")); // Works!
```

## The Solution

```rust
// Rust 2015
#[macro_use] extern crate turbonone;

// Rust 2018
use turbonone::turbonone;

fn my_function<T>(arg: Option<T>) -> &'static str {
    "Works!"
}

my_function(turbonone!()); // Works!
my_function(Some("An argument")); // Works!
```
