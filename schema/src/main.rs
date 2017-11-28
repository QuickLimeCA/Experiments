extern crate clap;

use clap::{Arg, App};

use std::io;
use std::io::Read;
use std::fs::File;

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

    let input_json_path = args.value_of("InputJson").unwrap();
    let input_json_data = read_file(&input_json_path).unwrap();

    let input_schema_path = args.value_of("InputSchema").unwrap();
    let input_schema_data = read_file(&input_schema_path).unwrap();

    println!("JSON: {:?}", input_json_data);
    println!("Schema: {:?}", input_schema_data);
}

fn read_file(input_file_path: &str) -> Result<String, io::Error> {
    let mut contents = String::new();
    File::open(input_file_path)?.read_to_string(&mut contents)?;
    Ok(contents)
}