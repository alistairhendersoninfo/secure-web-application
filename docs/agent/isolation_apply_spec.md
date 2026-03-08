# SPEC: Agent Isolation, Privilege, and Apply Flow

## Goals
- Constrain agent processes; minimize privileges; define safe apply/rollback.
- Support on-host operations for firewall, WAF, LSM, AV/IDS with least privilege.

## Non-Goals
- Tool internals beyond apply interfaces.

## Architecture Overview
- Agent runs unprivileged; spawns short-lived helpers with bounded capabilities per operation.
- Apply flow stages configuration, validates, atomically switches, probes health, and rolls back on failure.

```mermaid
sequenceDiagram
  participant C as Controller
  participant A as Agent (confined)
  participant H as Helper (cap-bounded)
  C->>A: Plan(steps)
  A->>H: Apply step with CAP_* (time-bounded)
  H-->>A: Result
  A-->>C: Status/Logs
```

## Detailed Design
- Confinement: seccomp-bpf, user namespaces, cgroups, AppArmor/SELinux profiles.
- Capabilities: grant CAP_NET_ADMIN only for nftables step; drop after; no ambient caps.
- Staging: write to temp dir; validate syntax; switch via atomic symlink or tool-native transaction.
- Health: pre-defined probes; immediate rollback if probes fail; cooldown before retry.

## Security Posture
- Minimize time-in-capability; log all escalations; deny permanent elevation.
- No direct network/file writes outside designated paths.

## Operations
- OS support matrix; fallbacks for distros; profile packaging and updates.

## Acceptance Criteria
- Documented helper capabilities per plugin category; apply/rollback sequence; probes and backoff.

## DaemonSet Agent Mode (K3s Deployments)

In K3s deployments, agents can run as DaemonSet pods on K3s nodes:
- **SecurityContext**: `runAsNonRoot: true`, `readOnlyRootFilesystem: true`, `allowPrivilegeEscalation: false`, `capabilities.drop: ["ALL"]`, `seccompProfile.type: RuntimeDefault`.
- **Privileged operations**: Use init containers or sidecar containers with narrowly scoped capabilities (e.g., `CAP_NET_ADMIN` for nftables) instead of elevating the main agent process. Init containers run once at startup; sidecars handle ongoing privileged operations via a Unix socket API.
- **Host access**: Where agents need host-level access (e.g., nftables, AppArmor), use `hostPID: false`, `hostNetwork: false`, and mount only specific host paths (`/etc/nftables.d`, `/etc/apparmor.d`) as read-only where possible.
- **Standalone mode**: For external hosts not running K3s, agents continue to run as standalone binaries with systemd confinement as described above.

See [K3s Infrastructure Spec](../deploy/k3s_infrastructure_spec.md) for DaemonSet deployment details.

## Open Questions
- Common helper framework vs per-plugin helpers.
