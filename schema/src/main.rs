extern crate clap;
extern crate serde;
extern crate serde_json;
extern crate valico;

use clap::{Arg, App};
use serde_json::{Value, from_str};
use valico::json_schema;

use std::io;
use std::io::Read;
use std::fs::File;
use std::path::Path;

fn main() {
    let args = App::new("Schema Validator")
        .about("Experiment in validating a JSON Schema using Valico")
        .arg(Arg::with_name("InputJson")
            .long("json")
            .short("j")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("InputSchema")
            .long("schema")
            .short("s")
            .takes_value(true)
            .required(true))
        .get_matches();

    let input_json_path = Path::new(args.value_of("InputJson").unwrap());
    let input_json_data = read_file(&input_json_path).unwrap();

    let input_schema_path = Path::new(args.value_of("InputSchema").unwrap());
    let input_schema_data = read_file(&input_schema_path).unwrap();

    let schema: Value = from_str(&input_schema_data).unwrap();
    let data: Value = from_str(&input_json_data).unwrap();
    let mut scope = json_schema::Scope::new();

    let schema_compiled = match scope.compile_and_return(schema.clone(), false) {
        Ok(schema) => schema,
        Err(err) => panic!("Error in schema {:?}", err)
    };

    let state = schema_compiled.validate(&data["data"]);

    println!("Is JSON Valid? {:?}", state.is_valid());
    if state.is_valid() == false {
        for err in state.errors {
            println!("");
            if err.get_detail().is_some() == true {
                println!("Where: {}\nWhat: {}\nWhy: {}", err.get_path(), err.get_title(), err.get_detail().unwrap());
            } else {
                println!("Where: {}\nWhat: {}", err.get_path(), err.get_title());
            }
        }
    }
}

fn read_file(input_file_path: &Path) -> Result<String, io::Error> {
    let mut contents = String::new();
    File::open(input_file_path)?.read_to_string(&mut contents)?;
    Ok(contents)
}