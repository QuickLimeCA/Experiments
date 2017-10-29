extern crate chrono;
extern crate openssl;

use chrono::prelude::*;

use openssl::asn1::Asn1TimeRef;
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

pub fn get_subject_alt_names(cert: & X509) -> Option<Vec<String>> {
    let entries = cert.subject_alt_names();
    match entries {
        None => None,
        Some(e) => Some(e.into_iter()
            .filter_map(|x| x.dnsname().and_then(|x| Some(x.to_owned())))
            .filter_map(Option::Some)
            .collect())
    }
}

pub fn get_validity_dates<'a>(cert: &'a X509) -> (&'a Asn1TimeRef, &'a Asn1TimeRef) {
    let not_before = cert.not_before();
    let not_after = cert.not_after();
    (not_before, not_after)
}

#[allow(non_snake_case)] //This func converts types, so type names are used in func name
pub fn convert_Asn1TimeRef_to_DateTimeUTC(input: & Asn1TimeRef) -> Result<chrono::DateTime<Utc>, chrono::ParseError> {
    use std::fmt::Write;

    let mut stringified = String::new();
    let result = write!(&mut stringified, "{}", input);
    if result.is_err() {
        panic!("Error writing Asn1TimeRef to string");
    }
    stringified = stringified.replace("GMT", "+00:00");

    let output = DateTime::parse_from_str(& stringified, "%b %d %H:%M:%S %Y %z")?;

    Ok(output.with_timezone(&Utc))
}