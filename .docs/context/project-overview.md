# SWAP Project Overview

## What is SWAP?

SWAP (Secure Web Application Platform) is a security-first platform for deploying, managing, and monitoring distributed security agents. Built entirely in Rust, it provides a hardened control plane for security operations.

## Current State (K3s Migration Phase)

The project is migrating from Docker Compose to K3s multi-cluster deployment. Key artifacts:

### Specifications
- `docs/deploy/k3s_infrastructure_spec.md` — Full K3s deployment specification
- `docs/deploy/single_image_spec.md` — Single-image Docker deployment (dev/test)
- `docs/network/site_gateway_spec.md` — DMZ ingress topology
- `docs/security/secrets_management_spec.md` — Secrets and PKI management
- `docs/ops/observability_metrics_spec.md` — Monitoring stack specification

### Helm Charts
- `deploy/k3s/charts/swap-core/` — Core cluster (controller, auth, PostgreSQL)
- `deploy/k3s/charts/swap-dmz/` — DMZ cluster (Traefik, WAF)
- `deploy/k3s/charts/swap-etl/` — ETL cluster (log ingestion)
- `deploy/k3s/charts/swap-monitoring/` — Monitoring cluster (Prometheus, Grafana)
- `deploy/k3s/rancher/fleet.yaml` — Rancher Fleet configuration

### Codebase
- `crates/` — Rust workspace with all SWAP services
- `proto/` — Protocol Buffer definitions
- `migrations/` — Database migrations

## Key Technical Decisions

See `.docs/architecture-decisions/` for ADRs covering:
- K3s over Docker Compose (ADR-001)
- Alpine 3.22 over Distroless (ADR-002)
- Multi-cluster topology (ADR-003)
- Headlamp + Rancher management (ADR-004)
