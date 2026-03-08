# K3s Infrastructure Deployment Spec

For the full specification, see `docs/deploy/k3s_infrastructure_spec.md` in the repository.

## Summary

This specification defines a production-grade K3s deployment topology for SWAP across four isolated cluster zones:

- **Core/Internal** — Controller, Auth Store, PostgreSQL, Agent Management, Plugin Registry
- **DMZ** — Traefik ingress, Coraza WAF, TLS termination (only public-facing cluster)
- **ETL/Data** — Log ingestion workers, transformation jobs, MinIO (optional)
- **Monitoring/Obs** — Prometheus (federated), Grafana, Alertmanager, Loki

An optional Rancher management cluster provides fleet-level visibility and GitOps configuration.

## Key Design Decisions

- **Alpine 3.22** base images with musl-static Rust binaries (no Distroless)
- **Cilium CNI** with eBPF for network policies (no kube-proxy)
- **Separate control planes** per cluster — no shared etcd or overlay
- **mTLS everywhere** using SPIFFE workload identity
- **Longhorn** for distributed block storage on StatefulSets
