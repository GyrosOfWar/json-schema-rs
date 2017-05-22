# json-schema-rs
A JSON schema validation library in Rust.
Usage:

```rust
use json_schema;
use serde_json;

// Simplest case: read both the JSON file and the schema from disk.
if let Err(errors) = json_schema::validate("path-to-json-file", "path-to-schema") {
    // Do stuff
}

// Parse the schema and the JSON separately
let schema = json_schema::parse_schema("path-to-schema").unwrap();
let json = serde_json::from_str(r#"{"key": "value"}"#);
if let Err(errors) = schema.validate(&json) {
    // Do stuff
}

// TODO example for Builder to construct schemas programatically
```
