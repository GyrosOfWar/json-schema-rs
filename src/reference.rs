use serde_json::Value;

use schema::{Context, Schema, SchemaBase};
use errors::ValidationError;

/// Schema that's a reference to another part of this schema.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReferenceSchema {
    #[serde(rename = "$ref")] reference: String,
}

impl ReferenceSchema {
    fn resolve(&self, ctx: &Context) -> Schema {
        // This document
        let root = ctx.root;
        if self.reference.starts_with('#') {
            match *root {
                Schema::Array(ref a) => {}
                _ => {}
            }
        }

        unimplemented!()
    }
}

impl SchemaBase for ReferenceSchema {
    fn validate_inner<'json>(
        &self,
        ctx: &Context,
        value: &'json Value,
        errors: &mut Vec<ValidationError<'json>>,
    ) {
        self.resolve(ctx).validate_inner(ctx, value, errors)
    }
}
