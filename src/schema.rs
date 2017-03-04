use json::JsonValue;

use super::{BooleanSchema, StringSchema, IntegerSchema};
use errors::ValidationError;
use array::ArraySchema;
use object::ObjectSchema;
use number::NumberSchema;

pub trait SchemaBase {
    fn validate_inner<'json>(&self,
                             value: &'json JsonValue,
                             errors: &mut Vec<ValidationError<'json>>);

    fn validate<'json>(&self, value: &'json JsonValue) -> Result<(), Vec<ValidationError<'json>>> {
        let mut errors = vec![];
        self.validate_inner(value, &mut errors);

        if errors.len() == 0 {
            Ok(())
        } else {
            Err(errors)
        }
    }
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

macro_rules! impl_from {
    ($name:ty, $schema:path) => (
        impl <'schema> From<$name> for Schema<'schema> {
            fn from(value: $name) -> Schema<'schema> {
                $schema(value)
            }
        }
    )
}

impl_from! { BooleanSchema<'schema>, Schema::Boolean }
impl_from! { ObjectSchema<'schema>, Schema::Object }
impl_from! { ArraySchema<'schema>, Schema::Array }
impl_from! { NumberSchema<'schema>, Schema::Number }
impl_from! { StringSchema<'schema>, Schema::String }
impl_from! { IntegerSchema<'schema>, Schema::Integer }

impl<'schema> SchemaBase for Schema<'schema> {
    fn validate_inner<'json>(&self,
                             value: &'json JsonValue,
                             errors: &mut Vec<ValidationError<'json>>) {
        use self::Schema::*;
        match *self {
            Boolean(ref s) => s.validate_inner(value, errors),
            Object(ref s) => s.validate_inner(value, errors),
            Array(ref s) => s.validate_inner(value, errors),
            Number(ref s) => s.validate_inner(value, errors),
            String(ref s) => s.validate_inner(value, errors),
            Integer(ref s) => s.validate_inner(value, errors),
        }
    }
}
