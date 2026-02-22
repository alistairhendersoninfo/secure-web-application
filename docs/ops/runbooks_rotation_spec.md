# SPEC: Operator Runbooks — Certs, Keys, and CA Rotation

## Goals
- Document operator procedures for rotating sensitive materials: TLS certs, plugin signing keys, and CAs. (Route slugs remain static by default.)

## Non-Goals
- Automating all steps; focus on safe, auditable procedures.

## Architecture Overview
- Certs: weekly controller cert rotation via restart; agent renewal before TTL/3; emergency revoke.
- Keys: plugin signing key rotation with dual-trust window; registry index update.
- CA: rare operation; staged rollout with trust anchors; revoke old after cutover.

```mermaid
sequenceDiagram
  participant Op as Operator
  participant C as Controller
  participant A as Agents
  Op->>C: Restart for new controller cert
  A-->>C: Renew agent certs progressively
  Op->>Registry: Add new plugin signing key (dual trust)
  Op->>C: Introduce new ICA, update trust stores
```

## Detailed Design
- Cert Rotation: enforce weekly restart; controller regenerates if self-signed; otherwise fetch from CA; agents renew before TTL/3.
- Plugin Key Rotation: add new key to allowlist; re-sign critical plugins; update registry index; remove old after window.
- CA Rotation: introduce new ICA signed by root; update trust stores; reissue certs; revoke old ICA.

## Security Posture
- Changes are audited; keys stored securely; procedures require multi-party approval.

## Acceptance Criteria
- Runbooks cover certs, plugin keys, CA; include steps, validation, and rollback notes.
