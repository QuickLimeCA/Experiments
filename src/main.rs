extern crate clap;
extern crate openssl;
use clap::{Arg, App};
use openssl::error::ErrorStack;
use openssl::nid;
use openssl::string::OpensslString;
use openssl::x509::X509;

use std::io;
use std::io::Read;
use std::fs::File;

fn main() {
    let args = App::new("X.509 Parse")
        .about("Experiment in parsing X.509 certificates")
        .arg(Arg::with_name("InputFile")
            .long("file")
            .short("f")
            .takes_value(true)
            .required(true))
        .get_matches();

    let input_file_path = args.value_of("InputFile").unwrap();
    let cert_bytes = get_cert_bytes(input_file_path).unwrap();
    let cert = X509::from_pem(&cert_bytes).unwrap();
    let cert_common_names = get_subject_name(&cert);
    for cn in cert_common_names {
        match cn {
            Ok(cn) => println!("{:?}", cn),
            Err(err) => println!("{:?}", err)
        }
    }
}

fn get_cert_bytes(input_file_path: &str) -> Result<Vec<u8>, io::Error> {
    let mut input_data = Vec::new();

    File::open(input_file_path)?.read_to_end(&mut input_data)?;

    Ok(input_data)
}

fn get_subject_name(cert: &X509) -> Vec<Result<OpensslString, ErrorStack>> {
    let entries = cert.subject_name();
    let common_name_entries = entries.entries_by_nid(nid::COMMONNAME);
    common_name_entries.map(|x| (*x).data().as_utf8()).collect()
}