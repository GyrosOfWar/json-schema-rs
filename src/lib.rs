#![allow(dead_code)]
#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate json;
#[macro_use]
extern crate error_chain;

mod errors;
mod array;

use std::collections::HashMap;

use json::JsonValue;
use json::object::Object;

use array::ArraySchema;

pub type ValidationResult<'json> = Result<(), ValidationError<'json>>;

#[derive(Debug, Clone)]
pub struct ValidationError<'json> {
    reason: ErrorReason,
    node: &'json JsonValue,
}

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorReason {
    TypeMismatch { expected: JsonType, found: JsonType },
    TupleLengthMismatch { schemas: usize, tuple: usize },
    MaxLength { expected: usize, found: usize },
    MinLength { expected: usize, found: usize },
    MissingProperty(String),
    ArrayItemNotUnique,
}

pub trait SchemaBase {
    fn validate<'json>(&self, value: &'json JsonValue, errors: &mut Vec<ValidationError<'json>>);
}

#[derive(Clone, Debug)]
pub enum Schema<'schema> {
    Boolean(BooleanSchema<'schema>),
    Object(ObjectSchema<'schema>),
    Array(ArraySchema<'schema>),
    Number(NumberSchema<'schema>),
    String(StringSchema<'schema>),
    Integer(IntegerSchema<'schema>),
}

impl<'schema> SchemaBase for Schema<'schema> {
    fn validate<'json>(&self, value: &'json JsonValue, errors: &mut Vec<ValidationError<'json>>) {
        use self::Schema::*;
        match *self {
            Boolean(ref s) => s.validate(value, errors),
            Object(ref s) => s.validate(value, errors),
            Array(ref s) => s.validate(value, errors),
            Number(ref s) => s.validate(value, errors),
            String(ref s) => s.validate(value, errors),
            Integer(ref s) => s.validate(value, errors),
        }
    }
}

#[derive(Clone, Debug)]
pub struct BooleanSchema<'schema> {
    description: Option<&'schema str>,
    id: Option<&'schema str>,
    title: Option<&'schema str>,
}

impl<'schema> SchemaBase for BooleanSchema<'schema> {
    fn validate<'json>(&self, value: &'json JsonValue, errors: &mut Vec<ValidationError<'json>>) {
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

#[derive(Clone, Debug)]
pub struct ObjectSchema<'schema> {
    description: Option<&'schema str>,
    id: Option<&'schema str>,
    title: Option<&'schema str>,

    property_schemas: Option<HashMap<String, Schema<'schema>>>,
}

impl<'schema> ObjectSchema<'schema> {
    fn validate_properties<'json>(&self,
                                  object: &'json Object,
                                  parent: &'json JsonValue,
                                  errors: &mut Vec<ValidationError<'json>>) {
        if let Some(ref schemas) = self.property_schemas {
            for (property, schema) in schemas {
                match object.get(&property) {
                    Some(value) => {
                        schema.validate(value, errors);
                    }
                    None => {
                        errors.push(ValidationError {
                            reason: ErrorReason::MissingProperty(property.clone()),
                            node: parent,
                        });
                    }
                }
            }
        }
    }
}

impl<'schema> SchemaBase for ObjectSchema<'schema> {
    fn validate<'json>(&self, value: &'json JsonValue, errors: &mut Vec<ValidationError<'json>>) {
        match value {
            &JsonValue::Object(_) => {}
            val => {
                errors.push(ValidationError {
                    reason: ErrorReason::TypeMismatch {
                        expected: JsonType::Object,
                        found: val.get_type(),
                    },
                    node: val,
                });
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct NumberSchema<'schema> {
    description: Option<&'schema str>,
    id: Option<&'schema str>,
    title: Option<&'schema str>,
}

impl<'schema> SchemaBase for NumberSchema<'schema> {
    fn validate<'json>(&self, value: &'json JsonValue, errors: &mut Vec<ValidationError<'json>>) {
        unimplemented!()
    }
}

#[derive(Clone, Debug)]
pub struct StringSchema<'schema> {
    description: Option<&'schema str>,
    id: Option<&'schema str>,
    title: Option<&'schema str>,
}

impl<'schema> SchemaBase for StringSchema<'schema> {
    fn validate<'json>(&self, value: &'json JsonValue, errors: &mut Vec<ValidationError<'json>>) {
        unimplemented!()
    }
}

#[derive(Clone, Debug)]
pub struct IntegerSchema<'schema> {
    description: Option<&'schema str>,
    id: Option<&'schema str>,
    title: Option<&'schema str>,
}

impl<'schema> SchemaBase for IntegerSchema<'schema> {
    fn validate<'json>(&self, value: &'json JsonValue, errors: &mut Vec<ValidationError<'json>>) {
        unimplemented!()
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
