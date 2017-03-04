use json::JsonValue;

use super::JsonType;

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorReason {
    TypeMismatch { expected: JsonType, found: JsonType },
    TupleLengthMismatch { schemas: usize, tuple: usize },
    MaxLength { expected: usize, found: usize },
    MinLength { expected: usize, found: usize },
    MissingProperty(String),
    ArrayItemNotUnique,
    NumberRange { bound: f64, value: f64 },
}

pub type ValidationResult<'json> = Result<(), ValidationError<'json>>;

#[derive(Debug, Clone)]
pub struct ValidationError<'json> {
    pub reason: ErrorReason,
    pub node: &'json JsonValue,
}