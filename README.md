[![crates.io](https://meritbadge.herokuapp.com/turbonone)](https://crates.io/crates/turbonone)
[![docs.rs](https://docs.rs/turbonone/badge.svg)](https://docs.rs/turbonone/)
[![license](https://img.shields.io/crates/l/turbonone)](https://github.com/WilliamVenner/turbonone/blob/master/LICENSE)

# turbonone (no_std)

Tiny macro for calling functions with generic `Option<T>` arguments.

## Usage

Add to your [Cargo.toml](https://doc.rust-lang.org/cargo/reference/manifest.html) file:

```toml
[dependencies]
turbonone = "0.*"
```

## The Problem

```rust
fn my_function<T>(arg: Option<T>) -> &'static str {
    "Works!"
}

fn my_box_function<T>(arg: Option<Box<T>>) -> &'static str {
    "Works!"
}

fn my_complex_function<T>(arg: Option<Arc<Box<T>>>) -> &'static str {
    "Works!"
}

my_function(None); // cannot infer type for type parameter `T` declared on the associated function `my_function`
my_function(Some("An argument")); // Works!

my_box_function(None); // cannot infer type for type parameter `T` declared on the associated function `my_box_function`
my_box_function(Some("An argument")); // Works!

my_complex_function(None); // cannot infer type for type parameter `T` declared on the associated function `my_complex_function`
my_complex_function(Some("An argument")); // Works!
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

fn my_box_function<T>(arg: Option<Box<T>>) -> &'static str {
    "Works!"
}

fn my_complex_function<T>(arg: Option<Arc<Box<T>>>) -> &'static str {
    "Works!"
}

my_function(turbonone!()); // Works!
my_function(Some("An argument")); // Works!

my_box_function(turbonone!(Box)); // Works!
my_box_function(turbonone!(Box<()>)); // Works!
my_box_function(Some("An argument")); // Works!

my_complex_function(turbonone!(Arc<Box<()>>)); // Works!
my_complex_function(Some("An argument")); // Works!
```
