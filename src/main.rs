extern crate clap;
extern crate serde_json;
extern crate json_schema;

use std::fs::File;

use clap::{App, Arg};
use json_schema::{Schema, SchemaBase};
use json_schema::errors::Result;

fn run() -> Result<()> {
    let matches = App::new("json_schema")
        .about("JSON schema validator")
        .author("Martin Tomasi <martin.tomasi@gmail.com>")
        .arg(Arg::with_name("schema")
            .short("s")
            .long("schema")
            .help("Path to the schema file")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("input")
            .short("i")
            .long("input")
            .help("One or more input files")
            .takes_value(true)
            .multiple(true)
            .min_values(1))
        .get_matches();
    let schema_path = matches.value_of("schema").unwrap();
    let schema: Schema = serde_json::from_reader(File::open(schema_path)?)?;

    for json_path in matches.values_of("input").unwrap() {
        let json = serde_json::from_reader(File::open(json_path)?)?;
        match schema.validate(&json) {
            Ok(_) => println!("{} validated successfully", json_path),
            Err(e) => println!("{} has errors:\n{}", json_path, e)
        }
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        println!("Error: {}", e);
    }
}