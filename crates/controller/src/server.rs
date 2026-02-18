use axum::{routing::get, Router};
use tower_http::set_header::SetResponseHeaderLayer;
use http::HeaderValue;

pub fn app() -> Router {
    // Minimal routes; add security headers
    let csp = HeaderValue::from_static("default-src 'self'; frame-ancestors 'none'; object-src 'none'");
    let hsts = HeaderValue::from_static("max-age=63072000; includeSubDomains; preload");
    Router::new()
        .route("/healthz", get(|| async { "ok" }))
        .layer(SetResponseHeaderLayer::if_not_present(
            http::header::CONTENT_SECURITY_POLICY,
            csp,
        ))
        .layer(SetResponseHeaderLayer::if_not_present(
            http::header::STRICT_TRANSPORT_SECURITY,
            hsts,
        ))
}
