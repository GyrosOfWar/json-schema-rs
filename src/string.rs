use regex::Regex;
use serde_json::Value;
use chrono::prelude::*;

use util::{JsonType, JsonValueExt};
use schema::{SchemaBase, Schema};
use errors::{ValidationError, ErrorReason};

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
                              node: &'json Value,
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
                             value: &'json Value,
                             errors: &mut Vec<ValidationError<'json>>) {
        match *value {
            Value::String(ref s) => {
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

    fn from_json(node: &Value) -> Option<Schema> {
        None
    }
}

#[derive(Clone, Debug, Default)]
pub struct StringSchemaBuilder<'schema> {
    description: Option<&'schema str>,
    id: Option<&'schema str>,
    title: Option<&'schema str>,

    min_length: Option<usize>,
    max_length: Option<usize>,
    pattern: Option<Regex>,
    format: Option<Format>,
}

impl<'schema> StringSchemaBuilder<'schema> {
    pub fn description<V: Into<&'schema str>>(mut self, value: V) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn id<V: Into<&'schema str>>(mut self, value: V) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn title<V: Into<&'schema str>>(mut self, value: V) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn min_length(mut self, value: usize) -> Self {
        self.min_length = Some(value);
        self
    }

    pub fn max_length(mut self, value: usize) -> Self {
        self.max_length = Some(value);
        self
    }

    pub fn pattern(mut self, pattern: Regex) -> Self {
        self.pattern = Some(pattern);
        self
    }

    pub fn format(mut self, format: Format) -> Self {
        self.format = Some(format);
        self
    }

    pub fn build(self) -> Schema<'schema> {
        Schema::from(StringSchema {
            description: self.description,
            id: self.id,
            title: self.title,

            min_length: self.min_length,
            max_length: self.max_length,
            pattern: self.pattern,
            format: self.format,
        })
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
            _ => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn string_len() {
        let schema = StringSchemaBuilder::default().min_length(5).max_length(10).build();
        let input = serde_json::from_str(r#" "123455" "#).unwrap();
        schema.validate(&input).unwrap();
    }

    #[test]
    fn wrong_string_len() {
        let schema = StringSchemaBuilder::default().min_length(5).max_length(10).build();
        let input = serde_json::from_str(r#" "123" "#).unwrap();
        assert!(schema.validate(&input).is_err());
    }

    #[test]
    fn date_format() {
        let schema = StringSchemaBuilder::default().format(Format::DateTime).build();
        let input = serde_json::from_str(r#" "1990-12-31T23:59:60Z" "#).unwrap();
        schema.validate(&input).unwrap();
    }

    #[test]
    fn wrong_date_format() {
        let schema = StringSchemaBuilder::default().format(Format::DateTime).build();
        let input = serde_json::from_str(r#" "1990-12-31T23:59:60" "#).unwrap();
        assert!(schema.validate(&input).is_err());
    }
}
