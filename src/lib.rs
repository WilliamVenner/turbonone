//! Tiny macro for calling functions with `Option<T>` arguments.
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
//! my_function(None); // cannot infer type for type parameter `T` declared on the associated function `my_function`
//! my_function(Some("An argument")); // Works!
//! ```
//! 
//! ## The Solution
//! 
//! ```rust
//! # extern crate core;
//! # extern crate turbonone;
//! # #[macro_use]
//! use turbonone::turbonone;
//! 
//! fn my_function<T>(arg: Option<T>) -> &'static str {
//!     "Works!"
//! }
//! 
//! my_function(turbonone!()); // Works!
//! my_function(Some("An argument")); // Works!
//! ```

#![no_std]
#[macro_export]
/// ```rust
/// #![no_std]
/// #[macro_export]
/// macro_rules! turbonone { () => { ::core::option::Option::None::<()> } }
/// ```
macro_rules! turbonone { () => { ::core::option::Option::None::<()> } }
