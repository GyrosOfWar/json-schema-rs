use std::{fmt, error};

use serde_json::Value;

use util::JsonType;
use string::Format;
use regex::Regex;

#[derive(Debug)]
pub struct ValidationError<'json> {
    pub reason: ErrorKind,
    pub node: &'json Value,
}

impl<'json> ValidationError<'json> {
    pub fn type_mismatch(node: &'json Value, expected: JsonType, found: JsonType) -> ValidationError<'json> {
        ValidationError {
            reason: ErrorKind::TypeMismatch { 
                expected, found
            },
            node: node
        }
    }
}

impl<'json> fmt::Display for ValidationError<'json> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error at {}: {}", self.node, self.reason)
    }
}

#[derive(Debug)]
pub struct ValidationErrors<'json>(pub Vec<ValidationError<'json>>);

impl<'json> fmt::Display for ValidationErrors<'json> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for error in &self.0 {
            write!(f, "Error at {}: {}\n", error.node, error.reason)?;
        }
        Ok(())
    }
}

impl<'json> error::Error for ValidationErrors<'json> {
    fn description(&self) -> &str {
        "Errors occurred during validation"
    }
}

impl<'json> From<ValidationErrors<'json>> for Error {
    fn from(err: ValidationErrors<'json>) -> Error {
        Error::from(format!("{}", err))
    }
}

error_chain! {
    foreign_links {
        Io(::std::io::Error);
        Serde(::serde_json::Error);
    }

    errors {
        TypeMismatch { expected: JsonType, found: JsonType } {
            description("Type mismatch")
            display("Type mismatch: expected {}, found {}", expected, found)
        }
        TupleLengthMismatch { schemas: usize, tuple: usize } {
            description("Tuple length mismatch")
            display("Tuple length mismatch: expected {}, found {}", schemas, tuple)
        }
        MaxLength { expected: usize, found: usize } {
            description("Maximum length exceeded")
            display("Length mismatch: Expected a maximum of {}, found {}", expected, found)
        }
        MinLength { expected: usize, found: usize } {
            description("Value below minumum length")
            display("Length mismatch: Expected a minimum of {}, found {}", expected, found)
        }
        MissingProperty(prop: String) {
            description("Missing object property")
            display("Missing object property: `{}`", prop)
        }
        ArrayItemNotUnique {
            description("Array items are not unique")
            display("Array items are not unique")
        }
        NumberRange { bound: f64, value: f64 } {
            description("Number out of range")
            display("Number out of range: bound is {}, value is {}", bound, value)
        }
        PropertyCount { bound: usize, found: usize } {
            description("Property count out of range")
            display("Property count out of range: bound is {}, value is {}", bound, found)
        }
        InvalidRegex(regex: String) {
            description("Invalid regex")
            display("Invalid regex: {}", regex)
        }
        InvalidFormat(format: Format) {
            description("Error parsing with format")
            display("Error parsing with format: {:?}", format)
        }
        RegexMismatch { regex: Regex } {
            description("Regex did not match")
            display("Regex did not match: {}", regex)
        }
    }
}
