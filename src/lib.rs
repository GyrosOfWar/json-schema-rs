//! A JSON schema validation library.
//! TODO
//! [ ] Null schema
//! [ ] schema references per JSON pointer syntax
//! [ ] enums

#![allow(unused, dead_code)]
#![deny(missing_debug_implementations, missing_copy_implementations,
        trivial_casts, trivial_numeric_casts,
        unsafe_code, unstable_features,
        unused_import_braces, unused_qualifications)]
#![warn(missing_docs)]
#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
#![recursion_limit="128"]

#[macro_use]
extern crate error_chain;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate regex;
extern crate chrono;
extern crate url;

/// Error and result types
pub mod errors;
/// Basic types
pub mod schema;
/// Implementation of the array schema
pub mod array;
/// Implementation of the object schema
pub mod object;
/// Implementation of the number schema
pub mod number;
/// Implementation of the string schema
pub mod string;
/// Implementation of the boolean schema
pub mod boolean;
/// Implementation of the integer schmea
pub mod integer;
mod util;

pub use schema::{SchemaBase, Schema};
pub use array::ArraySchemaBuilder;
pub use object::ObjectSchemaBuilder;
