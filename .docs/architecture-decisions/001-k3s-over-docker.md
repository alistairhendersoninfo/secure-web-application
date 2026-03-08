# ADR-001: K3s Over Docker Compose

## Context

SWAP was originally designed to run via Docker Compose for local development and single-node deployments. As the platform matured, requirements emerged for:

- Network-level isolation between trust zones (DMZ, internal, monitoring)
- Declarative workload management with health checks, rolling updates, and resource limits
- Multi-node deployment without custom orchestration scripts
- GitOps-driven configuration management

Docker Compose cannot provide network isolation between trust zones without complex manual iptables rules, and lacks native support for multi-node orchestration.

## Decision

Replace Docker Compose with K3s as the primary deployment target. K3s provides:

- Lightweight Kubernetes with a single binary and low resource overhead
- Native support for NetworkPolicies (via Cilium CNI) for trust zone isolation
- Helm charts for declarative, repeatable deployments
- Rancher Fleet integration for GitOps multi-cluster management
- StatefulSet support for PostgreSQL and other stateful workloads

The single-image Docker deployment remains available for development and testing.

## Consequences

- **Positive**: True network isolation, declarative management, multi-node scaling, GitOps
- **Positive**: K3s has lower overhead than full Kubernetes distributions
- **Negative**: Higher operational complexity than Docker Compose
- **Negative**: Requires Kubernetes knowledge for day-2 operations
- **Mitigation**: Helm charts and Rancher Fleet reduce operational burden
