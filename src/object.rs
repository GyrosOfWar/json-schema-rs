use std::collections::HashMap;

use serde_json::Value;
use serde_json::value::Map;
use regex::Regex;

use util::{JsonType, JsonValueExt};
use schema::{Schema, SchemaBase};
use errors::{ValidationError, ErrorKind};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectSchema {
    description: Option<String>,
    id: Option<String>,
    title: Option<String>,

    property_schemas: Option<HashMap<String, Schema>>,
    // TODO either object or bool
    additional_properties: Option<bool>,
    required: Option<Vec<String>>,
    min_properties: Option<usize>,
    max_properties: Option<usize>,
    pattern_properties: Option<HashMap<String, Schema>>,
}

impl ObjectSchema {
    fn additional_properties(&self) -> bool {
        self.additional_properties.unwrap_or(false)
    }

    fn validate_properties<'json>(&self,
                                  object: &'json Map<String, Value>,
                                  parent: &'json Value,
                                  errors: &mut Vec<ValidationError<'json>>) {
        if let Some(ref schemas) = self.property_schemas {
            for (property, schema) in schemas {
                match object.get(property) {
                    Some(value) => {
                        schema.validate_inner(value, errors);
                    }
                    None => {
                        if !self.additional_properties() {
                            errors.push(ValidationError {
                                            reason:
                                                ErrorKind::MissingProperty(property.to_string()),
                                            node: parent,
                                        });
                        }
                    }
                }
            }
        }
    }

    fn validate_required<'json>(&self,
                                object: &'json Map<String, Value>,
                                parent: &'json Value,
                                errors: &mut Vec<ValidationError<'json>>) {
        if let Some(ref required) = self.required {
            for property in required {
                if object.get(property).is_none() {
                    errors.push(ValidationError {
                                    reason: ErrorKind::MissingProperty(property.to_string()),
                                    node: parent,
                                })
                }
            }
        }
    }

    fn validate_count<'json>(&self,
                             object: &'json Map<String, Value>,
                             parent: &'json Value,
                             errors: &mut Vec<ValidationError<'json>>) {
        if let Some(min) = self.min_properties {
            if object.len() < min {
                errors.push(ValidationError {
                                reason: ErrorKind::PropertyCount {
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
                                reason: ErrorKind::PropertyCount {
                                    bound: max,
                                    found: object.len(),
                                },
                                node: parent,
                            })
            }
        }
    }

    fn validate_pattern_properties<'json>(&self,
                                          object: &'json Map<String, Value>,
                                          parent: &'json Value,
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
                                        reason: ErrorKind::InvalidRegex(format!("{}", e)),
                                        node: parent,
                                    })
                    }
                }
            }
        }
    }
}

