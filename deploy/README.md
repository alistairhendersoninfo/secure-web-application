# Deployment (Docker + Host Firewall)

Overview
- Containerized controller runs as non-root, read-only, with no extra capabilities.
- Exposes only port 10443 (HTTP/2 mTLS). No SSH, no proxies.
- Host firewall (nftables) drops all inbound except 10443.
- Self-signed server cert generation on startup; rotate weekly by default.

Steps
1) Configure Postgres and set `DATABASE_URL`.
2) Apply host firewall rules (requires sudo):
   ```bash
   cd deploy/nftables
   ./apply.sh
   ```
3) Build and run the container:
   ```bash
   cd deploy/docker
   mkdir -p certs plugins
   docker compose up -d --build
   ```

Notes
- Certs live in `deploy/docker/certs/`. On first start, the controller will generate a self-signed cert if `TLS_SELF_SIGNED=1`.
- Certificates are considered stale after 7 days; on next restart the controller regenerates them. To enforce rotation without live reload, restart weekly (e.g., via orchestrator policy).
- Running nftables inside the container is discouraged; apply firewall rules on the host for least privilege.
- The container drops all capabilities and runs read-only with tmpfs for `/tmp` and `/run`.
