use json::JsonValue;

pub trait JsonValueExt {
    fn get_type(&self) -> JsonType;
}

impl JsonValueExt for JsonValue {
    fn get_type(&self) -> JsonType {
        match *self {
            JsonValue::Boolean(_) => JsonType::Boolean,
            JsonValue::Array(_) => JsonType::Array,
            JsonValue::Null => JsonType::Null,
            JsonValue::String(_) |
            JsonValue::Short(_) => JsonType::String,
            JsonValue::Object(_) => JsonType::Object,
            JsonValue::Number(_) => {
                let n = self.as_f64().unwrap();
                if n.trunc() == n {
                    JsonType::Integer
                } else {
                    JsonType::Number
                }
            }
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
