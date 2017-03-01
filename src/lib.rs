extern crate json;
#[macro_use] extern crate error_chain;

mod errors;

pub type ValidationResult = Result<(), ValidationError>;

pub struct ValidationError {

}

pub trait SchemaBase {
    fn validate(&self, value: &json::JsonValue) -> ValidationResult;
}

pub enum Schema<'a> {
    Null,
    Boolean(BooleanSchema<'a>),
    Object(ObjectSchema<'a>),
    Array(ArraySchema<'a>),
    Number(NumberSchema),
    String(StringSchema),
    Integer(IntegerSchema),
}

pub struct BooleanSchema<'a> {
    description: Option<&'a str>,
    id: Option<&'a str>,
    title: &'a str,
}

impl<'a> SchemaBase for BooleanSchema<'a> {
    fn validate(&self, value: &json::JsonValue) -> ValidationResult {
        if value.is_boolean() {
            Ok(())
        } else {
            Err(ValidationError {
                // TOOD
            })
        }
    }
}

pub struct ObjectSchema<'a> {
    
}

pub struct ArraySchema<'a> {
    min_items: Option<usize>,
    max_items: Option<usize>,
    unique_items: bool,
    all_items_schema: Box<Option<Schema<'a>>>,
    item_schemas: Option<Vec<Schema<'a>>>,
}

pub struct NumberSchema;

pub struct StringSchema;

pub struct IntegerSchema;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
