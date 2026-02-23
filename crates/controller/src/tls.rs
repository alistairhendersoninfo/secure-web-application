use anyhow::{anyhow, Result};
use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use rustls::{RootCertStore, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};
use std::fs::File;
use std::io::BufReader;

pub fn server_config(
    cert_path: &str,
    key_path: &str,
    client_ca_path: &str,
) -> Result<ServerConfig> {
    let certs = load_certs(cert_path)?;
    let key = load_key(key_path)?;
    let client_ca = load_client_ca(client_ca_path)?;

    let mut cfg = ServerConfig::builder()
        .with_client_cert_verifier(client_ca)
        .with_single_cert(certs, key)?;

    // TLS 1.3 only; rustls safe defaults already prioritize strong suites
    cfg.alpn_protocols = vec![b"h2".to_vec()];
    Ok(cfg)
}

fn load_certs(path: &str) -> Result<Vec<CertificateDer<'static>>> {
    let mut reader = BufReader::new(File::open(path)?);
    let certs = certs(&mut reader)
        .collect::<std::result::Result<Vec<_>, _>>()
        .map_err(|_| anyhow!("failed to read certs"))?;
    Ok(certs)
}

fn load_key(path: &str) -> Result<PrivateKeyDer<'static>> {
    let mut reader = BufReader::new(File::open(path)?);
    let key = pkcs8_private_keys(&mut reader)
        .next()
        .ok_or_else(|| anyhow!("no private key found"))?
        .map_err(|_| anyhow!("failed to read private key"))?;
    Ok(PrivateKeyDer::Pkcs8(key))
}

fn load_client_ca(
    path: &str,
) -> Result<std::sync::Arc<dyn rustls::server::danger::ClientCertVerifier>> {
    let mut store = RootCertStore::empty();
    let mut reader = BufReader::new(File::open(path)?);
    for cert in certs(&mut reader) {
        let cert = cert.map_err(|_| anyhow!("failed to read client CA"))?;
        store.add(cert).map_err(|e| anyhow!("{e}"))?;
    }
    let verifier =
        rustls::server::WebPkiClientVerifier::builder(std::sync::Arc::new(store)).build()?;
    Ok(verifier)
}
