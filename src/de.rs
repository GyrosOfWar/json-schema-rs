#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Schema {
    #[serde(rename = "number")]
    Number(Number),
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Number {
    description: Option<String>,
    id: Option<String>,
    title: Option<String>,

    multiple_of: Option<f64>,
    minimum: Option<f64>,
    maximum: Option<f64>,
    exclusive_minimum: Option<bool>,
    exclusive_maximum: Option<bool>,
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
            Schema::Number(n) => n
        };
        assert_eq!(schema.maximum.unwrap(), 100.0);
        assert_eq!(schema.minimum.unwrap(), 0.0);
        assert_eq!(schema.exclusive_maximum, Some(true));
    }

}