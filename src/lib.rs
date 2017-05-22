//! A JSON schema validation library.
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

///  Error and result types
pub mod errors;
mod array;
pub mod schema;
mod object;
mod number;
mod string;
mod util;
mod boolean;
mod integer;

pub use errors::{ValidationError, ErrorKind, ValidationResult};
pub use schema::{SchemaBase, Schema};
pub use array::{ArraySchema, ArraySchemaBuilder};
pub use object::{ObjectSchema, ObjectSchemaBuilder};

use std::path::Path;
use std::fs::File;

use serde_json::Value;

use errors::Result;

/// Validates a JSON file with a schema. Returns an error if
/// the validation fails.
pub fn validate<P>(json_file: P, schema_file: P) -> Result<()>
    where P: AsRef<Path>
{
    let schema: Schema = serde_json::from_reader(File::open(schema_file)?)?;
    let json: Value = serde_json::from_reader(File::open(json_file)?)?;
    schema.validate(&json).map_err(From::from)
}