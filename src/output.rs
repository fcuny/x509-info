use chrono::prelude::*;
use std::net::{Ipv4Addr, Ipv6Addr};
use x509_parser::prelude::*;

/// How to format the output data.
#[derive(PartialEq, Eq, Debug, Copy, Clone, clap::ValueEnum)]
pub enum OutputFormat {
    /// Format the output as one line of plain text.
    Short,

    /// Format the output as plain text.
    Long,
}

impl OutputFormat {
    pub fn print(self, cert: X509Certificate) {
        match self {
            Self::Short => {
                self.short(cert);
            }
            Self::Long => {
                self.to_text(cert);
            }
        }
    }

    fn short(self, cert: X509Certificate) {
        let not_after = chrono::Local.timestamp(cert.validity().not_after.timestamp(), 0);
        let now: DateTime<Local> = Local::now();
        let remaining = not_after - now;

        if remaining >= chrono::Duration::zero() {
            println!(
                "{}: {} ({} days left)",
                cert.subject()
                    .iter_common_name()
                    .next()
                    .and_then(|cn| cn.as_str().ok())
                    .unwrap_or("<no name>"),
                not_after.to_rfc2822(),
                remaining.num_days(),
            );
        } else {
            println!(
                "{}: {} (it expired {} days ago)",
                cert.subject()
                    .iter_common_name()
                    .next()
                    .and_then(|cn| cn.as_str().ok())
                    .unwrap_or("<no name>"),
                not_after.to_rfc2822(),
                remaining.num_days(),
            );
        }
    }

    fn to_text(self, cert: X509Certificate) {
        let not_before = chrono::Local.timestamp(cert.validity().not_before.timestamp(), 0);
        let not_after = chrono::Local.timestamp(cert.validity().not_after.timestamp(), 0);
        let now: DateTime<Local> = Local::now();
        let remaining = not_after - now;
        let validity_duration = not_after - not_before;

        println!("certificate");
        println!(" version: {}", cert.version);
        println!(" serial: {}", cert.tbs_certificate.raw_serial_as_string());
        println!(" subject: {}", cert.subject());
        println!(" issuer: {}", cert.issuer());

        println!(" validity");
        println!("  not before    : {}", not_before.to_rfc2822());
        println!("  not after     : {}", not_after.to_rfc2822());
        println!("  validity days : {}", validity_duration.num_days());
        println!("  remaining days: {}", remaining.num_days());

        println!(" SANs:");
        if let Some(subnames) = subject_alternative_name(cert) {
            for name in subnames {
                println!("  {}", name);
            }
        }
    }
}

fn subject_alternative_name(cert: X509Certificate) -> Option<Vec<String>> {
    let mut subnames = Vec::new();
    if let Ok(Some(san)) = cert.subject_alternative_name() {
        let san = san.value;
        for name in &san.general_names {
            let s = match name {
                GeneralName::DNSName(s) => format!("DNS:{}", s),
                GeneralName::IPAddress(b) => {
                    let ip = match b.len() {
                        4 => {
                            let b = <[u8; 4]>::try_from(*b).unwrap();
                            let ip = Ipv4Addr::from(b);
                            format!("{}", ip)
                        }
                        16 => {
                            let b = <[u8; 16]>::try_from(*b).unwrap();
                            let ip = Ipv6Addr::from(b);
                            format!("{}", ip)
                        }
                        l => format!("invalid (len={})", l),
                    };
                    format!("IP address:{}", ip)
                }
                _ => format!("{:?}", name),
            };
            subnames.push(s);
        }
    }
    Some(subnames)
}
