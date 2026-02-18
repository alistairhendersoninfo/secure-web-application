use anyhow::{anyhow, Result};
use rustls::{ServerConfig, Certificate};
use rustls::{RootCertStore, ServerConfig as _};
use rustls_pemfile::{certs, pkcs8_private_keys};
use std::fs::File;
use std::io::BufReader;
use tonic::transport::{Identity as TonicIdentity, Certificate as TonicCertificate};

pub fn server_config(
    cert_path: &str,
    key_path: &str,
    client_ca_path: &str,
) -> Result<ServerConfig> {
    let certs = load_certs(cert_path)?;
    let key = load_key(key_path)?;
    let client_ca = load_client_ca(client_ca_path)?;

    let mut cfg = ServerConfig::builder()
        .with_safe_defaults()
        .with_client_cert_verifier(client_ca)
        .with_single_cert(certs, key)?;

    // TLS 1.3 only; rustls safe defaults already prioritize strong suites
    cfg.alpn_protocols = vec![b"h2".to_vec()];
    Ok(cfg)
}

fn load_certs(path: &str) -> Result<Vec<rustls::Certificate>> {
    let mut reader = BufReader::new(File::open(path)?);
    let certs = certs(&mut reader)
        .map_err(|_| anyhow!("failed to read certs"))?
        .into_iter()
        .map(rustls::Certificate)
        .collect();
    Ok(certs)
}

fn load_key(path: &str) -> Result<rustls::PrivateKey> {
    let mut reader = BufReader::new(File::open(path)?);
    let mut keys = pkcs8_private_keys(&mut reader).map_err(|_| anyhow!("failed to read private key"))?;
    let key = keys.remove(0);
    Ok(rustls::PrivateKey(key))
}

fn load_client_ca(path: &str) -> Result<std::sync::Arc<rustls::server::WebPkiClientVerifier>> {
    let mut store = RootCertStore::empty();
    let mut reader = BufReader::new(File::open(path)?);
    for cert in certs(&mut reader).map_err(|_| anyhow!("failed to read client CA"))? {
        store.add(&Certificate(cert)).map_err(|e| anyhow!("{e}"))?;
    }
    let verifier = rustls::server::WebPkiClientVerifier::builder(store).build()?;
    Ok(std::sync::Arc::new(verifier))
}

pub fn tonic_server_identity(cert_path: &str, key_path: &str) -> Result<TonicIdentity> {
    use std::fs;
    let cert = fs::read(cert_path)?;
    let key = fs::read(key_path)?;
    Ok(TonicIdentity::from_pem(cert, key))
}

pub fn tonic_client_ca(client_ca_path: &str) -> Result<TonicCertificate> {
    use std::fs;
    let ca = fs::read(client_ca_path)?;
    Ok(TonicCertificate::from_pem(ca))
}
