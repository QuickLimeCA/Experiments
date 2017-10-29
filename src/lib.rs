use std::io;
use std::io::Read;
use std::fs::File;

pub fn get_cert_bytes(input_file_path: &str) -> Result<Vec<u8>, io::Error> {
    let mut input_data = Vec::new();

    File::open(input_file_path)?.read_to_end(&mut input_data)?;

    Ok(input_data)
}