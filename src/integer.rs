use json::JsonValue;

use util::{JsonType, JsonValueExt};
use schema::{SchemaBase, Schema};
use errors::{ValidationError, ErrorReason};

#[derive(Clone, Debug, Default)]
pub struct IntegerSchema<'schema> {
    description: Option<&'schema str>,
    id: Option<&'schema str>,
    title: Option<&'schema str>,

    multiple_of: Option<f64>,
    minimum: Option<f64>,
    maximum: Option<f64>,
    exclusive_minimum: bool,
    exclusive_maximum: bool,
}

impl<'schema> SchemaBase for IntegerSchema<'schema> {
    fn validate_inner<'json>(&self,
                             value: &'json JsonValue,
                             errors: &mut Vec<ValidationError<'json>>) {
        match value.get_type() {
            JsonType::Integer => {}
            ty => {
                errors.push(ValidationError {
                    reason: ErrorReason::TypeMismatch {
                        expected: JsonType::Integer,
                        found: ty,
                    },
                    node: value,
                })
            }
        }
    }

    fn from_json(node: &JsonValue) -> Option<Schema> {
        None
    }
}
