use json::JsonValue;

use super::{Schema, SchemaBase, JsonValueExt, JsonType};
use errors::{ValidationError, ErrorReason};

#[derive(Clone, Debug)]
pub struct ArraySchema<'schema> {
    description: Option<&'schema str>,
    id: Option<&'schema str>,
    title: Option<&'schema str>,

    min_items: Option<usize>,
    max_items: Option<usize>,
    unique_items: bool,

    all_items_schema: Box<Option<Schema<'schema>>>,
    item_schemas: Option<Vec<Schema<'schema>>>,

    additional_items: bool,
}

impl<'schema> ArraySchema<'schema> {
    fn validate_size<'json>(&self,
                            array: &'json [JsonValue],
                            parent: &'json JsonValue,
                            errors: &mut Vec<ValidationError<'json>>) {
        if let Some(min) = self.min_items {
            if array.len() < min {
                errors.push(ValidationError {
                    reason: ErrorReason::MinLength {
                        expected: min,
                        found: array.len(),
                    },
                    node: parent,
                });
            }
        }
        if let Some(max) = self.max_items {
            if array.len() > max {
                errors.push(ValidationError {
                    reason: ErrorReason::MaxLength {
                        expected: max,
                        found: array.len(),
                    },
                    node: parent,
                });
            }
        }
    }

    fn validate_all_items_schema<'json>(&self,
                                        array: &'json [JsonValue],
                                        errors: &mut Vec<ValidationError<'json>>) {
        if let Some(ref schema) = *self.all_items_schema {
            for value in array {
                schema.validate(&value, errors);
            }
        }
    }

    fn validate_item_schema<'json>(&self,
                                   array: &'json [JsonValue],
                                   parent: &'json JsonValue,
                                   errors: &mut Vec<ValidationError<'json>>) {
        if let Some(ref schemas) = self.item_schemas {
            if schemas.len() != array.len() && !self.additional_items {
                errors.push(ValidationError {
                    reason: ErrorReason::TupleLengthMismatch {
                        schemas: schemas.len(),
                        tuple: array.len(),
                    },
                    node: parent,
                });
            }

            for (schema, value) in schemas.iter().zip(array) {
                schema.validate(value, errors);
            }
        }
    }

    fn validate_unique<'json>(&self,
                              array: &'json [JsonValue],
                              parent: &'json JsonValue,
                              errors: &mut Vec<ValidationError<'json>>) {
        if self.unique_items {
            let mut unique_items = vec![];
            for item in array {
                for contained in &unique_items {
                    if *contained == item {
                        errors.push(ValidationError {
                            node: parent,
                            reason: ErrorReason::ArrayItemNotUnique,
                        });
                        return;
                    }
                }
                unique_items.push(item);
            }
        }
    }
}


impl<'schema> SchemaBase for ArraySchema<'schema> {
    fn validate<'json>(&self, value: &'json JsonValue, errors: &mut Vec<ValidationError<'json>>) {
        match value {
            &JsonValue::Array(ref array) => {
                self.validate_size(array, value, errors);
                self.validate_all_items_schema(array, errors);
                self.validate_unique(array, value, errors);
            }
            val => {
                errors.push(ValidationError {
                    reason: ErrorReason::TypeMismatch {
                        expected: JsonType::Array,
                        found: val.get_type(),
                    },
                    node: value,
                })
            }
        }
    }
}

#[derive(Debug)]
pub struct ArraySchemaBuilder<'schema> {
    description: Option<&'schema str>,
    id: Option<&'schema str>,
    title: Option<&'schema str>,

    min_items: Option<usize>,
    max_items: Option<usize>,
    unique_items: bool,

    all_items_schema: Box<Option<Schema<'schema>>>,
    item_schemas: Option<Vec<Schema<'schema>>>,

    additional_items: bool,
}

impl<'schema> Default for ArraySchemaBuilder<'schema> {
    fn default() -> ArraySchemaBuilder<'schema> {
        ArraySchemaBuilder {
            description: None,
            id: None,
            title: None,

            min_items: None,
            max_items: None,
            unique_items: false,

            all_items_schema: Default::default(),
            item_schemas: Default::default(),

            additional_items: true,
        }
    }
}

impl<'schema> ArraySchemaBuilder<'schema> {
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

    pub fn min_items<V: Into<usize>>(mut self, value: V) -> Self {
        self.min_items = Some(value.into());
        self
    }

    pub fn max_items<V: Into<usize>>(mut self, value: V) -> Self {
        self.max_items = Some(value.into());
        self
    }

    pub fn unique_items<V: Into<bool>>(mut self, value: V) -> Self {
        self.unique_items = value.into();
        self
    }

    pub fn all_items_schema<V: Into<Schema<'schema>>>(mut self, value: V) -> Self {
        self.all_items_schema = Box::new(Some(value.into()));
        self
    }

    pub fn item_schemas<V: Into<Vec<Schema<'schema>>>>(mut self, value: V) -> Self {
        self.item_schemas = Some(value.into());
        self
    }

    pub fn additional_items<V: Into<bool>>(mut self, value: V) -> Self {
        self.additional_items = value.into();
        self
    }

    pub fn build(self) -> ArraySchema<'schema> {
        ArraySchema {
            description: self.description,
            id: self.id,
            title: self.title,

            min_items: self.min_items,
            max_items: self.max_items,
            unique_items: self.unique_items,

            all_items_schema: self.all_items_schema,
            item_schemas: self.item_schemas,

            additional_items: self.additional_items,
        }
    }
}

#[cfg(test)]
mod tests {
    use json;
    use super::*;
    use super::super::NumberSchemaBuilder;
    use errors::*;

    #[test]
    fn unique_elements() {
        let schema = ArraySchemaBuilder::default()
            .unique_items(true)
            .build();
        let input = json::parse("[1, 1, 2, 3, 4]").unwrap();
        let mut errors = vec![];
        schema.validate(&input, &mut errors);
        println!("{:?}", errors);
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].reason, ErrorReason::ArrayItemNotUnique);
    }

    #[test]
    fn default_schema() {
        let schema = ArraySchemaBuilder::default().build();
        let input = json::parse(r#"[1, "a", "b", {"test": 123}, []]"#).unwrap();
        let mut errors = vec![];
        schema.validate(&input, &mut errors);
        println!("{:?}", errors);
        assert_eq!(errors.len(), 0)
    }

    #[test]
    fn subschema() {
        let input = json::parse(r#"[1.2, 1.4, 1.9, 2.5]"#).unwrap();
        let item_schema = NumberSchemaBuilder::default()
            .minimum(1.0)
            .maximum(2.0)
            .build();
        let schema = ArraySchemaBuilder::default()
            .all_items_schema(item_schema)
            .build();
        let mut errors = vec![];
        schema.validate(&input, &mut errors);
        assert_eq!(errors.len(), 0);
    }
}
