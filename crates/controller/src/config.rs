use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub bind_addr: String,
    pub database_url: String,
    pub tls_cert_path: String,
    pub tls_key_path: String,
    pub client_ca_path: String,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        dotenvy::dotenv().ok();
        let bind_addr = std::env::var("BIND_ADDR").unwrap_or_else(|_| "0.0.0.0:8443".to_string());
        let database_url = std::env::var("DATABASE_URL")?;
        let tls_cert_path = std::env::var("TLS_CERT_PATH")?;
        let tls_key_path = std::env::var("TLS_KEY_PATH")?;
        let client_ca_path = std::env::var("CLIENT_CA_PATH")?;
        Ok(Self {
            bind_addr,
            database_url,
            tls_cert_path,
            tls_key_path,
            client_ca_path,
        })
    }
}
