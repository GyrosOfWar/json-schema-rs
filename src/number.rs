use json::JsonValue;

use super::{JsonType, JsonValueExt};
use errors::{ValidationError, ErrorReason};
use schema::{Schema, SchemaBase};

#[derive(Clone, Debug)]
pub struct NumberSchema<'schema> {
    description: Option<&'schema str>,
    id: Option<&'schema str>,
    title: Option<&'schema str>,

    multiple_of: Option<f64>,
    minimum: Option<f64>,
    maximum: Option<f64>,
    exclusive_minimum: bool,
    exclusive_maximum: bool,
}

impl<'schema> NumberSchema<'schema> {
    fn validate_range<'json>(&self,
                             node: &'json JsonValue,
                             value: f64,
                             errors: &mut Vec<ValidationError<'json>>) {
        let mut bound = None;
        if let Some(min) = self.minimum {
            let out_of_bounds = if self.exclusive_minimum {
                value < min
            } else {
                value <= min
            };
            if out_of_bounds {
                bound = Some(min);
            }
        }

        if let Some(max) = self.maximum {
            let out_of_bounds = if self.exclusive_maximum {
                value > max
            } else {
                value >= max
            };
            if out_of_bounds {
                bound = Some(max);
            }
        }

        if let Some(b) = bound {
            errors.push(ValidationError {
                reason: ErrorReason::NumberRange {
                    bound: b,
                    value: value,
                },
                node: node,
            })
        }
    }
}

impl<'schema> SchemaBase for NumberSchema<'schema> {
    fn validate_inner<'json>(&self,
                             value: &'json JsonValue,
                             errors: &mut Vec<ValidationError<'json>>) {
        if let &JsonValue::Number(_) = value {
            self.validate_range(value, value.as_f64().unwrap(), errors);
        } else {
            errors.push(ValidationError {
                reason: ErrorReason::TypeMismatch {
                    expected: JsonType::Number,
                    found: value.get_type(),
                },
                node: value,
            })
        }
    }
}

#[derive(Default)]
pub struct NumberSchemaBuilder<'schema> {
    description: Option<&'schema str>,
    id: Option<&'schema str>,
    title: Option<&'schema str>,

    multiple_of: Option<f64>,
    minimum: Option<f64>,
    maximum: Option<f64>,
    exclusive_minimum: bool,
    exclusive_maximum: bool,
}

impl<'schema> NumberSchemaBuilder<'schema> {
    pub fn minimum<V: Into<f64>>(mut self, value: V) -> Self {
        self.minimum = Some(value.into());
        self
    }

    pub fn maximum<V: Into<f64>>(mut self, value: V) -> Self {
        self.maximum = Some(value.into());
        self
    }

    pub fn build(self) -> Schema<'schema> {
        From::from(NumberSchema {
            description: self.description,
            id: self.id,
            title: self.title,

            multiple_of: self.multiple_of,
            minimum: self.minimum,
            maximum: self.maximum,
            exclusive_minimum: self.exclusive_minimum,
            exclusive_maximum: self.exclusive_maximum,
        })
    }
}
