use serde_json::Value;

use util::{JsonType, JsonValueExt};
use schema::{SchemaBase, Schema};
use errors::{ValidationError, ErrorReason};

#[derive(Clone, Debug)]
pub struct BooleanSchema<'schema> {
    description: Option<&'schema str>,
    id: Option<&'schema str>,
    title: Option<&'schema str>,
}

impl<'schema> SchemaBase for BooleanSchema<'schema> {
    fn validate_inner<'json>(&self,
                             value: &'json Value,
                             errors: &mut Vec<ValidationError<'json>>) {
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
    fn from_json(node: &Value) -> Option<Schema> {
        None
    }
}
