# SPEC: Secrets Management (DB creds, keys, tokens)

## Goals
- Define secure handling for secrets: storage, access, rotation.

## Non-Goals
- Full Vault/KMS setup guides.

## Architecture Overview
- Secrets sourced from OS keyring/KMS/Vault; injected as env vars or files with strict perms; in-memory use only.

```mermaid
flowchart LR
  KMS[OS KMS/TPM/Vault] --> App[Controller]
  App --> Use[In-memory Use]
```

## Detailed Design
- Storage: prefer OS KMS (TPM-backed) on agents; Vault or KMS for controller.
- Access: least privilege; no secrets in logs; zero-trust between components beyond issued credentials.
- Rotation: DB creds, API tokens, signing keys; runbooks and automation hooks.

## Security Posture
- Secrets never committed; minimal lifetime in memory; secure file perms when needed.

## Operations
- Secret owners and rotation cadence documented; audit of secret access.

## Acceptance Criteria
- Secrets flows documented; rotation procedures exist; integration points with KMS/Vault identified.
