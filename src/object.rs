use std::collections::HashMap;

use json::JsonValue;
use json::object::Object;
use regex::Regex;

use super::{JsonType, JsonValueExt};
use schema::{Schema, SchemaBase};
use errors::{ValidationError, ErrorReason};

#[derive(Clone, Debug)]
pub struct ObjectSchema<'schema> {
    description: Option<&'schema str>,
    id: Option<&'schema str>,
    title: Option<&'schema str>,

    property_schemas: Option<HashMap<String, Schema<'schema>>>,
    // TODO either object or bool!
    additional_properties: bool,
    required: Option<Vec<&'schema str>>,
    min_properties: Option<usize>,
    max_properties: Option<usize>,
    pattern_properties: Option<HashMap<&'schema str, Schema<'schema>>>,
}

impl<'schema> ObjectSchema<'schema> {
    fn validate_properties<'json>(&self,
                                  object: &'json Object,
                                  parent: &'json JsonValue,
                                  errors: &mut Vec<ValidationError<'json>>) {
        if let Some(ref schemas) = self.property_schemas {
            for (property, schema) in schemas {
                match object.get(&property) {
                    Some(value) => {
                        schema.validate_inner(value, errors);
                    }
                    None => {
                        if !self.additional_properties {
                            errors.push(ValidationError {
                                reason: ErrorReason::MissingProperty(property.clone()),
                                node: parent,
                            });
                        }
                    }
                }
            }
        }
    }

    fn validate_required<'json>(&self,
                                object: &'json Object,
                                parent: &'json JsonValue,
                                errors: &mut Vec<ValidationError<'json>>) {
        if let Some(ref required) = self.required {
            for property in required {
                if object.get(property).is_none() {
                    errors.push(ValidationError {
                        reason: ErrorReason::MissingProperty(property.to_string()),
                        node: parent
                    })
                }
            }
        }
    }

    fn validate_count<'json>(&self,
                                object: &'json Object,
                                parent: &'json JsonValue,
                                errors: &mut Vec<ValidationError<'json>>) {
        if let Some(min) = self.min_properties {
            if object.len() < min {
                errors.push(ValidationError {
                    reason: ErrorReason::PropertyCount { bound: min, found: object.len() },
                    node: parent
                })
            }
        }

        if let Some(max) = self.max_properties {
            if object.len() > max {
                errors.push(ValidationError {
                    reason: ErrorReason::PropertyCount { bound: max, found: object.len() },
                    node: parent
                })
            }
        }
    }

    fn validate_pattern_properties<'json>(&self,
                                object: &'json Object,
                                parent: &'json JsonValue,
                                errors: &mut Vec<ValidationError<'json>>) {
        if let Some(ref patterns) = self.pattern_properties {
            for (pattern, schema) in patterns {
                // TODO(performance) cache compiled regexes
                match Regex::new(pattern) {
                    Ok(re) => {
                        // TODO
                    },
                    Err(e) => errors.push(ValidationError {
                        reason: ErrorReason::InvalidRegex(format!("{}", e)),
                        node: parent
                    })
                }
            }
        }
    }
}

impl<'schema> SchemaBase for ObjectSchema<'schema> {
    fn validate_inner<'json>(&self,
                             value: &'json JsonValue,
                             errors: &mut Vec<ValidationError<'json>>) {
        match value {
            &JsonValue::Object(ref o) => {
                self.validate_properties(o, value, errors);
                self.validate_required(o, value, errors);
                self.validate_count(o, value, errors);
            }
            val => {
                errors.push(ValidationError {
                    reason: ErrorReason::TypeMismatch {
                        expected: JsonType::Object,
                        found: val.get_type(),
                    },
                    node: val,
                });
            }
        }
    }
}

//pub struct ObjectSchemaBuilder<'schema> {
    
//}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn required_props() {
        
    }
}