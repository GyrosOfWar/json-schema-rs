use std::net::{Ipv4Addr, Ipv6Addr};

use regex::Regex;
use serde_json::Value;
use chrono::prelude::*;
use url::Url;

use util::{JsonType, JsonValueExt};
use schema::{SchemaBase, Context, Schema};
use errors::{ValidationError, ErrorKind};

mod regex_serde {
    use serde::{self, Deserialize, Serializer, Deserializer};
    use regex::Regex;

    pub fn serialize<S>(regex: &Option<Regex>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *regex {
            Some(ref r) => serializer.serialize_str(r.as_str()),
            None => serializer.serialize_none(),
        }
    }


    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Regex>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Regex::new(&s).map_err(serde::de::Error::custom).map(Some)
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct StringSchema {
    description: Option<String>,
    id: Option<String>,
    title: Option<String>,

    min_length: Option<usize>,
    max_length: Option<usize>,
    pattern: Option<String>,
    format: Option<Format>,
}

impl StringSchema {
    fn validate_string<'json>(
        &self,
        value: &'json str,
        node: &'json Value,
        errors: &mut Vec<ValidationError<'json>>,
    ) {
        if let Some(format) = self.format {
            if !format.is_valid(value) {
                errors.push(ValidationError {
                    reason: ErrorKind::InvalidFormat(format),
                    node: node,
                })
            }
        }

        if let Some(min) = self.min_length {
            if value.len() < min {
                errors.push(ValidationError {
                    reason: ErrorKind::MinLength {
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
                    reason: ErrorKind::MinLength {
                        expected: max,
                        found: value.len(),
                    },
                    node: node,
                })
            }
        }

        if let Some(ref re) = self.pattern {
            match Regex::new(re) {
                Ok(re) => {
                    if !re.is_match(value) {
                        errors.push(ValidationError {
                            reason: ErrorKind::RegexMismatch { regex: re.clone() },
                            node: node,
                        })
                    }
                }
                Err(e) => {
                    errors.push(ValidationError {
                        reason: ErrorKind::InvalidRegex(re.clone()),
                        node: node,
                    })
                }
            }
        }
    }
}

impl SchemaBase for StringSchema {
    #[doc(hidden)]
    fn validate_inner<'json>(
        &self,
        ctx: &Context,
        value: &'json Value,
        errors: &mut Vec<ValidationError<'json>>,
    ) {
        match value {
            &Value::String(ref s) => {
                self.validate_string(s.as_str(), value, errors);
            }
            val => {
                errors.push(ValidationError::type_mismatch(
                    value,
                    JsonType::String,
                    value.get_type(),
                ))
            }
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct StringSchemaBuilder {
    description: Option<String>,
    id: Option<String>,
    title: Option<String>,

    min_length: Option<usize>,
    max_length: Option<usize>,
    pattern: Option<String>,
    format: Option<Format>,
}

#[allow(unused)]
impl StringSchemaBuilder {
    pub fn description<V: Into<String>>(mut self, value: V) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn id<V: Into<String>>(mut self, value: V) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn title<V: Into<String>>(mut self, value: V) -> Self {
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

    pub fn pattern(mut self, pattern: String) -> Self {
        self.pattern = Some(pattern);
        self
    }

    pub fn format(mut self, format: Format) -> Self {
        self.format = Some(format);
        self
    }

    pub fn build(self) -> Schema {
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

/// Checking the string's contents according to a given format.
#[derive(Clone, Debug, Copy, PartialEq, Serialize, Deserialize)]
pub enum Format {
    /// Date time format according to RFC 3339
    #[serde(rename = "date-time")]
    DateTime,
    /// An email address
    #[serde(rename = "email")]
    Email,
    /// A host name
    #[serde(rename = "hostname")]
    Hostname,
    /// An IPv4 address
    #[serde(rename = "ipv4")]
    Ipv4,
    /// An IPv6 address
    #[serde(rename = "ipv6")]
    Ipv6,
    /// A URI
    #[serde(rename = "uri")]
    Uri,
}

impl Format {
    fn is_valid(&self, input: &str) -> bool {
        match *self {
            Format::DateTime => DateTime::parse_from_rfc3339(input).is_ok(),
            Format::Uri => Url::parse(input).is_ok(),
            Format::Ipv4 => input.parse::<Ipv4Addr>().is_ok(),
            Format::Ipv6 => input.parse::<Ipv6Addr>().is_ok(),
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
        let schema = StringSchemaBuilder::default()
            .min_length(5)
            .max_length(10)
            .build();
        let input = serde_json::from_str(r#" "123456" "#).unwrap();
        schema.validate(&input).unwrap();
    }

    #[test]
    fn wrong_string_len() {
        let schema = StringSchemaBuilder::default()
            .min_length(5)
            .max_length(10)
            .build();
        let input = serde_json::from_str(r#" "123" "#).unwrap();
        assert!(schema.validate(&input).is_err());
    }

    #[test]
    fn date_format() {
        let schema = StringSchemaBuilder::default()
            .format(Format::DateTime)
            .build();
        let input = serde_json::from_str(r#" "1990-12-31T23:59:60Z" "#).unwrap();
        schema.validate(&input).unwrap();
    }

    #[test]
    fn wrong_date_format() {
        let schema = StringSchemaBuilder::default()
            .format(Format::DateTime)
            .build();
        let input = serde_json::from_str(r#" "1990-12-31T23:59:60" "#).unwrap();
        assert!(schema.validate(&input).is_err());
    }
}
