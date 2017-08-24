extern crate json_schema;
extern crate serde_json;

use std::str::FromStr;

use json_schema::Schema;

const SCHEMA: &str = r#"
{
    "type": "array",
    "items": {
        "type": "object",
        "additionalProperties": false,
        "properties": {
            "x": {"type": "number", "minimum": 0.0, "maximum": 100.0},
            "y": {"type": "number", "minimum": 0.0, "maximum": 100.0},
            "z": {"type": "number", "minimum": 0.0, "maximum": 100.0}
        }
    }
}
"#;

const VALUE: &str = r#"
[
    {"x": 99.2, "y": 0.1, "z": 21.9},
    {"x": 20.4, "y": 30.1, "z": 11.2},
    {"x": 30.9, "y": 3.2, "z": 26.2}
]
"#;

const VALUE_WITH_ERRORS: &str = r#"
[
    {"x": 99.2, "y": 0.1, "z": 21.9},
    {"x": 20.4, "y": 30.1, "z": 11.2},
    {"x": 30.9, "y": 3.2, "z": 100.2}
]
"#;

fn main() {
    let schema = Schema::from_str(SCHEMA).unwrap();
    let value = serde_json::from_str(VALUE).unwrap();
    match schema.validate(&value) {
        Ok(_) => println!("No errors!"),
        Err(e) => println!("Errors validating JSON: {}", e),
    }

    let value_with_errors = serde_json::from_str(VALUE_WITH_ERRORS).unwrap();
    match schema.validate(&value_with_errors) {
        Ok(_) => println!("No errors!"),
        Err(e) => println!("Errors validating JSON: {}", e),
    }
}
