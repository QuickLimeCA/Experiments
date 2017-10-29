extern crate chrono;
extern crate clap;
extern crate openssl;
extern crate helperlib;

use chrono::Utc;
use clap::{Arg, App};
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
    let cert_bytes = helperlib::get_cert_bytes(input_file_path).unwrap();
    let cert = X509::from_pem(&cert_bytes).unwrap();
    
    println!("------------COMMON NAMES------------");
    let cert_common_names = helperlib::get_subject_name(&cert);
    for cn in cert_common_names {
        match cn {
            Ok(cn) => println!("{:?}", cn),
            Err(err) => println!("{:?}", err)
        }
    }

    println!("------------SUBJECT ALT NAMES------------");
    let cert_subject_alt_names = helperlib::get_subject_alt_names(&cert);
    match cert_subject_alt_names {
        None => println!("{:?}", "No SANs listed in certificate".to_string()),
        Some(sans) => {
            for san in sans {
                println!("{:?}", san)
            }
        }
    }

    println!("------------Validity Dates------------");
    let validity_dates = helperlib::get_validity_dates(&cert);
    println!("Not before: {}", validity_dates.0);
    println!("Not after: {}", validity_dates.1);

    let not_before = helperlib::convert_Asn1TimeRef_to_DateTimeUTC(validity_dates.0);
    match not_before {
        Err(e) => println!("Error parsing not_before date: {}", e),
        Ok(not_before) => println!("Not before: {:?}", not_before)
    }

    let not_after = helperlib::convert_Asn1TimeRef_to_DateTimeUTC(validity_dates.1);
    match not_after {
        Err(e) => println!("Error parsing not_after date: {}", e),
        Ok(not_after) => println!("Not after: {:?}", not_after)
    }

    if let Ok(not_before) = not_before {
        if let Ok(not_after) = not_after {
            let now = Utc::now();
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

