# SPEC: PKI, Identities, and Enrollment/Attestation

## Goals
- Establish a hardened PKI for controller and agent identities with short-lived certificates.
- Define enrollment, renewal, and revocation flows; support offline-friendly operations.
- Optional SPIFFE/SPIRE and TPM-backed keys with remote attestation.

## Non-Goals
- Full SPIFFE/SPIRE setup guides; this focuses on interfaces and policy.

## Architecture Overview
- Offline root CA; online intermediate CA for issuance.
- Controller and agents use mTLS; agents authenticate solely via certs.
- Enrollment supports bootstrap token mode or pre-provisioned certs.

```mermaid
flowchart LR
  RootCA[(Offline Root CA)] --> ICA[(Intermediate CA)]
  ICA --> C[Controller Cert]
  ICA --> A[Agent Certs]
  A -. mTLS .- C
```

## Detailed Design
- Certificate Profiles
  - Controller: serverAuth (CN alt names include hostnames/IPs), TTL 30–90d.
  - Agent: clientAuth (SPIFFE ID optional), TTL 7–30d, renewable.
- Enrollment
  - Modes: (1) Bootstrap token + CSR, (2) Pre-provisioned agent cert.
  - Evidence: AgentHello (hostname, OS, labels), optional TPM quote.
  - Policy: allowlist and/or label selectors; attest evidence verification.
- Renewal
  - Agents renew before 2/3 of TTL; failure triggers backoff; no password fallback.
  - Grace period window; revocation list respected.
- Revocation
  - CRL/OCSP optional; controller maintains denylist for identities.
- SPIFFE/SPIRE (optional)
  - Controller integrates with SPIRE server for SVID issuance; agent uses SPIRE workload API.
- TPM-backed Keys (optional)
  - Agent key generated in TPM; attestation binds key to platform state.

## Security Posture
- Private keys never leave their hosts; agent keys seal to TPM when available.
- Short-lived certs reduce replay/exfiltration risk; pinned controller CA.

## Operations
- Root CA stored offline with documented rotation; ICA rotated yearly.
- Enrollment bootstrap tokens are time-bound, single-use, and audited.

## Acceptance Criteria
- Documented CSR, enrollment, renewal, and revocation flows with APIs.
- Optional SPIFFE/SPIRE and TPM flows specified.
- Audit fields captured for every issuance/renewal/revocation.

## Open Questions
- Default TTLs for controller vs agent.
- Attestation requirements per environment.
