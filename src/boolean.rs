use serde_json::Value;

use util::{JsonType, JsonValueExt};
use schema::SchemaBase;
use errors::{ValidationError, ErrorKind};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BooleanSchema {
    description: Option<String>,
    id: Option<String>,
    title: Option<String>,
}

impl SchemaBase for BooleanSchema {
    fn validate_inner<'json>(&self,
                             value: &'json Value,
                             errors: &mut Vec<ValidationError<'json>>) {
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
