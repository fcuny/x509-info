use anyhow::{anyhow, Error};
use rustls::{
    Certificate, ClientConfig, ClientConnection, OwnedTrustAnchor, RootCertStore, ServerName,
};

use std::io::Write;
use std::sync::Arc;

struct SkipServerVerification;

impl SkipServerVerification {
    fn new() -> Arc<Self> {
        Arc::new(Self)
    }
}
impl rustls::client::ServerCertVerifier for SkipServerVerification {
    fn verify_server_cert(
        &self,
        _end_entity: &rustls::Certificate,
        _intermediates: &[rustls::Certificate],
        _server_name: &rustls::ServerName,
        _scts: &mut dyn Iterator<Item = &[u8]>,
        _ocsp_response: &[u8],
        _now: std::time::SystemTime,
    ) -> Result<rustls::client::ServerCertVerified, rustls::Error> {
        Ok(rustls::client::ServerCertVerified::assertion())
    }
}

pub fn get_certificates(
    domain: String,
    port: u16,
    insecure: bool,
) -> Result<Vec<Certificate>, Error> {
    let mut tcp_stream = std::net::TcpStream::connect(format!("{}:{}", domain, port))?;

    let config = if insecure {
        ClientConfig::builder()
            .with_safe_defaults()
            .with_custom_certificate_verifier(SkipServerVerification::new())
            .with_no_client_auth()
    } else {
        let mut root_store = RootCertStore::empty();
        root_store.add_server_trust_anchors(webpki_roots::TLS_SERVER_ROOTS.0.iter().map(|ta| {
            OwnedTrustAnchor::from_subject_spki_name_constraints(
                ta.subject,
                ta.spki,
                ta.name_constraints,
            )
        }));
        ClientConfig::builder()
            .with_safe_defaults()
            .with_root_certificates(root_store)
            .with_no_client_auth()
    };

    let server_name = ServerName::try_from(domain.as_ref())?;

    let mut conn = ClientConnection::new(Arc::new(config), server_name)?;
    while conn.wants_write() {
        conn.write_tls(&mut tcp_stream)?;
    }

    tcp_stream.flush()?;
    while conn.is_handshaking() && conn.peer_certificates().is_none() {
        conn.read_tls(&mut tcp_stream)?;
        conn.process_new_packets()?;
    }

    match conn.peer_certificates() {
        Some(c) => Ok(c.to_vec()),
        None => Err(anyhow!("no certificate found for {}", domain))?,
    }
}
