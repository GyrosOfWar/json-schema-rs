use serde_json::Value;

use util::{JsonType, JsonValueExt};
use schema::{Context, SchemaBase};
use errors::ValidationError;

/// Schema for integer values like `42`.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct IntegerSchema {
    description: Option<String>,
    id: Option<String>,
    title: Option<String>,

    multiple_of: Option<f64>,
    minimum: Option<f64>,
    maximum: Option<f64>,
    exclusive_minimum: Option<bool>,
    exclusive_maximum: Option<bool>,
}

impl SchemaBase for IntegerSchema {
    #[doc(hidden)]
    fn validate_inner<'json>(
        &self,
        _ctx: &Context,
        value: &'json Value,
        errors: &mut Vec<ValidationError<'json>>,
    ) {
        match value.get_type() {
            JsonType::Integer => {}
            ty => errors.push(ValidationError::type_mismatch(value, JsonType::Integer, ty)),
        }
    }
}

// TODO make builder for schema
