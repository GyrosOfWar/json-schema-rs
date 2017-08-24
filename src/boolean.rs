use serde_json::Value;

use util::{JsonType, JsonValueExt};
use schema::{Context, SchemaBase};
use errors::{ErrorKind, ValidationError};

/// A schema for a JSON boolean value (`true`, `false`).
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct BooleanSchema {
    description: Option<String>,
    id: Option<String>,
    title: Option<String>,
}

impl SchemaBase for BooleanSchema {
    #[doc(hidden)]
    fn validate_inner<'json>(
        &self,
        _ctx: &Context,
        value: &'json Value,
        errors: &mut Vec<ValidationError<'json>>,
    ) {
        if !value.is_boolean() {
            errors.push(ValidationError {
                reason: ErrorKind::TypeMismatch {
                    expected: JsonType::Boolean,
                    found: value.get_type(),
                },
                node: value,
            });
        }
    }
}
// TODO add builder struct
