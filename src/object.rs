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
    // TODO either object or bool
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
                        node: parent,
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
                    reason: ErrorReason::PropertyCount {
                        bound: min,
                        found: object.len(),
                    },
                    node: parent,
                })
            }
        }

        if let Some(max) = self.max_properties {
            if object.len() > max {
                errors.push(ValidationError {
                    reason: ErrorReason::PropertyCount {
                        bound: max,
                        found: object.len(),
                    },
                    node: parent,
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
                        let mut found_match = false;
                        for (prop, value) in object.iter() {
                            if re.is_match(prop) {
                                schema.validate_inner(value, errors);
                                found_match = true;
                            }
                        }
                        if !found_match {
                            // Error: No matching property found
                        }
                    }
                    Err(e) => {
                        errors.push(ValidationError {
                            reason: ErrorReason::InvalidRegex(format!("{}", e)),
                            node: parent,
                        })
                    }
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
                self.validate_pattern_properties(o, value, errors);
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

#[derive(Debug, Clone)]
pub struct ObjectSchemaBuilder<'schema> {
    description: Option<&'schema str>,
    id: Option<&'schema str>,
    title: Option<&'schema str>,

    property_schemas: Option<HashMap<String, Schema<'schema>>>,
    // TODO either object or bool
    additional_properties: bool,
    required: Option<Vec<&'schema str>>,
    min_properties: Option<usize>,
    max_properties: Option<usize>,
    pattern_properties: Option<HashMap<&'schema str, Schema<'schema>>>,
}

impl<'schema> Default for ObjectSchemaBuilder<'schema> {
    fn default() -> ObjectSchemaBuilder<'schema> {
        ObjectSchemaBuilder {
            description: Default::default(),
            id: Default::default(),
            title: Default::default(),

            property_schemas: Default::default(),
            additional_properties: true,
            required: Default::default(),
            min_properties: Default::default(),
            max_properties: Default::default(),
            pattern_properties: Default::default(),
        }
    }
}

impl<'schema> ObjectSchemaBuilder<'schema> {
    pub fn description<V: Into<&'schema str>>(mut self, value: V) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn id<V: Into<&'schema str>>(mut self, value: V) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn title<V: Into<&'schema str>>(mut self, value: V) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn property_schemas<V: Into<HashMap<String, Schema<'schema>>>>(mut self, value: V) -> Self {
        self.property_schemas = Some(value.into());
        self
    }

    pub fn required<V: Into<Vec<&'schema str>>>(mut self, value: V) -> Self {
        self.required = Some(value.into());
        self
    }

    pub fn additional_properties<V: Into<bool>>(mut self, value: V) -> Self {
        self.additional_properties = value.into();
        self
    }

    pub fn build(self) -> Schema<'schema> {
        From::from(ObjectSchema {
            description: self.description,
            id: self.id,
            title: self.title,

            property_schemas: self.property_schemas,
            additional_properties: self.additional_properties,
            required: self.required,
            min_properties: self.min_properties,
            max_properties: self.max_properties,
            pattern_properties: self.pattern_properties,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use json;

    #[test]
    fn required_props() {
        let input = json::parse(r#"{"id": 123.0, "name": "test", "unspecified": null}"#).unwrap();
        let schema = ObjectSchemaBuilder::default().required(vec!["id", "name"]).build();
        schema.validate(&input).unwrap();
    }

    #[test]
    fn disallow_additional() {
        let input = json::parse(r#"{"id": 123.0, "name": "test", "unspecified": null}"#).unwrap();
        let schema = ObjectSchemaBuilder::default()
            .additional_properties(false)
            .required(vec!["id", "name"])
            .build();
        schema.validate(&input).unwrap();
    }

    #[test]
    fn missing_props() {
        let input = json::parse(r#"{"id": 123.0, "name": "test"}"#).unwrap();
        let schema = ObjectSchemaBuilder::default().required(vec!["id", "name", "missing"]).build();
        let errors = schema.validate(&input).unwrap_err();
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].reason,
                   ErrorReason::MissingProperty(String::from("missing")));
    }
}
