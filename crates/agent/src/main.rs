use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    let controller_url = std::env::var("CONTROLLER_URL").unwrap_or_else(|_| "https://127.0.0.1:8443".to_string());
    println!("agent starting; controller={controller_url}");
    // TODO: establish outbound-only mTLS connection and heartbeat
    Ok(())
}
