//! [![crates.io](https://meritbadge.herokuapp.com/turbonone)](https://crates.io/crates/turbonone)
//! [![docs.rs](https://docs.rs/turbonone/badge.svg)](https://docs.rs/turbonone/)
//! [![license](https://img.shields.io/crates/l/turbonone)](https://github.com/WilliamVenner/turbonone/blob/master/LICENSE)
//!
//! Simple macro for calling functions with generic `Option<T>` arguments.
//!
//! ## Usage
//!
//! Add to your [Cargo.toml](https://doc.rust-lang.org/cargo/reference/manifest.html) file:
//!
//! ```toml
//! [dependencies]
//! turbonone = "0.*"
//! ```
//!
//! ## The Problem
//!
//! ```ignore
//! # extern crate core;
//! # extern crate turbonone;
//! # use turbonone::turbonone;
//! fn my_function<T>(arg: Option<T>) -> &'static str {
//!     "Works!"
//! }
//!
//! fn my_box_function<T>(arg: Option<Box<T>>) -> &'static str {
//!     "Works!"
//! }
//!
//! fn my_complex_function<T>(arg: Option<Arc<Box<T>>>) -> &'static str {
//!     "Works!"
//! }
//!
//! my_function(None); // cannot infer type for type parameter `T` declared on the associated function `my_function`
//! my_function(Some("An argument")); // Works!
//!
//! my_box_function(None); // cannot infer type for type parameter `T` declared on the associated function `my_box_function`
//! my_box_function(Some(Box::new("An argument"))); // Works!
//!
//! my_complex_function(None); // cannot infer type for type parameter `T` declared on the associated function `my_complex_function`
//! my_complex_function(Some(Arc::new(Box::new("An argument")))); // Works!
//! ```
//!
//! ## The Solution
//!
//! ```rust
//! # extern crate core;
//! # extern crate turbonone;
//! # #[macro_use]
//! use turbonone::turbonone;
//! # use std::sync::Arc;
//!
//! fn my_function<T>(arg: Option<T>) -> &'static str {
//!     "Works!"
//! }
//!
//! fn my_box_function<T>(arg: Option<Box<T>>) -> &'static str {
//!     "Works!"
//! }
//!
//! fn my_complex_function<T>(arg: Option<Arc<Box<T>>>) -> &'static str {
//!     "Works!"
//! }
//!
//! my_function(turbonone!()); // Works!
//! my_function(Some("An argument")); // Works!
//!
//! my_box_function(turbonone!(Box)); // Works!
//! my_box_function(turbonone!(Box<()>)); // Works!
//! my_box_function(Some(Box::new("An argument"))); // Works!
//!
//! my_complex_function(turbonone!(Arc<Box<()>>)); // Works!
//! my_complex_function(Some(Arc::new(Box::new("An argument")))); // Works!
//! ```

#![no_std]

extern crate alloc;
use alloc::{format, string::ToString};

extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn turbonone(tokens: TokenStream) -> TokenStream {
    if tokens.is_empty() {
        "::core::option::Option::None::<()>".parse().unwrap()
    } else {
        let mut elided_unit_type = true;
        {
            let mut i: u8 = 0;
            for _ in tokens.clone().into_iter() {
                i += 1;
                if i > 1 {
                    elided_unit_type = false;
                    break;
                }
            }
        }
        if elided_unit_type {
            format!("::core::option::Option::None::<{}<()>>", tokens.to_string())
        } else {
            format!("::core::option::Option::None::<{}>", tokens.to_string())
        }
        .parse()
        .unwrap()
    }
}
