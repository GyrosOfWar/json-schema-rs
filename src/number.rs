use serde_json::Value;

use util::{JsonType, JsonValueExt};
use errors::{ValidationError, ErrorKind};
use schema::{SchemaBase, Context, Schema};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct NumberSchema {
    description: Option<String>,
    id: Option<String>,
    title: Option<String>,

    multiple_of: Option<f64>,
    minimum: Option<f64>,
    maximum: Option<f64>,
    exclusive_minimum: Option<bool>,
    exclusive_maximum: Option<bool>,
}

impl NumberSchema {
    fn exclusive_maximum(&self) -> bool {
        self.exclusive_maximum.unwrap_or(false)
    }

    fn exclusive_minimum(&self) -> bool {
        self.exclusive_minimum.unwrap_or(false)
    }

    fn validate_range<'json>(&self,
                             node: &'json Value,
                             value: f64,
                             errors: &mut Vec<ValidationError<'json>>) {
        let mut bound = None;
        if let Some(min) = self.minimum {
            let out_of_bounds = if self.exclusive_minimum() {
                value < min
            } else {
                value <= min
            };
            if out_of_bounds {
                bound = Some(min);
            }
        }

        if let Some(max) = self.maximum {
            let out_of_bounds = if self.exclusive_maximum() {
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
                            reason: ErrorKind::NumberRange {
                                bound: b,
                                value: value,
                            },
                            node: node,
                        })
        }
    }
}

impl SchemaBase for NumberSchema {
    #[doc(hidden)]
    fn validate_inner<'json>(&self,
                             ctx: &Context,
                             value: &'json Value,
                             errors: &mut Vec<ValidationError<'json>>) {
        if let Value::Number(_) = *value {
            self.validate_range(value, value.as_f64().unwrap(), errors);
        } else {
            errors.push(ValidationError {
                            reason: ErrorKind::TypeMismatch {
                                expected: JsonType::Number,
                                found: value.get_type(),
                            },
                            node: value,
                        })
        }
    }
}

#[derive(Default, Debug)]
pub struct NumberSchemaBuilder {
    description: Option<String>,
    id: Option<String>,
    title: Option<String>,

    multiple_of: Option<f64>,
    minimum: Option<f64>,
    maximum: Option<f64>,
    exclusive_minimum: bool,
    exclusive_maximum: bool,
}

impl NumberSchemaBuilder {
    pub fn minimum(mut self, value: f64) -> Self {
        self.minimum = Some(value);
        self
    }

    pub fn maximum(mut self, value: f64) -> Self {
        self.maximum = Some(value);
        self
    }

    pub fn build(self) -> Schema {
        From::from(NumberSchema {
                       description: self.description,
                       id: self.id,
                       title: self.title,

                       multiple_of: self.multiple_of,
                       minimum: self.minimum,
                       maximum: self.maximum,
                       exclusive_minimum: Some(self.exclusive_minimum),
                       exclusive_maximum: Some(self.exclusive_maximum),
                   })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn range() {}
}
