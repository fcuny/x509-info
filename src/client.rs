use anyhow::{anyhow, Error};
use rustls::client::danger::{HandshakeSignatureValid, ServerCertVerified, ServerCertVerifier};
use rustls::crypto::{ring, verify_tls12_signature, verify_tls13_signature, CryptoProvider};
use rustls::pki_types::{self, CertificateDer, ServerName, UnixTime};
use rustls::{ClientConfig, ClientConnection};
use rustls::{DigitallySignedStruct, SignatureScheme};

use std::io::Write;
use std::sync::Arc;

#[derive(Debug)]
struct NoCertificateVerification {}

impl ServerCertVerifier for NoCertificateVerification {
    fn verify_server_cert(
        &self,
        _end_entity: &CertificateDer<'_>,
        _intermediates: &[CertificateDer<'_>],
        _server_name: &pki_types::ServerName<'_>,
        _ocsp_response: &[u8],
        _now: UnixTime,
    ) -> Result<ServerCertVerified, rustls::Error> {
        Ok(ServerCertVerified::assertion())
    }

    fn verify_tls12_signature(
        &self,
        message: &[u8],
        cert: &CertificateDer<'_>,
        dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, rustls::Error> {
        verify_tls12_signature(
            message,
            cert,
            dss,
            &ring::default_provider().signature_verification_algorithms,
        )
    }

    fn verify_tls13_signature(
        &self,
        message: &[u8],
        cert: &CertificateDer<'_>,
        dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, rustls::Error> {
        verify_tls13_signature(
            message,
            cert,
            dss,
            &ring::default_provider().signature_verification_algorithms,
        )
    }

    fn supported_verify_schemes(&self) -> Vec<SignatureScheme> {
        ring::default_provider()
            .signature_verification_algorithms
            .supported_schemes()
    }
}

pub fn get_certificates(
    domain: String,
    port: u16,
    insecure: bool,
) -> Result<Vec<CertificateDer<'static>>, Error> {
    let mut tcp_stream = std::net::TcpStream::connect(format!("{}:{}", domain, port))?;

    let config = if insecure {
        ClientConfig::builder()
            .dangerous()
            .with_custom_certificate_verifier(Arc::new(NoCertificateVerification {}))
            .with_no_client_auth()
    } else {
        let root_store = rustls::RootCertStore {
            roots: webpki_roots::TLS_SERVER_ROOTS.to_vec(),
        };
        ClientConfig::builder_with_provider(
            CryptoProvider {
                cipher_suites: vec![ring::cipher_suite::TLS13_CHACHA20_POLY1305_SHA256],
                kx_groups: vec![ring::kx_group::X25519],
                ..ring::default_provider()
            }
            .into(),
        )
        .with_protocol_versions(&[&rustls::version::TLS13])
        .unwrap()
        .with_root_certificates(root_store)
        .with_no_client_auth()
    };

    let server_name = ServerName::try_from(domain.clone())
        .expect("invalid DNS name")
        .to_owned();

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
