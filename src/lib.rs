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

use std::path::Path;
use std::fs::File;
use std::io;
use std::collections::HashMap;

use serde_json::Value;

use errors::{Result, ValidationErrors};

/// Validates a JSON file with a schema. Returns an error if
/// opening either of the files or the validation fails.
pub fn validate<P>(json_file: P, schema_file: P) -> Result<()>
    where P: AsRef<Path>
{
    let schema_file = io::BufReader::new(File::open(schema_file)?);
    let schema: Schema = serde_json::from_reader(schema_file)?;
    let json_file = io::BufReader::new(File::open(json_file)?);
    let json: Value = serde_json::from_reader(json_file)?;
    schema.validate(&json).map_err(From::from)
}

/// Parses a schema from the given path.
pub fn parse_schema<P>(path: P) -> Result<Schema>
    where P: AsRef<Path>
{
    let file = io::BufReader::new(File::open(path)?);
    serde_json::from_reader(file).map_err(From::from)
}