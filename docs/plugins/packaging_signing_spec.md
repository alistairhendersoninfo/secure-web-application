# SPEC: Plugin Packaging, Signing, and Registry Trust

## Goals
- Define a secure packaging format, signing policy, and registry trust model for plugins.
- Ensure only verified, policy-compliant plugins run with declared capabilities.

## Non-Goals
- UI flows (covered in Plugin Registry UI SPEC).

## Architecture Overview
- Artifact: WASM module + manifest (name, version, category, capabilities, schemas) + detached signature.
- Registry: signed index; controller verifies signature chains and policies.

```mermaid
flowchart LR
  Dev[Plugin Dev] --> Build[Build & Sign]
  Build --> Registry[(Registry + Signed Index)]
  Registry --> Controller
  Controller --> Verify[Signature + Policy]
  Verify --> Runtime[Plugin Runtime]
```

## Detailed Design
- Manifest fields: id (UUID), name, version (semver), category, required hostcalls, memory/cpu limits, schema versions.
- Signing: Ed25519 or ECDSA P-256; detached signature over module digest + manifest.
- Trust policy: allowlist of public keys; optional expiration and revocation list.
- Update policy: immutable versions; downgrade/rollback allowed only to signed known-good.
- Revocation: key revocation and per-plugin version denylist.

## Security Posture
- No unsigned plugins; runtime validates signature and capabilities before instantiation.
- Capabilities enforced by runtime; no ambient FS/network.

## Operations
- Key management: rotation and revocation; build provenance (SLSA).
- Registry mirroring for offline environments.

## Acceptance Criteria
- Packaging and signing format defined; verification steps documented.
- Policy model (allowlist/denylist) and capability enforcement specified.

## Open Questions
- Multi-signature threshold for critical plugins?
