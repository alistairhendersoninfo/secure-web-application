use anyhow::Result;
use tonic::transport::{Channel, ClientTlsConfig, Certificate, Identity};

pub mod proto {
    tonic::include_proto!("swap.v1");
}

use proto::heartbeat_service_client::HeartbeatServiceClient;
use proto::HeartbeatRequest;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    let controller_url = std::env::var("CONTROLLER_URL").unwrap_or_else(|_| "https://127.0.0.1:8443".to_string());
    let agent_cert_path = std::env::var("AGENT_CERT_PATH")?;
    let agent_key_path = std::env::var("AGENT_KEY_PATH")?;
    let controller_ca_path = std::env::var("CONTROLLER_CA_PATH")?;

    println!("agent starting; controller={controller_url}");

    let tls = client_tls(agent_cert_path, agent_key_path, controller_ca_path)?;
    let channel = Channel::from_shared(controller_url)?
        .tls_config(tls)?
        .connect()
        .await?;

    let mut client = HeartbeatServiceClient::new(channel);
    let req = HeartbeatRequest { agent_id: String::new(), seq: 1, monotonic_ms: 0 };
    let _ = client.heartbeat(req).await?;
    Ok(())
}

fn client_tls(cert_path: String, key_path: String, ca_path: String) -> Result<ClientTlsConfig> {
    use std::fs;
    let cert = fs::read(cert_path)?;
    let key = fs::read(key_path)?;
    let ca = fs::read(ca_path)?;
    let identity = Identity::from_pem(cert, key);
    let ca = Certificate::from_pem(ca);
    Ok(ClientTlsConfig::new().identity(identity).ca_certificate(ca).domain_name("controller"))
}
