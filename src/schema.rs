use serde_json::Value;

use boolean::BooleanSchema;
use integer::IntegerSchema;
use errors::{ValidationError, ValidationErrors};
use array::ArraySchema;
use object::ObjectSchema;
use number::NumberSchema;
use string::StringSchema;

pub trait SchemaBase {
    #[doc(hidden)]
    fn validate_inner<'json>(&self,
                             value: &'json Value,
                             errors: &mut Vec<ValidationError<'json>>);

    fn validate<'json>(&self, value: &'json Value) -> Result<(), ValidationErrors<'json>> {
        let mut errors = vec![];
        self.validate_inner(value, &mut errors);

        if errors.is_empty() {
            Ok(())
        } else {
            Err(ValidationErrors(errors))
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Copy)]
pub struct EmptySchema;

impl SchemaBase for EmptySchema {
    fn validate_inner<'json>(&self,
                             _value: &'json Value,
                             _errors: &mut Vec<ValidationError<'json>>) {

    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Schema {
    #[serde(rename = "boolean")]
    Boolean(BooleanSchema),
    #[serde(rename = "object")]
    Object(ObjectSchema),
    #[serde(rename = "array")]
    Array(ArraySchema),
    #[serde(rename = "number")]
    Number(NumberSchema),
    #[serde(rename = "string")]
    String(StringSchema),
    #[serde(rename = "integer")]
    Integer(IntegerSchema),
    Empty(EmptySchema),
}

macro_rules! impl_traits {
    ($name:ty, $schema:path) => (
        impl  From<$name> for Schema {
            fn from(value: $name) -> Schema {
                $schema(value)
            }
        }
    )
}

impl_traits! { BooleanSchema, Schema::Boolean }
impl_traits! { ObjectSchema, Schema::Object }
impl_traits! { ArraySchema, Schema::Array }
impl_traits! { NumberSchema, Schema::Number }
impl_traits! { StringSchema, Schema::String }
impl_traits! { IntegerSchema, Schema::Integer }
impl_traits! { EmptySchema, Schema::Empty }

impl SchemaBase for Schema {
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
}
