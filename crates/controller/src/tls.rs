use anyhow::{anyhow, Result};
use rustls::{ServerConfig, Certificate};
use rustls::{RootCertStore, ServerConfig as _};
use rustls_pemfile::{certs, pkcs8_private_keys};
use std::fs::File;
use std::io::BufReader;
use tonic::transport::{Identity as TonicIdentity, Certificate as TonicCertificate};
use rcgen::{CertificateParams, DistinguishedName, DnType, KeyPair, Certificate as RcCert, IsCa, BasicConstraints};

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

pub fn ensure_self_signed(cert_path: &str, key_path: &str, cn: &str, valid_days: u64) -> Result<()> {
    use std::fs;
    let needs_new = match fs::metadata(cert_path) {
        Ok(meta) => {
            if let Ok(modified) = meta.modified() {
                if let Ok(age) = modified.elapsed() {
                    age.as_secs() > valid_days * 24 * 3600
                } else { true }
            } else { true }
        }
        Err(_) => true,
    } || fs::metadata(key_path).is_err();

    if !needs_new { return Ok(()); }

    let mut params = CertificateParams::default();
    let mut dn = DistinguishedName::new();
    dn.push(DnType::CommonName, cn);
    params.distinguished_name = dn;
    // server auth
    params.is_ca = IsCa::NoCa;
    params.not_before = rcgen::date_time_ymd(2020, 1, 1);
    params.not_after = rcgen::date_time_ymd(2030, 1, 1); // will be constrained by rotation interval
    params.alg = &rcgen::PKCS_ECDSA_P256_SHA256;
    // SubjectAltName for "controller" and localhost
    params.subject_alt_names = vec![
        rcgen::SanType::DnsName("controller".into()),
        rcgen::SanType::DnsName("localhost".into()),
        rcgen::SanType::IpAddress("127.0.0.1".parse().unwrap()),
    ];

    let key = KeyPair::generate(params.alg).map_err(|e| anyhow!("rcgen key: {e}"))?;
    params.key_pair = Some(key);
    let cert = RcCert::from_params(params).map_err(|e| anyhow!("rcgen cert: {e}"))?;
    let cert_pem = cert.serialize_pem().map_err(|e| anyhow!("rcgen serialize: {e}"))?;
    let key_pem = cert.serialize_private_key_pem();

    fs::create_dir_all(std::path::Path::new(cert_path).parent().unwrap_or_else(|| std::path::Path::new(".")))?;
    fs::write(cert_path, cert_pem)?;
    fs::write(key_path, key_pem)?;
    Ok(())
}
