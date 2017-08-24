use serde_json::Value;

use util::{JsonType, JsonValueExt};
use errors::{ValidationError, ErrorKind};
use schema::{Schema, SchemaBase, Context};

/// Schema for JSON arrays like `[1, 2, 3]`.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct ArraySchema {
    description: Option<String>,
    id: Option<String>,
    title: Option<String>,

    min_items: Option<usize>,
    max_items: Option<usize>,
    unique_items: Option<bool>,

    items: Option<Items>,

    additional_items: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum Items {
    List(Box<Schema>),
    Tuple(Vec<Schema>),
}

impl ArraySchema {
    fn additional_items(&self) -> bool {
        self.additional_items.unwrap_or(false)
    }

    fn unique_items(&self) -> bool {
        self.unique_items.unwrap_or(false)
    }

    fn validate_size<'json>(
        &self,
        array: &'json [Value],
        parent: &'json Value,
        errors: &mut Vec<ValidationError<'json>>,
    ) {
        if let Some(min) = self.min_items {
            if array.len() < min {
                errors.push(ValidationError {
                    reason: ErrorKind::MinLength {
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
                    reason: ErrorKind::MaxLength {
                        expected: max,
                        found: array.len(),
                    },
                    node: parent,
                });
            }
        }
    }

    fn validate_items<'json>(
        &self,
        ctx: &Context,
        array: &'json [Value],
        parent: &'json Value,
        errors: &mut Vec<ValidationError<'json>>,
    ) {
        if let Some(ref items) = self.items {
            match *items {
                Items::Tuple(ref schemas) => {
                    if schemas.len() != array.len() && !self.additional_items() {
                        errors.push(ValidationError {
                            reason: ErrorKind::TupleLengthMismatch {
                                schemas: schemas.len(),
                                tuple: array.len(),
                            },
                            node: parent,
                        });
                    }

                    for (schema, value) in schemas.iter().zip(array) {
                        schema.validate_inner(ctx, value, errors);
                    }
                }
                Items::List(ref schema) => {
                    for value in array {
                        schema.validate_inner(ctx, value, errors);
                    }
                }
            }
        }
    }

    fn validate_unique<'json>(
        &self,
        array: &'json [Value],
        parent: &'json Value,
        errors: &mut Vec<ValidationError<'json>>,
    ) {
        if self.unique_items() {
            let mut unique_items = vec![];
            for item in array {
                for contained in &unique_items {
                    if *contained == item {
                        errors.push(ValidationError {
                            node: parent,
                            reason: ErrorKind::ArrayItemNotUnique,
                        });
                        return;
                    }
                }
                unique_items.push(item);
            }
        }
    }
}


impl SchemaBase for ArraySchema {
    #[doc(hidden)]
    fn validate_inner<'json>(
        &self,
        ctx: &Context,
        value: &'json Value,
        errors: &mut Vec<ValidationError<'json>>,
    ) {
        match value {
            &Value::Array(ref array) => {
                self.validate_size(array, value, errors);
                self.validate_items(ctx, array, value, errors);
                self.validate_unique(array, value, errors);
            }
            val => {
                errors.push(ValidationError::type_mismatch(
                    val,
                    JsonType::Array,
                    val.get_type(),
                ))
            }
        }
    }
}

/// A builder for creating array schemas programatically.
#[derive(Debug)]
pub struct ArraySchemaBuilder {
    description: Option<String>,
    id: Option<String>,
    title: Option<String>,

    min_items: Option<usize>,
    max_items: Option<usize>,
    unique_items: bool,

    items: Option<Items>,
    additional_items: bool,
}

impl Default for ArraySchemaBuilder {
    fn default() -> ArraySchemaBuilder {
        ArraySchemaBuilder {
            description: None,
            id: None,
            title: None,

            min_items: None,
            max_items: None,
            unique_items: false,
            items: Default::default(),

            additional_items: true,
        }
    }
}

impl ArraySchemaBuilder {
    /// Sets the description.
    pub fn description<V: Into<String>>(mut self, value: V) -> Self {
        self.description = Some(value.into());
        self
    }

    /// Sets the ID.
    pub fn id<V: Into<String>>(mut self, value: V) -> Self {
        self.id = Some(value.into());
        self
    }
    /// Sets the title.
    pub fn title<V: Into<String>>(mut self, value: V) -> Self {
        self.title = Some(value.into());
        self
    }
    /// Set the minimum number of items this array must have.
    pub fn min_items(mut self, value: usize) -> Self {
        self.min_items = Some(value);
        self
    }
    /// Set the maximum number of items this array may have.
    pub fn max_items(mut self, value: usize) -> Self {
        self.max_items = Some(value);
        self
    }
    /// Make it so array items have to be unique.
    pub fn unique_items(mut self) -> Self {
        self.unique_items = true;
        self
    }
    /// Set a schema that every item must conform to. (list validation)
    pub fn all_items_schema<V: Into<Schema>>(mut self, value: V) -> Self {
        self.items = Some(Items::List(Box::new(value.into())));
        self
    }
    /// Set a list of schemas that each item must conform to. (tuple validation)
    pub fn item_schemas<V: Into<Vec<Schema>>>(mut self, value: V) -> Self {
        self.items = Some(Items::Tuple(value.into()));
        self
    }
    /// Set whether additional items are allowed (tuple validation).
    pub fn additional_items(mut self, value: bool) -> Self {
        self.additional_items = value;
        self
    }
    /// Returns the finished `Schema`.
    pub fn build(self) -> Schema {
        From::from(ArraySchema {
            description: self.description,
            id: self.id,
            title: self.title,

            min_items: self.min_items,
            max_items: self.max_items,
            unique_items: Some(self.unique_items),

            items: self.items,
            additional_items: Some(self.additional_items),
        })
    }
}

#[cfg(test)]
mod tests {
    use serde_json;

    use super::*;
    use errors::ErrorKind;
    use number::NumberSchemaBuilder;

    #[test]
    fn unique_elements() {
        let schema = ArraySchemaBuilder::default().unique_items().build();
        let input = serde_json::from_str("[1, 1, 2, 3, 4]").unwrap();
        let errors = schema.validate(&input).unwrap_err().0;
        assert_eq!(errors.len(), 1);
        if let ErrorKind::ArrayItemNotUnique = errors[0].reason {

        } else {
            assert!(false, "Wrong error reason");
        }
    }

    #[test]
    fn default_schema() {
        let schema = ArraySchemaBuilder::default().build();
        let input = serde_json::from_str(r#"[1, "a", "b", {"test": 123}, []]"#).unwrap();
        let result = schema.validate(&input);
        assert!(result.is_ok());
    }

    #[test]
    fn subschema() {
        let input = serde_json::from_str(r#"[[], 1.2, 1.4, 1.9, 2.5]"#).unwrap();
        let item_schema = NumberSchemaBuilder::default()
            .minimum(1.0)
            .maximum(2.0)
            .build();
        let schema = ArraySchemaBuilder::default()
            .all_items_schema(item_schema)
            .build();
        let errors = schema.validate(&input).unwrap_err().0;
        assert_eq!(errors.len(), 2);
        assert_eq!(*errors[0].node, input[0]);
        if let ErrorKind::NumberRange { value, bound } = errors[1].reason {
            assert_eq!(value, 2.5);
            assert_eq!(bound, 2.0);
        } else {
            assert!(false, "Wrong property");
        }
    }
}
