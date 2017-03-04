use regex::Regex;
use json::JsonValue;

use super::{JsonType, JsonValueExt};
use schema::SchemaBase;
use errors::{ValidationError, ErrorReason};
use chrono::prelude::*;

#[derive(Clone, Debug)]
pub struct StringSchema<'schema> {
    description: Option<&'schema str>,
    id: Option<&'schema str>,
    title: Option<&'schema str>,

    min_length: Option<usize>,
    max_lengt: Option<usize>,
    pattern: Option<Regex>,
    format: Option<Format>,
}

impl<'schema> SchemaBase for StringSchema<'schema> {
    fn validate_inner<'json>(&self,
                             value: &'json JsonValue,
                             errors: &mut Vec<ValidationError<'json>>) {
        match *value {
            JsonValue::String(ref s) => {}
            JsonValue::Short(ref s) => {}
            _ => {
                errors.push(ValidationError {
                    reason: ErrorReason::TypeMismatch {
                        expected: JsonType::String,
                        found: value.get_type(),
                    },
                    node: value,
                });
            }
        }
    }
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Format {
    DateTime,
    Email,
    Hostname,
    Ipv4,
    Ipv6,
    Uri,
}

impl Format {
    pub fn is_valid(&self, input: &str) -> bool {
        match *self {
            Format::DateTime => DateTime::parse_from_rfc3339(input).is_ok(),
            _ => unimplemented!(),
        }
    }
}
