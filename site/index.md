# SWAP — Secure Web Application Platform

**Hardened security orchestration with K3s multi-cluster deployment.**

SWAP is a security-first platform for deploying, managing, and monitoring distributed security agents across infrastructure. Built in Rust with mTLS everywhere, SWAP provides a hardened control plane for security operations.

## Why SWAP?

- **Zero-trust networking** — mTLS between all components, SPIFFE identity for workloads
- **Multi-cluster isolation** — Four dedicated K3s clusters separate trust zones (DMZ, Core, ETL, Monitoring)
- **Minimal attack surface** — Alpine 3.22 musl-static binaries, no shells in production images
- **Plugin architecture** — WebAssembly plugin system for extensible security tooling
- **GitOps-native** — Rancher Fleet for declarative multi-cluster management

## Quick Links

- [Features](features.md) — What SWAP provides
- [Architecture](architecture.md) — How it's built
- [Getting Started](getting-started.md) — Deploy your first cluster
- [K3s Infrastructure Spec](specs/k3s-infrastructure.md) — Full deployment specification