impl SchemaBase for ObjectSchema {
    fn validate_inner<'json>(&self,
                             value: &'json Value,
                             errors: &mut Vec<ValidationError<'json>>) {
        match value {
            &Value::Object(ref o) => {
                self.validate_properties(o, value, errors);
                self.validate_required(o, value, errors);
                self.validate_count(o, value, errors);
                self.validate_pattern_properties(o, value, errors);
            }
            val => {
                errors.push(ValidationError {
                                reason: ErrorKind::TypeMismatch {
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
pub struct ObjectSchemaBuilder {
    description: Option<String>,
    id: Option<String>,
    title: Option<String>,

    property_schemas: Option<HashMap<String, Schema>>,
    // TODO either object or bool
    additional_properties: bool,
    required: Option<Vec<String>>,
    min_properties: Option<usize>,
    max_properties: Option<usize>,
    pattern_properties: Option<HashMap<String, Schema>>,
}

impl Default for ObjectSchemaBuilder {
    fn default() -> ObjectSchemaBuilder {
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

impl ObjectSchemaBuilder {
    pub fn description<V: Into<String>>(mut self, value: V) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn id<V: Into<String>>(mut self, value: V) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn title<V: Into<String>>(mut self, value: V) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn property_schemas(mut self, value: HashMap<String, Schema>) -> Self {
        self.property_schemas = Some(value.into());
        self
    }

    pub fn required<V: Into<Vec<String>>>(mut self, value: V) -> Self {
        self.required = Some(value.into());
        self
    }

    pub fn additional_properties(mut self, value: bool) -> Self {
        self.additional_properties = value;
        self
    }

    pub fn add_property<K: Into<String>, V: Into<Schema>>(mut self, name: K, value: V) -> Self {
        let mut map;
        match self.property_schemas {
            Some(m) => {
                map = m;
            }
            None => {
                map = HashMap::new();
            }
        }
        map.insert(name.into(), value.into());
        self.property_schemas = Some(map);
        self
    }

    pub fn build(self) -> Schema {
        From::from(ObjectSchema {
                       description: self.description,
                       id: self.id,
                       title: self.title,

                       property_schemas: self.property_schemas,
                       additional_properties: Some(self.additional_properties),
                       required: self.required,
                       min_properties: self.min_properties,
                       max_properties: self.max_properties,
                       pattern_properties: self.pattern_properties,
                   })
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use super::*;
    use integer::IntegerSchema;
    use string::StringSchema;
    use array::ArraySchemaBuilder;
    use number::NumberSchema;
    use serde_json;

    #[test]
    fn required_props() {
        let input = serde_json::from_str(r#"{"id": 123.0, "name": "test", "unspecified": null}"#)
            .unwrap();
        let schema = ObjectSchemaBuilder::default()
            .required(vec!["id".into(), "name".into()])
            .build();
        schema.validate(&input).unwrap();
    }

    #[test]
    fn disallow_additional() {
        let input = serde_json::from_str(r#"{"id": 123.0, "name": "test", "unspecified": null}"#)
            .unwrap();
        let schema = ObjectSchemaBuilder::default()
            .additional_properties(false)
            .required(vec!["id".into(), "name".into()])
            .build();
        schema.validate(&input).unwrap();
    }

    #[test]
    fn missing_props() {
        let input = serde_json::from_str(r#"{"id": 123.0, "name": "test"}"#).unwrap();
        let schema = ObjectSchemaBuilder::default()
            .required(vec!["id".into(), "name".into(), "missing".into()])
            .build();
        let errors = schema.validate(&input).unwrap_err().0;
        assert_eq!(errors.len(), 1);

        if let ErrorKind::MissingProperty(ref prop) = errors[0].reason {
            assert_eq!(prop.as_str(), "missing");
        } else {
            assert!(false, "Wrong property");
        }
    }

    #[test]
    fn schema_properties() {
        let input = serde_json::from_str(r#"{"id": 123, "name": "test", "tags": ["a", "b", "c"],
        "color": [255, 255, 255]}"#)
                .unwrap();
        let mut schemas = HashMap::new();
        schemas.insert("id".into(), Schema::from(IntegerSchema::default()));
        schemas.insert("name".into(), Schema::from(StringSchema::default()));
        let tags = ArraySchemaBuilder::default()
            .all_items_schema(Schema::from(StringSchema::default()))
            .build();
        schemas.insert("tags".into(), tags);
        let color = ArraySchemaBuilder::default()
            .additional_items(false)
            .item_schemas(vec![Schema::from(IntegerSchema::default()),
                               Schema::from(IntegerSchema::default()),
                               Schema::from(IntegerSchema::default())])
            .build();
        schemas.insert("color".into(), color);

        let schema = ObjectSchemaBuilder::default()
            .property_schemas(schemas)
            .additional_properties(false)
            .build();

        schema.validate(&input).unwrap();
    }

    fn canada_schema() -> Schema {
        let vector = ArraySchemaBuilder::default()
            .item_schemas(vec![From::from(NumberSchema::default()),
                               From::from(NumberSchema::default())])
            .build();

        let coordinates = ArraySchemaBuilder::default()
            .all_items_schema(ArraySchemaBuilder::default()
                                  .all_items_schema(vector)
                                  .build())
            .build();

        let geometry = ObjectSchemaBuilder::default()
            .add_property("type", StringSchema::default())
            .add_property("coordinatees", coordinates)
            .build();

        let features = ArraySchemaBuilder::default()
            .all_items_schema(ObjectSchemaBuilder::default()
                                  .add_property("type", StringSchema::default())
                                  .add_property("geometry", geometry)
                                  .build())
            .build();
        ObjectSchemaBuilder::default()
            .add_property("type", Schema::from(StringSchema::default()))
            .add_property("features", features)
            .build()
    }

    #[test]
    fn test_canada_small() {
        let input = serde_json::from_reader(File::open("data/canada-small.json").unwrap()).unwrap();
        let schema = canada_schema();
        schema.validate(&input).unwrap();
    }

    #[test]
    fn test_canada_big() {
        let input = serde_json::from_reader(File::open("data/canada.json").unwrap()).unwrap();
        let schema = canada_schema();
        schema.validate(&input).unwrap();
    }

    #[test]
    fn parse_schema() {
        let schema: Schema =
            serde_json::from_reader(File::open("data/canada-schema.json").unwrap()).unwrap();
        let input = serde_json::from_reader(File::open("data/canada.json").unwrap()).unwrap();
        schema.validate(&input).unwrap();
    }
}
