# ADR-003: Multi-Cluster Topology (4-Zone Split)

## Context

A security platform handling sensitive data needs strict isolation between components with different trust levels. A single Kubernetes cluster with namespaces provides logical isolation but not network-level isolation — a compromised pod can potentially reach any other pod in the cluster.

## Decision

Deploy SWAP across four dedicated K3s clusters, each in a separate network zone:

| Cluster | Trust Level | Purpose |
|---------|-------------|---------|
| **DMZ** | Lowest | Public-facing ingress, WAF, TLS termination |
| **Core** | Highest | Controller, auth, database, agent management |
| **ETL/Data** | Medium | Log ingestion, transformation, storage |
| **Monitoring** | Medium | Prometheus, Grafana, Alertmanager, Loki |

Each cluster has its own control plane, etcd, and Cilium CNI. Inter-cluster communication uses routable IPs with mTLS — no shared overlay network.

An optional Rancher management cluster provides fleet-level visibility.

## Consequences

- **Positive**: A compromised DMZ pod cannot reach Core cluster pods — true network isolation
- **Positive**: Each cluster can be sized, updated, and scaled independently
- **Positive**: Blast radius of any failure is contained to one zone
- **Negative**: Higher infrastructure cost (4+ control planes)
- **Negative**: More complex networking setup (routable IPs, firewall rules, mTLS between clusters)
- **Mitigation**: Rancher Fleet provides single-pane management across all clusters
