# secure-web-application (Rust) — Security Wrapper Platform

Hardened Rust workspace for a controller/agent security orchestration platform with Postgres, mTLS, CI checks, and security-first defaults. Designed to host a plugin model (WASM/WASI) to configure and observe tools like nftables, Coraza WAF, NGINX/Apache, AppArmor, ClamAV, Falco, and more.

Components
- crates/controller: axum/hyper-based API with rustls mTLS and Postgres (sqlx) foundation.
- crates/agent: outbound-only mTLS client skeleton for controller communication.
- crates/shared: shared types and utilities.
- migrations: initial Postgres schema and audit trail primitives.

Quick Start (dev)
- Set `DATABASE_URL` to a Postgres URL with TLS.
- Provide `TLS_CERT_PATH`, `TLS_KEY_PATH`, and `CLIENT_CA_PATH` for server mTLS.
- `cargo build` to compile (CI enforces fmt, clippy, and audit).

Security Highlights
- Rust (memory safe), TLS 1.3 only via rustls, strict ciphers.
- Postgres with RLS-ready schema and audit chain foundation.
- Workspace-wide forbid-unsafe and lints.

See `prompt.md` for the working requirements and architecture decisions we are iterating on.
