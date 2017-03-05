use regex::Regex;
use json::JsonValue;

use util::{JsonType, JsonValueExt};
use schema::SchemaBase;
use errors::{ValidationError, ErrorReason};
use chrono::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct StringSchema<'schema> {
    description: Option<&'schema str>,
    id: Option<&'schema str>,
    title: Option<&'schema str>,

    min_length: Option<usize>,
    max_length: Option<usize>,
    pattern: Option<Regex>,
    format: Option<Format>,
}

impl<'schema> StringSchema<'schema> {
    fn validate_string<'json>(&self,
                              value: &'json str,
                              node: &'json JsonValue,
                              errors: &mut Vec<ValidationError<'json>>) {
        if let Some(format) = self.format {
            if !format.is_valid(value) {
                errors.push(ValidationError {
                    reason: ErrorReason::InvalidFormat(format),
                    node: node,
                })
            }
        }

        if let Some(min) = self.min_length {
            if value.len() < min {
                errors.push(ValidationError {
                    reason: ErrorReason::MinLength {
                        expected: min,
                        found: value.len(),
                    },
                    node: node,
                })
            }
        }

        if let Some(max) = self.max_length {
            if value.len() > max {
                errors.push(ValidationError {
                    reason: ErrorReason::MinLength {
                        expected: max,
                        found: value.len(),
                    },
                    node: node,
                })
            }
        }

        if let Some(ref re) = self.pattern {
            if !re.is_match(value) {
                errors.push(ValidationError {
                    reason: ErrorReason::RegexMismatch { regex: re.clone() },
                    node: node,
                })
            }
        }
    }
}

impl<'schema> SchemaBase for StringSchema<'schema> {
    fn validate_inner<'json>(&self,
                             value: &'json JsonValue,
                             errors: &mut Vec<ValidationError<'json>>) {
        match *value {
            JsonValue::String(ref s) => {
                self.validate_string(s.as_str(), value, errors);
            }
            JsonValue::Short(ref s) => {
                self.validate_string(s.as_str(), value, errors);
            }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_len() {}
}
