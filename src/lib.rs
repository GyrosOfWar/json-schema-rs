#![allow(dead_code)]
#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate json;
extern crate regex;
extern crate chrono;
#[macro_use]
extern crate lazy_static;

mod errors;
mod array;
mod schema;
mod object;
mod number;
mod string;

use json::JsonValue;
use regex::Regex;

pub use errors::{ValidationError, ErrorReason};
pub use schema::{SchemaBase, Schema};
pub use array::{ArraySchema, ArraySchemaBuilder};
pub use object::{ObjectSchema, ObjectSchemaBuilder};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JsonType {
    Null,
    Boolean,
    Object,
    Array,
    Number,
    String,
    Integer,
}

#[derive(Clone, Debug)]
pub struct BooleanSchema<'schema> {
    description: Option<&'schema str>,
    id: Option<&'schema str>,
    title: Option<&'schema str>,
}

impl<'schema> SchemaBase for BooleanSchema<'schema> {
    fn validate_inner<'json>(&self,
                             value: &'json JsonValue,
                             errors: &mut Vec<ValidationError<'json>>) {
        if !value.is_boolean() {
            errors.push(ValidationError {
                reason: ErrorReason::TypeMismatch {
                    expected: JsonType::Boolean,
                    found: value.get_type(),
                },
                node: value,
            });
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct IntegerSchema<'schema> {
    description: Option<&'schema str>,
    id: Option<&'schema str>,
    title: Option<&'schema str>,
}

impl<'schema> SchemaBase for IntegerSchema<'schema> {
    fn validate_inner<'json>(&self,
                             value: &'json JsonValue,
                             errors: &mut Vec<ValidationError<'json>>) {
        match value.get_type() {
            JsonType::Integer => {}
            ty => {
                errors.push(ValidationError {
                    reason: ErrorReason::TypeMismatch {
                        expected: JsonType::Integer,
                        found: ty,
                    },
                    node: value,
                })
            }
        }
    }
}

pub trait JsonValueExt {
    fn get_type(&self) -> JsonType;
}

impl JsonValueExt for JsonValue {
    fn get_type(&self) -> JsonType {
        match *self {
            JsonValue::Boolean(_) => JsonType::Boolean,
            JsonValue::Array(_) => JsonType::Array,
            JsonValue::Null => JsonType::Null,
            JsonValue::String(_) |
            JsonValue::Short(_) => JsonType::String,
            JsonValue::Object(_) => JsonType::Object,
            JsonValue::Number(_) => {
                let n = self.as_f64().unwrap();
                if n.trunc() == n {
                    JsonType::Integer
                } else {
                    JsonType::Number
                }
            }
        }
    }
}
