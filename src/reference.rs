use serde_json::Value;

use schema::{Context, SchemaBase};
use errors::ValidationError;

/// Schema that's a reference to another part of this schema.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReferenceSchema {
    #[serde(rename = "$ref")] reference: String,
}

impl SchemaBase for ReferenceSchema {
    fn validate_inner<'json>(&self,
                             _ctx: &Context,
                             _value: &'json Value,
                             _errors: &mut Vec<ValidationError<'json>>) {
        //self.resolve().validate_inner(ctx, value, errors)
    }
}
