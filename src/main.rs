extern crate chrono;
extern crate clap;
extern crate openssl;
extern crate x509parselib;

use chrono::prelude::*;
use clap::{Arg, App};
use openssl::asn1::Asn1TimeRef;
use openssl::x509::X509;

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
    let cert_bytes = x509parselib::get_cert_bytes(input_file_path).unwrap();
    let cert = X509::from_pem(&cert_bytes).unwrap();
    
    println!("------------COMMON NAMES------------");
    let cert_common_names = x509parselib::get_subject_name(&cert);
    for cn in cert_common_names {
        match cn {
            Ok(cn) => println!("{:?}", cn),
            Err(err) => println!("{:?}", err)
        }
    }

    println!("------------SUBJECT ALT NAMES------------");
    let cert_subject_alt_names = get_subject_alt_names(&cert);
    match cert_subject_alt_names {
        None => println!("{:?}", "No SANs listed in certificate".to_string()),
        Some(sans) => {
            for san in sans {
                println!("{:?}", san)
            }
        }
    }

    println!("------------Validity Dates------------");
    let validity_dates = get_validity_dates(&cert);
    println!("Not before: {}", validity_dates.0);
    println!("Not after: {}", validity_dates.1);

    let not_before = convert_Asn1TimeRef_to_DateTimeUTC(validity_dates.0);
    match not_before {
        Err(e) => println!("Error parsing not_before date: {}", e),
        Ok(not_before) => println!("Not before: {:?}", not_before)
    }

    let not_after = convert_Asn1TimeRef_to_DateTimeUTC(validity_dates.1);
    match not_after {
        Err(e) => println!("Error parsing not_after date: {}", e),
        Ok(not_after) => println!("Not after: {:?}", not_after)
    }

    if let Ok(not_before) = not_before {
        if let Ok(not_after) = not_after {
            let now = chrono::Utc::now();
            if not_before <= now && now <= not_after {
                println!("Certificate is between validity dates");
            } else if now < not_before {
                println!("Certificate not yet valid");
            } else {
                println!("Certificate expired");
            }
        }
    }
}



fn get_subject_alt_names(cert: & X509) -> Option<Vec<String>> {
    let entries = cert.subject_alt_names();
    match entries {
        None => None,
        Some(e) => Some(e.into_iter()
            .filter_map(|x| x.dnsname().and_then(|x| Some(x.to_owned())))
            .filter_map(Option::Some)
            .collect())
    }
}

fn get_validity_dates<'a>(cert: &'a X509) -> (&'a Asn1TimeRef, &'a Asn1TimeRef) {
    let not_before = cert.not_before();
    let not_after = cert.not_after();
    (not_before, not_after)
}

#[allow(non_snake_case)] //This func converts types, so type names are used in func name
fn convert_Asn1TimeRef_to_DateTimeUTC(input: & Asn1TimeRef) -> Result<chrono::DateTime<Utc>, chrono::ParseError> {
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