mod config;
mod db;
mod tls;
mod server;
mod grpc;

use anyhow::Result;
use tokio::net::TcpListener;
use tokio_rustls::TlsAcceptor;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .init();

    let cfg = config::Config::from_env()?;
    let pool = db::connect(&cfg.database_url).await?;
    db::migrate(&pool).await?;

    // gRPC server (mTLS)
    let grpc_addr: std::net::SocketAddr = cfg.grpc_bind_addr.parse()?;
    let identity = tls::tonic_server_identity(&cfg.tls_cert_path, &cfg.tls_key_path)?;
    let client_ca = tls::tonic_client_ca(&cfg.client_ca_path)?;

    // Optionally keep a small HTTPS /healthz server on a separate port
    let http_bind = cfg.bind_addr.clone();
    tokio::spawn(async move {
        match serve_health(http_bind).await {
            Ok(()) => {}
            Err(e) => tracing::warn!(error = ?e, "health server failed"),
        }
    });

    tracing::info!(addr = %cfg.grpc_bind_addr, "gRPC listening (mTLS)");
    grpc::serve_grpc(grpc_addr, identity, client_ca, pool).await?;
    Ok(())
}

async fn serve_health(bind_addr: String) -> anyhow::Result<()> {
    let listener = TcpListener::bind(&bind_addr).await?;
    let app = server::app();
    loop {
        let (stream, _) = listener.accept().await?;
        let app = app.clone();
        tokio::spawn(async move {
            if let Err(err) = axum::serve(stream, app).await {
                tracing::warn!(?err, "serve error");
            }
        });
    }
}
