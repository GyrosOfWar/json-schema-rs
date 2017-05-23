use serde_json::Value;

use schema::{SchemaBase, Context, Schema};
use errors::ValidationError;
use util::JsonValueExt;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReferenceSchema {
    #[serde(rename = "$ref")]
    reference: String,
}

impl ReferenceSchema {
    fn resolve(&self) -> Schema {
        // TODO
        // This document
        if self.reference.starts_with('#') {

        }

        unimplemented!()
    }
}

impl SchemaBase for ReferenceSchema {
    fn validate_inner<'json>(&self,
                             ctx: &Context,
                             value: &'json Value,
                             errors: &mut Vec<ValidationError<'json>>) {
        self.resolve().validate_inner(ctx, value, errors)
    }
}
