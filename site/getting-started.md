# Getting Started

## Prerequisites

- 4+ Linux nodes (bare-metal or VMs) with routable IPs between them
- SSH access to all nodes
- `helm` v3.14+
- `kubectl` v1.31+
- `k3sup` for cluster bootstrapping

## Quick Start

### 1. Bootstrap the Core Cluster

```bash
# Install K3s on the core node
k3sup install \
  --ip <CORE_NODE_IP> \
  --user root \
  --k3s-extra-args '--disable=traefik --flannel-backend=none'

# Install Cilium CNI
helm install cilium cilium/cilium \
  --namespace kube-system \
  --set operator.replicas=1
```

### 2. Deploy SWAP Core Services

```bash
# Add the SWAP Helm chart
helm install swap-core deploy/k3s/charts/swap-core \
  --namespace swap-system \
  --create-namespace \
  --values deploy/k3s/charts/swap-core/values.yaml
```

### 3. Bootstrap Additional Clusters

Repeat the K3s installation for DMZ, ETL, and Monitoring nodes, then deploy the corresponding Helm charts:

```bash
helm install swap-dmz deploy/k3s/charts/swap-dmz --namespace swap-system --create-namespace
helm install swap-etl deploy/k3s/charts/swap-etl --namespace swap-system --create-namespace
helm install swap-monitoring deploy/k3s/charts/swap-monitoring --namespace swap-system --create-namespace
```

### 4. Configure Rancher Fleet (Optional)

```bash
# Install Rancher on the management cluster
helm install rancher rancher-latest/rancher \
  --namespace cattle-system \
  --create-namespace \
  --set hostname=rancher.your-domain.com

# Apply fleet configuration
kubectl apply -f deploy/k3s/rancher/fleet.yaml
```

## Next Steps

- Review the [K3s Infrastructure Spec](specs/k3s-infrastructure.md) for full deployment details
- See [Architecture](architecture.md) for the multi-cluster topology
- Check [Features](features.md) for platform capabilities
