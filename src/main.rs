extern crate webpki_roots;

mod client;
mod output;

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

    #[clap(short, long,value_enum, default_value_t = output::OutputFormat::Short)]
    format: output::OutputFormat,
}

fn main() {
    let args = Args::parse();

    let domain = args.domain;

    let certs = client::get_certificates(domain, args.port);

    match certs {
        Ok(certs) => {
            let (_, cert) =
                x509_parser::certificate::X509Certificate::from_der(certs[0].as_ref()).unwrap();
            output::OutputFormat::print(args.format, cert);
        }
        Err(e) => {
            println!("error: {}", e);
            std::process::exit(1);
        }
    };
}
