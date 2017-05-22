use regex::Regex;

use string::Format;

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Schema {
    #[serde(rename = "number")]
    Number(NumberSchema),
    #[serde(rename = "string")]
    String(StringSchema)
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NumberSchema {
    pub description: Option<String>,
    pub id: Option<String>,
    pub title: Option<String>,

    pub multiple_of: Option<f64>,
    pub minimum: Option<f64>,
    pub maximum: Option<f64>,
    pub exclusive_minimum: Option<bool>,
    pub exclusive_maximum: Option<bool>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StringSchema {
    description: Option<String>,
    id: Option<String>,
    title: Option<String>,

    min_length: Option<usize>,
    max_length: Option<usize>,
    #[serde(with = "regex_serde")]
    pattern: Option<Regex>,
    format: Option<Format>,
}

#[cfg(test)]
mod tests {
    use serde_json;
    use super::Schema;

    #[test]
    fn parse_number() {
        let schema: Schema = serde_json::from_str(r#"{
            "type": "number",
            "minimum": 0,
            "maximum": 100,
            "exclusiveMaximum": true
        }"#).unwrap();
        let schema = match schema { 
            Schema::Number(n) => n,
            _ => panic!("Not a number schema")
        };
        assert_eq!(schema.maximum.unwrap(), 100.0);
        assert_eq!(schema.minimum.unwrap(), 0.0);
        assert_eq!(schema.exclusive_maximum, Some(true));
    }
}

mod regex_serde {
    use serde::{self, Deserialize, Serializer, Deserializer};
    use regex::Regex;

    pub fn serialize<S>(regex: &Option<Regex>, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        match *regex {
            Some(ref r) => serializer.serialize_str(&format!("{}", r.as_str())),
            None => serializer.serialize_none()
        }
    }


    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Regex>, D::Error>
        where D: Deserializer<'de>
    {
        let s = String::deserialize(deserializer)?;
        Regex::new(&s).map_err(serde::de::Error::custom).map(Some)
    }
}