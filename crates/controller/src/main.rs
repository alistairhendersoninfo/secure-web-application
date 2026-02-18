mod config;
mod db;
mod tls;
mod server;

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
    let _pool = db::connect(&cfg.database_url).await?;

    let tls_cfg = tls::server_config(&cfg.tls_cert_path, &cfg.tls_key_path, &cfg.client_ca_path)?;
    let acceptor = TlsAcceptor::from(std::sync::Arc::new(tls_cfg));

    let listener = TcpListener::bind(&cfg.bind_addr).await?;
    tracing::info!(addr = %cfg.bind_addr, "controller listening (mTLS)");

    let app = server::app();

    loop {
        let (stream, _) = listener.accept().await?;
        let acceptor = acceptor.clone();
        let app = app.clone();
        tokio::spawn(async move {
            let stream = match acceptor.accept(stream).await {
                Ok(s) => s,
                Err(err) => {
                    tracing::warn!(?err, "TLS accept failed");
                    return;
                }
            };
            if let Err(err) = axum::serve(stream, app).await {
                tracing::warn!(?err, "serve error");
            }
        });
    }
}
