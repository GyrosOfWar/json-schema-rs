use std::str::FromStr;

use serde_json::{self, Value};

use boolean::BooleanSchema;
use integer::IntegerSchema;
use errors::{Error, ValidationError, ValidationErrors};
use array::ArraySchema;
use object::ObjectSchema;
use number::NumberSchema;
use string::StringSchema;
use reference::ReferenceSchema;

// TODO move the other parameters to the context?
#[doc(hidden)]
#[derive(Debug)]
pub struct Context<'s> {
    pub root: &'s Schema,
}

/// The trait that all schema types implement.
pub trait SchemaBase {
    #[doc(hidden)]
    fn validate_inner<'json>(
        &self,
        ctx: &Context,
        value: &'json Value,
        errors: &mut Vec<ValidationError<'json>>,
    );

    /// Validates a JSON value with this schema.
    fn validate_start<'json>(
        &self,
        value: &'json Value,
        root: &Schema,
    ) -> Result<(), ValidationErrors<'json>> {
        let mut errors = vec![];
        let context = Context { root };
        self.validate_inner(&context, value, &mut errors);

        if errors.is_empty() {
            Ok(())
        } else {
            Err(ValidationErrors(errors))
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Copy)]
#[doc(hidden)]
pub struct EmptySchema;

#[doc(hidden)]
impl SchemaBase for EmptySchema {
    fn validate_inner<'json>(
        &self,
        _ctx: &Context,
        _value: &'json Value,
        _errors: &mut Vec<ValidationError<'json>>,
    ) {

    }
}

/// Enum representing the different types of schemas.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Schema {
    /// Boolean schema. `true` or `false`.
    #[serde(rename = "boolean")]
    Boolean(BooleanSchema),
    /// A schema for a JSON object like `{"food": "noodles"}`
    #[serde(rename = "object")]
    Object(ObjectSchema),
    /// A schema for a JSON array like `["noodles", "eggs", "bacon"]`
    #[serde(rename = "array")]
    Array(ArraySchema),
    /// A schema for a JSON number, usually floating points like `3.14`.
    #[serde(rename = "number")]
    Number(NumberSchema),
    /// A schema for a string, like `"food"`
    #[serde(rename = "string")]
    String(StringSchema),
    /// A schem a for an integer like `42`.
    #[serde(rename = "integer")]
    Integer(IntegerSchema),
    /// The empty schema `{}`.
    Empty(EmptySchema),
    /// A reference to some other schema
    Reference(ReferenceSchema),
}

impl Schema {
    /// Kicks off validation for this schema.
    pub fn validate<'json>(&self, value: &'json Value) -> Result<(), ValidationErrors<'json>> {
        self.validate_start(value, self)
    }
    /// Resolve references for this schema
    pub fn resolve_references(&mut self, schema: &Value) {
        if let Some(obj) = schema.as_object() {
            for (key, value) in obj {
                if key == "$ref" && value.is_string() {
                    let path = value.as_str().unwrap();
                    // This document
                    if path.starts_with('#') {
                        if let Some(definition) = schema.pointer(&path[1..]) {
                            println!("{}", definition);
                        }
                    } 
                    // URI reference
                    else {
                        
                    }
                }
            }
        }
    }
}

impl FromStr for Schema {
    type Err = Error;
    fn from_str(s: &str) -> ::std::result::Result<Schema, Self::Err> {
        serde_json::from_str(s).map_err(From::from)
    }
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
impl_traits! { ReferenceSchema, Schema::Reference }

impl SchemaBase for Schema {
    #[doc(hidden)]
    fn validate_inner<'json>(
        &self,
        ctx: &Context,
        value: &'json Value,
        errors: &mut Vec<ValidationError<'json>>,
    ) {
        use self::Schema::*;
        match *self {
            Boolean(ref s) => s.validate_inner(ctx, value, errors),
            Object(ref s) => s.validate_inner(ctx, value, errors),
            Array(ref s) => s.validate_inner(ctx, value, errors),
            Number(ref s) => s.validate_inner(ctx, value, errors),
            String(ref s) => s.validate_inner(ctx, value, errors),
            Integer(ref s) => s.validate_inner(ctx, value, errors),
            Empty(ref s) => s.validate_inner(ctx, value, errors),
            Reference(ref s) => s.validate_inner(ctx, value, errors),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use serde_json::{self, Value};

    use super::Schema;

    #[test]
    fn test_schema_references() {
        let schema_raw: Value = serde_json::from_reader(File::open("data/schema-with-refs.json").unwrap()).unwrap();
        let mut parsed_schema: Schema = serde_json::from_value(schema_raw.clone()).unwrap();
        parsed_schema.resolve_references(&schema_raw);
    }
}