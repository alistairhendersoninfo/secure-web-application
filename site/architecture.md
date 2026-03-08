# Architecture

## Overview

SWAP uses a multi-cluster K3s topology to enforce strict network isolation between trust zones. Each cluster runs its own control plane, CNI (Cilium), and etcd — there is no shared overlay network.

```mermaid
flowchart TB
  subgraph mgmt["Rancher Management"]
    R[Rancher Server]
  end

  subgraph core["Core / Internal K3s"]
    CTRL[Controller]
    AUTH[Auth Store]
    PG[PostgreSQL]
    AMGMT[Agent Management]
    PREG[Plugin Registry]
  end

  subgraph dmz["DMZ K3s"]
    TF[Traefik Reverse Proxy]
    WAF[Coraza WAF]
    TLS_T[TLS Termination]
  end

  subgraph etl["ETL / Data K3s"]
    LIW[Log Ingestion Workers]
    TJ[Transformation Jobs]
  end

  subgraph obs["Monitoring / Obs K3s"]
    PROM[Prometheus]
    GRAF[Grafana]
    AM[Alertmanager]
    LOKI[Loki]
  end

  R -->|manages| core
  R -->|manages| dmz
  R -->|manages| etl
  R -->|manages| obs

  dmz -->|mTLS| core
  core -->|mTLS| etl
  core -->|mTLS| obs
  etl -->|mTLS| obs
```

## Technology Stack

| Layer | Technology | Purpose |
|-------|-----------|---------|
| **Runtime** | K3s | Lightweight Kubernetes distribution |
| **CNI** | Cilium | eBPF-based networking + network policies |
| **Ingress** | Traefik | Reverse proxy with Coraza WAF sidecar |
| **Identity** | SPIFFE/SPIRE | Workload identity + mTLS certificates |
| **Certificates** | cert-manager | Automated certificate lifecycle |
| **Storage** | Longhorn | Distributed block storage for StatefulSets |
| **Fleet** | Rancher Fleet | GitOps multi-cluster management |
| **Language** | Rust | All SWAP services compiled as musl-static binaries |
| **Images** | Alpine 3.22 | Minimal runtime base image |

## Container Image Pipeline

All SWAP images follow a two-stage build:

1. **Builder stage** — `rust:1.87-alpine3.22` compiles the binary with `RUSTFLAGS='-C target-feature=+crt-static'`
2. **Runtime stage** — `alpine:3.22` with only the static binary, CA certificates, and a non-root user

No shell, no package manager, no debugging tools in production images.

## Inter-Cluster Communication

Clusters communicate over routable IPs with mTLS. There is no shared overlay network or VPN tunnel. Each cluster's Cilium CNI manages its own pod network independently.

Cross-cluster traffic is restricted by:

- Cilium `CiliumNetworkPolicy` on egress
- Firewall rules on the host network
- mTLS certificate validation (SPIFFE trust domain per cluster)
