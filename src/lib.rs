#![allow(dead_code)]

extern crate json;
#[macro_use] extern crate error_chain;

mod errors;

use json::JsonValue;

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

#[derive(Debug, Clone, Copy)]
pub enum ErrorReason {
    TypeMismatch { expected: JsonType, found: JsonType },
    TupleLengthMismatch { schemas: usize, tuple: usize },
    MaxLength { expected: usize, found: usize },
    MinLength { expected: usize, found: usize },
}

pub trait SchemaBase {
    fn validate<'json>(&self, value: &'json JsonValue) -> ValidationResult<'json>;
}

pub enum Schema<'schema> {
    Boolean(BooleanSchema<'schema>),
    Object(ObjectSchema<'schema>),
    Array(ArraySchema<'schema>),
    Number(NumberSchema<'schema>),
    String(StringSchema<'schema>),
    Integer(IntegerSchema<'schema>),
}

impl<'schema> Schema<'schema> {
    pub fn validate<'json>(&self, value: &'json JsonValue) -> ValidationResult<'json> {
        use self::Schema::*;
        match *self {
            Boolean(ref s) => s.validate(value),
            Object(ref s) => s.validate(value),
            Array(ref s) => s.validate(value),
            Number(ref s) => s.validate(value),
            String(ref s) => s.validate(value),
            Integer(ref s) => s.validate(value)
        }
    }
}

pub struct BooleanSchema<'schema> {
    description: Option<&'schema str>,
    id: Option<&'schema str>,
    title: Option<&'schema str>,
}

impl<'schema> SchemaBase for BooleanSchema<'schema> {
    fn validate<'json>(&self, value: &'json JsonValue) -> ValidationResult<'json> {
        if value.is_boolean() {
            Ok(())
        } else {
            Err(ValidationError {
                reason: ErrorReason::TypeMismatch { expected: JsonType::Boolean, found: value.get_type() },
                node: value,
            })
        }
    }
}

pub struct ObjectSchema<'schema> {
    description: Option<&'schema str>,
    id: Option<&'schema str>,
    title: Option<&'schema str>,

    // TODO
}

impl<'schema> SchemaBase for ObjectSchema<'schema> {
    fn validate<'json>(&self, value: &'json JsonValue) -> ValidationResult<'json> {
        unimplemented!()
    }
}

pub struct ArraySchema<'schema> {
    min_items: Option<usize>,
    max_items: Option<usize>,
    unique_items: bool,
    all_items_schema: Box<Option<Schema<'schema>>>,
    item_schemas: Option<Vec<Schema<'schema>>>,
    // TODO
}

impl<'schema> ArraySchema<'schema> {
    fn validate_size<'json>(&self, array: &'json [JsonValue], parent: &'json JsonValue) -> ValidationResult<'json> {
        if let Some(min) = self.min_items {
            if array.len() < min {
                return Err(ValidationError {
                    reason: ErrorReason::MinLength { expected: min, found: array.len() },
                    node: parent,
                })
            }
        }
        if let Some(max) = self.max_items {
            if array.len() > max {
                return Err(ValidationError {
                    reason: ErrorReason::MaxLength { expected: max, found: array.len() },
                    node: parent,
                })
            }
        }

        Ok(())
    }

    fn validate_all_items_schema<'json>(&self, array: &'json [JsonValue]) -> ValidationResult<'json> {
        if let Some(ref schema) = *self.all_items_schema {
            for value in array {
                schema.validate(&value)?;
            }
        }

        Ok(())
    }

    fn validate_item_schema<'json>(&self, array: &'json [JsonValue], parent: &'json JsonValue) -> ValidationResult<'json> {
        if let Some(ref schemas) = self.item_schemas {
            if schemas.len() != array.len() {
                return Err(ValidationError {
                    reason: ErrorReason::TupleLengthMismatch { schemas: schemas.len(), tuple: array.len() },
                    node: parent
                })
            }

            for (schema, value) in schemas.iter().zip(array) {
                schema.validate(value)?;
            }
        }

        Ok(())
    }
}


impl<'schema> SchemaBase for ArraySchema<'schema> {
    fn validate<'json>(&self, value: &'json JsonValue) -> ValidationResult<'json> {
        match value {
            &JsonValue::Array(ref array) => {
                self.validate_size(array, value)?;
                self.validate_all_items_schema(array)?;
                Ok(())
            },
            val => Err(ValidationError {
                reason: ErrorReason::TypeMismatch { expected: JsonType::Array, found: val.get_type() },
                node: value,
            })
        }
    }
}

pub struct NumberSchema<'schema> {
    description: Option<&'schema str>,
    id: Option<&'schema str>,
    title: Option<&'schema str>,
}

impl<'schema> SchemaBase for NumberSchema<'schema> {
    fn validate<'json>(&self, value: &'json JsonValue) -> ValidationResult<'json> {
        unimplemented!()
    }
}

pub struct StringSchema<'schema> {
    description: Option<&'schema str>,
    id: Option<&'schema str>,
    title: Option<&'schema str>,
}

impl<'schema> SchemaBase for StringSchema<'schema> {
    fn validate<'json>(&self, value: &'json JsonValue) -> ValidationResult<'json> {
        unimplemented!()
    }
}

pub struct IntegerSchema<'schema> {
    description: Option<&'schema str>,
    id: Option<&'schema str>,
    title: Option<&'schema str>,
}

impl<'a> SchemaBase for IntegerSchema<'a> {
    fn validate<'json>(&self, value: &'json JsonValue) -> ValidationResult<'json> {
        unimplemented!()
    }
}

pub trait HasJsonType {
    fn get_type(&self) -> JsonType;
}

impl HasJsonType for JsonValue {
    fn get_type(&self) -> JsonType {
        match *self {
            JsonValue::Boolean(_) => JsonType::Boolean,
            JsonValue::Array(_) => JsonType::Array,
            JsonValue::Null => JsonType::Null,
            JsonValue::String(_) | JsonValue::Short(_) => JsonType::String,
            JsonValue::Object(_) => JsonType::Object,
            JsonValue::Number(n) => JsonType::Number
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
