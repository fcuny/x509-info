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

    /// Accept invalid certificate
    #[clap(short, long, default_value_t = false)]
    insecure: bool,

    #[clap(short, long,value_enum, default_value_t = output::OutputFormat::Short)]
    format: output::OutputFormat,
}

fn main() {
    let args = Args::parse();

    match client::get_certificates(args.domain, args.port, args.insecure) {
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
