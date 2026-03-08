# Features

## Security-First Architecture

SWAP is designed from the ground up with security as the primary concern, not an afterthought.

### Mutual TLS Everywhere

All inter-service and inter-cluster communication uses mTLS with SPIFFE-based workload identity. No plaintext traffic, no shared secrets — every connection is authenticated and encrypted.

### Hardened Container Images

All production images use Alpine 3.22 with musl-static Rust binaries. No shell, no package manager, no unnecessary utilities in the runtime image. Images are signed and verified through a supply-chain attestation pipeline.

### Network Isolation

Four dedicated K3s clusters enforce network-level isolation between trust zones:

| Cluster | Purpose | Trust Level |
|---------|---------|-------------|
| **DMZ** | Public-facing ingress, WAF, TLS termination | Lowest — internet-exposed |
| **Core** | Controller, auth, PostgreSQL, agent management | Highest — internal only |
| **ETL/Data** | Log ingestion, transformation, storage | Medium — receives external data |
| **Monitoring** | Prometheus, Grafana, Alertmanager, Loki | Medium — observes all clusters |

## Plugin System

SWAP supports WebAssembly plugins for extensible security tooling. Plugins run in a sandboxed WASM runtime with capability-based permissions, limiting what each plugin can access.

## Agent Management

Distributed security agents connect to the SWAP control plane for:

- Centralized policy distribution
- Log collection and forwarding
- Health monitoring and alerting
- Remote configuration updates

## Operational Tooling

- **Rancher Fleet** — GitOps-driven multi-cluster management
- **Headlamp** — Per-cluster Kubernetes dashboard
- **Prometheus Federation** — Unified metrics across all clusters
- **Loki** — Centralized log aggregation
