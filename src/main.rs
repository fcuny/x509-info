extern crate webpki_roots;

mod client;

use chrono::TimeZone as _;
use clap::Parser;
use x509_parser::prelude::*;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Domain to check
    domain: String,

    /// Port to check
    #[clap(short, long, default_value_t = 443)]
    port: u16,
}

fn main() {
    let args = Args::parse();

    let domain = args.domain;

    let certs = client::get_certificates(domain, args.port);

    match certs {
        Ok(certs) => {
            let (_, cert) =
                x509_parser::certificate::X509Certificate::from_der(certs[0].as_ref()).unwrap();
            pretty_print(cert);
        }
        Err(e) => {
            println!("error: {}", e);
            std::process::exit(1);
        }
    };
}

fn pretty_print(cert: X509Certificate) {
    println!(
        "\tSubject: CN={} O={} L={}",
        cert.subject()
            .iter_common_name()
            .next()
            .and_then(|cn| cn.as_str().ok())
            .unwrap(),
        cert.subject()
            .iter_organization()
            .next()
            .and_then(|o| o.as_str().ok())
            .unwrap_or_default(),
        cert.subject()
            .iter_locality()
            .next()
            .and_then(|l| l.as_str().ok())
            .unwrap_or_default(),
    );
    println!(
        "\tIssuer:  CN={} O={} L={}",
        cert.issuer()
            .iter_common_name()
            .next()
            .and_then(|cn| cn.as_str().ok())
            .unwrap(),
        cert.issuer()
            .iter_organization()
            .next()
            .and_then(|o| o.as_str().ok())
            .unwrap_or_default(),
        cert.issuer()
            .iter_locality()
            .next()
            .and_then(|l| l.as_str().ok())
            .unwrap_or_default(),
    );

    let not_before = chrono::Local
        .timestamp(cert.validity().not_before.timestamp(), 0)
        .to_rfc3339();

    let not_after = chrono::Local
        .timestamp(cert.validity().not_after.timestamp(), 0)
        .to_rfc3339();

    if let Some(subnames) = subject_alternative_name(cert) {
        let dns_names = subnames.join(", ");
        println!("\tDNS Names: {}", dns_names);
    }

    println!("\tValidity Period");
    println!("\t\tNot before: {}", not_before);
    println!("\t\tNot After:  {}", not_after);
}

fn subject_alternative_name(cert: X509Certificate) -> Option<Vec<String>> {
    let mut subnames = Vec::new();
    if let Ok(Some(san)) = cert.subject_alternative_name() {
        let san = san.value;
        for name in &san.general_names {
            if let GeneralName::DNSName(name) = name {
                subnames.push(name.to_string());
            }
        }
    }
    Some(subnames)
}
