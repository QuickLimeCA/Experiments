extern crate openssl;

use openssl::error::ErrorStack;
use openssl::nid;
use openssl::string::OpensslString;
use openssl::x509::X509;

use std::io;
use std::io::Read;
use std::fs::File;

pub fn get_cert_bytes(input_file_path: &str) -> Result<Vec<u8>, io::Error> {
    let mut input_data = Vec::new();

    File::open(input_file_path)?.read_to_end(&mut input_data)?;

    Ok(input_data)
}

pub fn get_subject_name(cert: &X509) -> Vec<Result<OpensslString, ErrorStack>> {
    let entries = cert.subject_name();
    let common_name_entries = entries.entries_by_nid(nid::COMMONNAME);
    common_name_entries.map(|x| (*x).data().as_utf8()).collect()
}