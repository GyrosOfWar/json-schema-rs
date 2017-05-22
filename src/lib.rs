#![allow(dead_code)]
#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate serde_json;
extern crate regex;
extern crate chrono;
#[macro_use]
extern crate lazy_static;

pub mod errors;
pub mod array;
pub mod schema;
pub mod object;
pub mod number;
pub mod string;
pub mod util;
pub mod boolean;
pub mod integer;

pub use errors::{ValidationError, ErrorReason};
pub use schema::{SchemaBase, Schema};
pub use array::{ArraySchema, ArraySchemaBuilder};
pub use object::{ObjectSchema, ObjectSchemaBuilder};
