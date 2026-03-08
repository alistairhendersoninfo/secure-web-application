# How Specs Work

## SPEC Template Convention

All specifications in `docs/` follow a consistent structure:

### Required Sections

1. **Goals** — What the spec achieves (bulleted list)
2. **Non-Goals** — What is explicitly out of scope
3. **Architecture Overview** — High-level design with Mermaid diagrams
4. **Detailed Design** — Implementation specifics, configuration examples, data flows
5. **Open Questions** — Unresolved decisions (removed when resolved)

### Optional Sections

- **Per-Component Breakdown** — Tables or subsections for each component
- **Operational Runbook** — Day-2 operations, troubleshooting, scaling procedures
- **Security Considerations** — Threat model, trust boundaries, attack surface

## File Naming

Specs use snake_case with a `_spec.md` suffix:
- `k3s_infrastructure_spec.md`
- `single_image_spec.md`
- `secrets_management_spec.md`

## Spec Location

Specs are organized by domain under `docs/`:
- `docs/deploy/` — Deployment and infrastructure
- `docs/security/` — Security, PKI, secrets
- `docs/network/` — Networking, ingress, gateway
- `docs/ops/` — Operations, monitoring, upgrades
- `docs/agent/` — Agent management, isolation

## Mermaid Diagrams

All architecture diagrams use Mermaid syntax (validated by CI via `mermaid-validate.yml`). Prefer `flowchart TB` or `flowchart LR` for architecture diagrams and `sequenceDiagram` for interaction flows.
