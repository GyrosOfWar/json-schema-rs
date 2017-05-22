use serde_json::Value;

use boolean::BooleanSchema;
use integer::IntegerSchema;
use errors::ValidationError;
use array::ArraySchema;
use object::ObjectSchema;
use number::NumberSchema;
use string::StringSchema;

pub trait SchemaBase {
    #[doc(hidden)]
    fn validate_inner<'json>(&self,
                             value: &'json Value,
                             errors: &mut Vec<ValidationError<'json>>);

    fn from_json(node: &Value) -> Option<Schema>;

    fn validate<'json>(&self, value: &'json Value) -> Result<(), Vec<ValidationError<'json>>> {
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
pub struct EmptySchema;

impl SchemaBase for EmptySchema {
    fn from_json(_node: &Value) -> Option<Schema> {
        None
    }

    fn validate_inner<'json>(&self, value: &'json Value, errors: &mut Vec<ValidationError<'json>>) {
        
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
    Empty(EmptySchema),
}

macro_rules! impl_traits {
    ($name:ty, $schema:path) => (
        impl <'schema> From<$name> for Schema<'schema> {
            fn from(value: $name) -> Schema<'schema> {
                $schema(value)
            }
        }
    )
}

impl_traits! { BooleanSchema<'schema>, Schema::Boolean }
impl_traits! { ObjectSchema<'schema>, Schema::Object }
impl_traits! { ArraySchema<'schema>, Schema::Array }
impl_traits! { NumberSchema<'schema>, Schema::Number }
impl_traits! { StringSchema<'schema>, Schema::String }
impl_traits! { IntegerSchema<'schema>, Schema::Integer }
impl_traits! { EmptySchema, Schema::Empty }

impl<'schema> SchemaBase for Schema<'schema> {
    fn validate_inner<'json>(&self,
                             value: &'json Value,
                             errors: &mut Vec<ValidationError<'json>>) {
        use self::Schema::*;
        match *self {
            Boolean(ref s) => s.validate_inner(value, errors),
            Object(ref s) => s.validate_inner(value, errors),
            Array(ref s) => s.validate_inner(value, errors),
            Number(ref s) => s.validate_inner(value, errors),
            String(ref s) => s.validate_inner(value, errors),
            Integer(ref s) => s.validate_inner(value, errors),
            Empty(ref s) => s.validate_inner(value, errors),
        }
    }

    fn from_json(node: &Value) -> Option<Schema> {
        None
    }
}
