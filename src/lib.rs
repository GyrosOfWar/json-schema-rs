#![allow(dead_code)]

extern crate json;
#[macro_use] extern crate error_chain;

mod errors;

use json::JsonValue;

pub type ValidationResult<'a> = Result<(), ValidationError<'a>>;

pub struct ValidationError<'a> {
    reason: ErrorReason,
    node: &'a JsonValue,
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

pub enum ErrorReason {
    TypeMismatch { expected: JsonType, found: JsonType },
    TupleLengthMismatch { schemas: usize, tuple: usize },
    MaxLength { expected: usize, found: usize },
    MinLength { expected: usize, found: usize },
}

pub trait SchemaBase {
    fn validate(&self, value: &JsonValue) -> ValidationResult;
}

pub enum Schema<'a> {
    Boolean(BooleanSchema<'a>),
    Object(ObjectSchema<'a>),
    Array(ArraySchema<'a>),
    Number(NumberSchema<'a>),
    String(StringSchema<'a>),
    Integer(IntegerSchema<'a>),
}

impl<'a> Schema<'a> {
    pub fn validate(&self, value: &JsonValue) -> ValidationResult {
        use self::Schema::*;
        // TODO this is not right
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

pub struct BooleanSchema<'a> {
    description: Option<&'a str>,
    id: Option<&'a str>,
    title: Option<&'a str>,
}

impl<'a> SchemaBase for BooleanSchema<'a> {
    fn validate(&self, value: &JsonValue) -> ValidationResult {
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

pub struct ObjectSchema<'a> {
    description: Option<&'a str>,
    id: Option<&'a str>,
    title: Option<&'a str>,

    // TODO
}

impl<'a> SchemaBase for ObjectSchema<'a> {
    fn validate(&self, value: &JsonValue) -> ValidationResult {
        unimplemented!()
    }
}

pub struct ArraySchema<'a> {
    min_items: Option<usize>,
    max_items: Option<usize>,
    unique_items: bool,
    all_items_schema: Box<Option<Schema<'a>>>,
    item_schemas: Option<Vec<Schema<'a>>>,
    // TODO
}

impl<'a> ArraySchema<'a> {
    fn validate_size(&self, array: &[JsonValue], parent: &JsonValue) -> ValidationResult {
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

    fn validate_all_items_schema(&self, array: &[JsonValue]) -> ValidationResult {
        if let Some(ref schema) = *self.all_items_schema {
            for value in array {
                schema.validate(&value)?;
            }
        }

        Ok(())
    }

    fn validate_item_schema(&self, array: &[JsonValue], parent: &JsonValue) -> ValidationResult {
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

impl<'a> SchemaBase for ArraySchema<'a> {
    fn validate(&self, value: &JsonValue) -> ValidationResult {
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

pub struct NumberSchema<'a> {
    description: Option<&'a str>,
    id: Option<&'a str>,
    title: Option<&'a str>,
}

impl<'a> SchemaBase for NumberSchema<'a> {
    fn validate(&self, value: &json::JsonValue) -> ValidationResult {
        unimplemented!()
    }
}

pub struct StringSchema<'a> {
    description: Option<&'a str>,
    id: Option<&'a str>,
    title: Option<&'a str>,
}

impl<'a> SchemaBase for StringSchema<'a> {
    fn validate(&self, value: &json::JsonValue) -> ValidationResult {
        unimplemented!()
    }
}

pub struct IntegerSchema<'a> {
    description: Option<&'a str>,
    id: Option<&'a str>,
    title: Option<&'a str>,
}

impl<'a> SchemaBase for IntegerSchema<'a> {
    fn validate(&self, value: &json::JsonValue) -> ValidationResult {
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
