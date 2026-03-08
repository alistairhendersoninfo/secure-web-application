# ADR-004: Per-Cluster Headlamp + Rancher Fleet Management

## Context

Multi-cluster Kubernetes deployments need management tooling for:

1. **Per-cluster visibility** — Viewing workloads, logs, and events in each cluster
2. **Fleet-level management** — Deploying configurations across all clusters from a single source of truth

## Decision

Use a two-layer management approach:

- **Headlamp** — Deployed per-cluster as a lightweight Kubernetes dashboard. Provides read-only visibility into workloads, pod logs, and events without requiring kubectl access.
- **Rancher Fleet** — Deployed on a dedicated management cluster (or co-located on Core for dev/staging). Provides GitOps-driven configuration management across all clusters.

Rancher Fleet watches the `deploy/k3s/rancher/fleet.yaml` configuration and applies Helm charts to target clusters based on labels.

## Consequences

- **Positive**: Developers get per-cluster dashboards without kubectl complexity
- **Positive**: GitOps ensures all cluster configurations are version-controlled and auditable
- **Positive**: Rancher Fleet supports staged rollouts and drift detection
- **Negative**: Rancher adds operational overhead (its own cluster, upgrades, RBAC)
- **Mitigation**: Rancher is optional — clusters can be managed directly with kubectl and Helm
