#![allow(dead_code)]
#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate json;
#[macro_use]
extern crate error_chain;

mod errors;

use std::collections::{HashMap, HashSet};

use json::JsonValue;
use json::object::Object;

pub type ValidationResult<'json> = Result<(), ValidationError<'json>>;

pub struct ValidationError<'json> {
    reason: ErrorReason,
    node: &'json JsonValue,
}

#[derive(Debug, Clone, Copy)]
pub enum JsonType {
    Null,
    Boolean,
    Object,
    Array,
    Number,
    String,
    Integer,
}

#[derive(Debug, Clone)]
pub enum ErrorReason {
    TypeMismatch {
        expected: JsonType,
        found: JsonType,
    },
    TupleLengthMismatch {
        schemas: usize,
        tuple: usize,
    },
    MaxLength {
        expected: usize,
        found: usize,
    },
    MinLength {
        expected: usize,
        found: usize,
    },
    MissingProperty(String),
    ArrayItemNotUnique {}
}

pub trait SchemaBase {
    fn validate<'json>(&self, value: &'json JsonValue, errors: &mut Vec<ValidationError<'json>>);
}

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

pub struct ArraySchema<'schema> {
    description: Option<&'schema str>,
    id: Option<&'schema str>,
    title: Option<&'schema str>,

    min_items: Option<usize>,
    max_items: Option<usize>,
    unique_items: bool,
    all_items_schema: Box<Option<Schema<'schema>>>,
    item_schemas: Option<Vec<Schema<'schema>>>,
}

impl<'schema> ArraySchema<'schema> {
    fn validate_size<'json>(&self,
                            array: &'json [JsonValue],
                            parent: &'json JsonValue,
                            errors: &mut Vec<ValidationError<'json>>) {
        if let Some(min) = self.min_items {
            if array.len() < min {
                errors.push(ValidationError {
                    reason: ErrorReason::MinLength {
                        expected: min,
                        found: array.len(),
                    },
                    node: parent,
                });
            }
        }
        if let Some(max) = self.max_items {
            if array.len() > max {
                errors.push(ValidationError {
                    reason: ErrorReason::MaxLength {
                        expected: max,
                        found: array.len(),
                    },
                    node: parent,
                });
            }
        }
    }

    fn validate_all_items_schema<'json>(&self,
                                        array: &'json [JsonValue],
                                        errors: &mut Vec<ValidationError<'json>>) {
        if let Some(ref schema) = *self.all_items_schema {
            for value in array {
                schema.validate(&value, errors);
            }
        }
    }

    fn validate_item_schema<'json>(&self,
                                   array: &'json [JsonValue],
                                   parent: &'json JsonValue,
                                   errors: &mut Vec<ValidationError<'json>>) {
        if let Some(ref schemas) = self.item_schemas {
            if schemas.len() != array.len() {
                errors.push(ValidationError {
                    reason: ErrorReason::TupleLengthMismatch {
                        schemas: schemas.len(),
                        tuple: array.len(),
                    },
                    node: parent,
                });
            }

            for (schema, value) in schemas.iter().zip(array) {
                schema.validate(value, errors);
            }
        }
    }

    fn validate_unique<'json>(&self,
                              array: &'json [JsonValue],
                              parent: &'json JsonValue,
                              errors: &mut Vec<ValidationError<'json>>) {
        // TODO implement PartialEq<JsonValue> for JsonValue
        /*
        if self.unique_items {
            let mut unique_items = vec![];
            for item in array {
                for contained in &unique_items {
                    if contained == item {
                        errors.push(ValidationError {
                            node: parent,
                            reason: ErrorReason::ArrayItemNotUnique {}
                        });
                        return;
                    }
                }
                unique_items.push(item);
            }
        }*/
    }
}


impl<'schema> SchemaBase for ArraySchema<'schema> {
    fn validate<'json>(&self, value: &'json JsonValue, errors: &mut Vec<ValidationError<'json>>) {
        match value {
            &JsonValue::Array(ref array) => {
                self.validate_size(array, value, errors);
                self.validate_all_items_schema(array, errors);
                self.validate_unique(array, value, errors);
            }
            val => {
                errors.push(ValidationError {
                    reason: ErrorReason::TypeMismatch {
                        expected: JsonType::Array,
                        found: val.get_type(),
                    },
                    node: value,
                })
            }
        }
    }
}

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
            JsonValue::Number(_) => JsonType::Number,
        }
    }
}

pub struct SchemaBuilder<'s> {
    root: Schema<'s>
}

#[cfg(test)]
mod tests {
    use json;
    use super::*;

    #[test]
    fn array_schema() {
        let input = "[1, 2, 3, 4]";
        let schema = ArraySchema {
            description: None,
            id: None,
            title: None,
            min_items: Some(1),
            max_items: Some(2),
            unique_items: false,
            all_items_schema: Box::new(None),
            item_schemas: None,
        };
        let json = json::parse(input).unwrap();
        let mut errors = vec![];
        schema.validate(&json, &mut errors);
        assert!(errors.len() > 0);
    }
}
