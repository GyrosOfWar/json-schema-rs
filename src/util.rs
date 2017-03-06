use json::JsonValue;

fn parse_index(s: &str) -> Option<usize> {
    if s.starts_with('+') || (s.starts_with('0') && s.len() != 1) {
        return None;
    }
    s.parse().ok()
}

pub trait JsonValueExt {
    fn get_type(&self) -> JsonType;
    fn pointer<'a>(&'a self, pointer: &str) -> Option<&'a JsonValue>;
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
    // Stolen from https://docs.rs/serde_json/0.9.8/src/serde_json/value.rs.html#131-463
    fn pointer<'a>(&'a self, pointer: &str) -> Option<&'a JsonValue> {
        if pointer == "" {
            return Some(self);
        }
        if !pointer.starts_with('/') {
            return None;
        }

        let tokens = pointer.split('/').skip(1).map(|x| x.replace("~1", "/").replace("~0", "~"));
        let mut target = self;

        for token in tokens {
            let target_opt = match *target {
                JsonValue::Object(ref obj) => obj.get(&token),
                JsonValue::Array(ref list) => parse_index(&token).and_then(|x| list.get(x)),
                _ => return None,
            };
            if let Some(t) = target_opt {
                target = t;
            } else {
                return None;
            }
        }
        Some(target)
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


#[cfg(test)]
mod tests {
    use json;
    use super::JsonValueExt;

    #[test]
    fn pointer_test() {
        let json_str = r#"
        {
            "user": {
                "id": 123,
                "name": "user",
                "friends": [1, 2]
            },
            "tags": ["test", "1", "2"],
            "coordinates": [[1.0, 2.0], [4.0, 2.0]]
        }
        "#;

        let value = json::parse(json_str).unwrap();
        let id = value.pointer("/user/id").unwrap().as_u64().unwrap();
        assert_eq!(id, 123);
        let name = value.pointer("/user/name").unwrap().as_str().unwrap();
        assert_eq!(name, "user");
    }
}