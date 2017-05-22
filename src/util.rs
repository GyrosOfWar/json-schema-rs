use serde_json::Value;

pub trait JsonValueExt {
    fn get_type(&self) -> JsonType;
}

impl JsonValueExt for Value {
    fn get_type(&self) -> JsonType {
        match *self {
            Value::Null => JsonType::Null,
            Value::Bool(_) => JsonType::Boolean,
            Value::Number(ref n) => {
                if n.is_f64() {
                    JsonType::Number
                } else {
                    JsonType::Integer
                }
            },
            Value::Array(_) => JsonType::Array,
            Value::Object(_) => JsonType::Object,
            Value::String(_) => JsonType::String
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JsonType {
    Null,
    Boolean,
    Object,
    Array,
    Number,
    String,
    Integer,
}