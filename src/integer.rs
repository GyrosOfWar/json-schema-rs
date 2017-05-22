use serde_json::Value;

use util::{JsonType, JsonValueExt};
use schema::SchemaBase;
use errors::{ValidationError, ErrorKind};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
    fn validate_inner<'json>(&self,
                             value: &'json Value,
                             errors: &mut Vec<ValidationError<'json>>) {
        match value.get_type() {
            JsonType::Integer => {}
            ty => {
                errors.push(ValidationError {
                                reason: ErrorKind::TypeMismatch {
                                    expected: JsonType::Integer,
                                    found: ty,
                                },
                                node: value,
                            })
            }
        }
    }
}
