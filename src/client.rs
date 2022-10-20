use rustls::{
    Certificate, ClientConfig, ClientConnection, OwnedTrustAnchor, RootCertStore, ServerName,
};
use std::error::Error;

use std::io::Write;
use std::sync::Arc;

pub fn get_certificates(domain: String, port: u16) -> Result<Vec<Certificate>, Box<dyn Error>> {
    let mut tcp_stream = std::net::TcpStream::connect(format!("{}:{}", domain, port))?;

    let mut root_store = RootCertStore::empty();
    root_store.add_server_trust_anchors(webpki_roots::TLS_SERVER_ROOTS.0.iter().map(|ta| {
        OwnedTrustAnchor::from_subject_spki_name_constraints(
            ta.subject,
            ta.spki,
            ta.name_constraints,
        )
    }));

    let config = ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(root_store)
        .with_no_client_auth();

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
        None => Err("no certificate found")?,
    }
}
