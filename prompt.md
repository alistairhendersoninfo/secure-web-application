# Security Web Application Platform (SWAP) — Prompt / Requirements

This document captures initial goals, constraints, architecture options, and security requirements for building a hardened web security orchestration platform with a plugin model to configure and observe security tooling (e.g., nftables, Coraza WAF, NGINX, Apache httpd, AppArmor, ClamAV, Falco, Fluent Bit alternatives), and to debug configuration issues and policy decisions across the fleet.

This is a living document; we will iterate and converge on concrete designs and milestones.

## 1) Problem Statement

Build a platform that:

- Centrally manages configuration of diverse security components across Linux servers (firewalling, WAFs, HTTP servers/reverse proxies, LSM profiles, AV/IDS, logging pipelines).
- Provides a plugin system so new tools can be onboarded without core changes.
- Collects and analyzes logs/events to pinpoint configuration and runtime issues (e.g., AppArmor denies, WAF header/route blocks, required open ports for services).
- Operates with an extreme security posture: memory-safe implementation, strong isolation, encryption in transit and at rest, minimal attack surface, least privilege, auditable changes, and supply-chain integrity.
- Can run fully on a private LAN but supports exposure to the internet when absolutely needed, with robust defenses (mTLS, CSP, CSRF protection, RBAC/ABAC, etc.).
- Avoids reliance on external web servers like NGINX/Apache for core serving—uses an embedded server with modern TLS and strict hardening.

## 2) High-Level Goals and Non-Goals

Goals
- Security-first: default-deny posture, defense-in-depth, verifiable integrity.
- Generic core: reusable modules for authn/z, users/tenancy, logging, charting, network interface configs, policy staging/rollout/rollback.
- Plugin extensibility: stable interfaces, versioned contracts, safe sandboxing.
- Deterministic auditability: tamper-evident config and action logs.
- Operational simplicity: minimal dependencies, single static binaries where practical, reproducible builds.

Non-Goals (initially)
- Full SIEM replacement; focus is security-config orchestration + focused diagnostics.
- Rich multi-cloud discovery/inventory; start with explicit enrollment.
- Complex multi-master clustering; start with single controller + HA later.

## 3) Threat Model & Security Objectives

Assumptions
- Controller and managed servers are primarily on private networks; internet exposure may be required for certain deployments.
- Adversaries may gain footholds on some servers, attempt lateral movement, attempt to subvert configuration channels, and may try to inject malicious plugins or exploit the web UI/API.

Objectives
- Confidentiality and integrity of all control-plane traffic (config, commands, logs) via mTLS 1.3 with strong client certs and pinning.
- No password-based auth for agents; certificate-based identity (consider SPIFFE IDs) with short-lived cert rotation.
- Web UI/API: resist XSS, CSRF, SSRF, IDOR, SQLi, RCE. Strict CSP and templating. Parametrized queries only.
- Data at rest encrypted with authenticated encryption. Keys protected by OS KMS/TPM/HSM where available.
- Plugins sandboxed with strong isolation and least privilege (e.g., WASM/WASI w/ capability-based host functions). No direct network/file access unless explicitly granted per-plugin.
- Audit trail is tamper-evident (append-only log with content hashes/signatures; optionally anchored to an external transparency log).
- Supply-chain hardening: reproducible builds, dependency pinning, SLSA provenance, artifact signing, SBOMs.

Anti-Goals
- Relying on MAC address filtering for security. MAC is unreliable beyond L2 and easily spoofed; use cert-based identity + IP allowlisting as secondary defense if needed.

## 4) Architecture Options

Option A — Per-Host Full Web App
- Each managed server runs the full web app + UI.
- Pros: Local-only changes; minimal central blast radius.
- Cons: Operationally heavy, fragmented authn/z, harder to audit globally, larger attack surface per host.

Option B — Central Controller + Lightweight Agents (Recommended)
- Central controller (UI/API) orchestrates; minimal agents run on each managed server.
- Agents pull desired state and push telemetry over an outbound mTLS connection to the controller (pull-based model minimizes inbound exposure on agents).
- Plugins live in the controller (for plan/analysis) and/or as safe sandboxed code within agents for on-host application.
- Pros: Smaller per-host attack surface; consistent policy; easier audit; simple upgrades; network-friendly (only outbound agent->controller).
- Cons: Controller becomes critical; needs HA and careful hardening over time.

Option C — Controller with Remote Gateways per Site/Segment
- Same as B but with an additional site-gateway to aggregate agents.
- Pros: Scales network control, isolates failure domains.
- Cons: Extra moving parts.

Recommendation
- Start with Option B: Single controller + per-host agent with outbound-only mTLS.
- Add site gateways later for larger deployments.

## 5) Language and Runtime Choices

Candidates
- Rust: Memory safe, no GC, excellent async networking (hyper/axum), strong cryptography ecosystem (ring/rustls), good WASM support, minimal unsafe when curated. Steeper learning and compile times but best for extreme-hardening.
- Go: Memory safe (no raw pointers) but data races possible; simple concurrency; excellent stdlib; solid TLS; great developer velocity. Some libs use unsafe; avoid CGO/unsafe where possible. Good alternative if team prefers Go.
- C/C++: High performance but memory-unsafe by default; requires extreme discipline and still error-prone. Not recommended for the core.

Proposal
- Core controller, agent, and plugin sandbox host in Rust for maximum safety.
- If Go is preferred by the team, the design remains valid: use net/http with strict middlewares, crypto/tls, and gRPC with mTLS. We will enforce strict safe coding guidelines and avoid unsafe/CGO paths.

## 6) Embedded HTTP Server

Rust Stack (Recommended)
- HTTP: axum + hyper + tower.
- TLS: rustls (TLS 1.3 only; strong cipher suites; OCSP stapling; client cert auth support).
- HTTP/2 required; HTTP/1.1 optional; no TLS renegotiation; ALPN enforced.
- Security headers: HSTS, CSP (script-src 'self' with nonces, no inline), Referrer-Policy, X-Content-Type-Options, Permissions-Policy.

Go Stack (Alternative)
- net/http, crypto/tls (TLS 1.3), h2c/h2 configured strictly.
- Optional: reuse portions of Caddy libraries if embedding is acceptable; otherwise write strict handlers/middlewares.

Note on “security-focused httpd”
- Rather than embedding a large, general-purpose server, we build on small, well-audited libraries (hyper/rustls or net/http/crypto/tls) with minimal feature surface, strict defaults, and aggressive hardening.

## 7) Data Storage and Security

Primary Database: Hardened PostgreSQL
- TLS-only connections; verify server certs; client certs for admin/control-plane access when appropriate.
- Row-Level Security (RLS) for multi-tenant scenarios; `pgaudit` for auditing; logical decoding optional for external audit feeds.
- Encryption at rest via disk/LUKS or external KMS; application-level encryption for sensitive blobs when required.

Modeling Strategy
- Relational core for identities, roles, policies, agents, deployments, plugin registry, tasks, and audit trail.
- JSONB for plugin-specific configurations and outputs (plans, diagnostics) with versioned schemas and constraints.
- Use CHECK constraints, generated columns, and JSON Schema validation at the application level for invariants.

Audit Trail
- Append-only table with cryptographic hash chaining (hash(prev_hash || entry)) and signature of batches; periodic anchoring to external transparency log optional.

Agents
- Avoid persistent DB where possible; keep minimal state and ephemeral queues.

Key Management
- Offline root CA; intermediate issuing CA for controller and agents.
- mTLS with short-lived certs; automated rotation (consider SPIFFE/SPIRE for workload identities and issuance).
- Secrets sealed using OS KMS/TPM; in-memory decrypted only when necessary.

## 8) Plugin Model

Goals
- Add new tool integrations without changing core.
- Ensure plugins cannot compromise the controller/agent.

Approach: WASM/WASI Sandboxed Plugins
- Plugins compiled to WebAssembly (WASI) from Rust (preferred), TinyGo, AssemblyScript where appropriate.
- Capability-based host API: expose only specific operations per plugin type (e.g., render config, validate desired state, compute diffs, parse tool logs, produce hints). No filesystem or network unless explicitly granted per-plugin.
- Deterministic inputs/outputs via Protobuf/FlatBuffers or JSON schema with versioning.
- Execution limits: CPU time, memory quota, fuel metering, deadline enforcement.
- Signing and verification: plugins must be signed; controller verifies signature + version/policy before loading.

Plugin Categories
- Firewall (nftables/iptables wrappers).
- WAF (Coraza for ModSecurity-compat; NGINX/Apache WAF modules integration).
- HTTP servers (NGINX/Apache) config translators.
- LSM profiles (AppArmor, SELinux policies).
- AV/IDS (ClamAV, Falco) config and rule management.
- Observability adapters (parsers for tool logs; transformations) — note: collection is handled by the agent, not by external fluent-bit.

Lifecycle
1) Desired state: Controller receives declarative config (YAML/JSON/Protobuf) for a tool.
2) Plugin validates desired state, renders target config files, and outputs a plan (diff + apply steps + health checks).
3) Controller/Agent applies plan on target host with privilege separation (see §10) and captures results/logs.
4) Plugin analyzes logs and outputs diagnostics/hints.

## 9) Controller–Agent Protocol

Transport
- Agent establishes outbound-only mTLS 1.3 to the controller (HTTP/2). Certificates are mutually verified; server cert pinned to controller CA.
- Message layer integrity: in addition to TLS, optionally sign message envelopes (Ed25519) for defense-in-depth and audit replay detection.

API
- gRPC/Protobuf service definitions for:
  - Enrollment & attestation (TPM-based attestation optional).
  - Desired state fetch (pull model) + incremental updates with leases.
  - Apply plan with staged rollback and health probes.
  - Log/event streaming with backpressure and chunking.
  - Heartbeats/metrics.

Resilience
- Backoff and jitter; persistent queues on agent only if needed; idempotent operations.
- Strict timeouts; monotonic clocks; sequence numbers to prevent replays.

## 10) Privilege and Isolation on Managed Hosts

- Agent runs as non-root by default with Linux capabilities granted just-in-time via sub-processes (e.g., CAP_NET_ADMIN for nftables operations only within short-lived helpers).
- Use dedicated service accounts; chroot or user namespaces where possible.
- Apply seccomp-bpf, AppArmor/SELinux profiles to confine the agent and helpers.
- Configuration writes occur in a staging directory; atomic switch (symlink swap or tool-native transactional apply) with verification.

## 11) Web Application Security (Controller)

- Authentication: Prefer passwordless (WebAuthn/FIDO2) + optional mTLS for admin areas; support OIDC/SAML for enterprise.
- Authorization: RBAC with scoped roles; optional ABAC (tags like environment/site/tool).
- Input handling: Strong typing; no reflection-based deserialization; JSON schema validation; limit sizes and depths.
- SQL safety: Use compile-time-checked parameterized queries; no string concatenation.
- XSS/Clickjacking: Strict templating, output encoding, CSP (no inline), frame-ancestors 'none', sanitized markdown where used.
- CSRF: SameSite=strict cookies; anti-CSRF tokens for state-changing operations; double-submit cookies.
- Rate limiting and DoS controls per route and per identity.
- Audit logging: Every config change, login, token issuance, plugin load, plan apply.

## 12) Logging and Telemetry Pipeline

- Collection: Agent tails relevant logs (journal, tool-specific files, kernel/audit), parses via built-in readers (not Fluent Bit), and ships over the same mTLS channel.
- Privacy and security: Records are compressed and optionally encrypted at the message layer (AEAD) in addition to TLS; integrity signed. PII minimization by default.
- Storage: Controller writes logs to an append-only store with hash chaining; optional export to external SIEM.
- Diagnostics: Plugins receive parsed events to generate actionable hints (e.g., “AppArmor profile X denied path Y” → “add rule Z” or “open port 123/udp”).

## 13) Network Exposure and Access Controls

- Agents: no inbound listeners in the default design. Outbound-only to controller on a single port.
- Controller: listens on a dedicated port; restrict by IP allowlist as an extra layer. Mandatory mTLS for agent endpoints.
- MAC address allowlisting is not a reliable security control; rely on mTLS identities. MACs are not visible across L3 and are trivially spoofable on L2.

## 14) Supply Chain and Build Security

- Reproducible builds; dependency pinning with checksums; vendoring where appropriate.
- SLSA provenance on build artifacts; SBOM generation (CycloneDX/SPDX).
- Artifact signing (Sigstore/cosign) and verification at deploy.
- Continuous security testing: SAST (Rust: cargo audit, clippy; Go: govulncheck), DAST on API, fuzzing on parsing and plugin interfaces.

## 15) Operational Model

- Single-binary controller (Rust) + single-binary agents (Rust). Systemd units with hardened settings (ProtectSystem, PrivateTmp, NoNewPrivileges, CapabilityBoundingSet, AmbientCapabilities minimal).
- Backup/restore: encrypted snapshots of the controller DB and append-only audit log.
- Upgrades: signed binaries; blue/green or canary deploy; automatic agent upgrades via signed packages with staged rollout.
- **Primary production deployment target: K3s on Alpine 3.22** with a hybrid multi-cluster topology (Core, DMZ, ETL/Data, Monitoring/Obs clusters). Headlamp provides per-cluster local management UI; Rancher serves as fleet manager across clusters. See [K3s Infrastructure Spec](docs/deploy/k3s_infrastructure_spec.md) for full details.
- The single-binary/systemd model remains valid for edge and minimal deployments where Kubernetes is not warranted.

## 16) Initial Deliverables and Milestones

Phase 1 — Foundations
- Controller scaffold with axum/hyper/rustls, mTLS, basic RBAC, SQLCipher-backed config store, and tamper-evident audit log.
- Agent scaffold with outbound mTLS, enrollment, heartbeat, and log streaming of a small set (e.g., journald).
- Plugin runtime (wasmtime) with a sample plugin (e.g., parse AppArmor denies and suggest rules).

Phase 2 — First Integrations
- nftables plugin: desired state → nft rule sets; plan/apply/rollback; health probes.
- Coraza WAF plugin: header/route policy suggestions based on observed blocks; config rendering for NGINX/Apache.
- AppArmor plugin: profile rendering and incremental adjustments.

Phase 3 — Hardening & Scale
- SPIFFE/SPIRE integration for cert issuance; TPM-backed keys on agents.
- Site-gateway option; HA controller; Postgres option.
- Extended diagnostics playbooks; richer charting; more plugins (ClamAV, Falco).

## 17) Open Questions / Decisions to Make

- Primary implementation language: Rust (recommended) vs Go (team preference?).
- DB default: SQLCipher vs sled vs hardened Postgres.
- Plugin sandbox: WASM/WASI (recommended) — any need for separate process-level plugin option?
- Attestation: SPIFFE/SPIRE and TPM 2.0 required or optional?
- Protocol: gRPC/Protobuf over mTLS vs custom HTTP/JSON for early phases.
- Exposure model: controller internet-facing ever, or strictly behind VPN/priv network?

## 18) Notes on “Hack-Proof” Aspirations

No system is provably hack-proof. We will:
- Minimize the attack surface (small binaries, few deps, no dynamic modules in core).
- Use memory-safe languages and sandbox plugins.
- Enforce strong identities, least privilege, and short-lived creds.
- Layer defenses (TLS + message signing + audit chaining + secure storage).
- Invest in continuous verification (fuzzing, SAST/DAST, code review, threat modeling).

## 19) Immediate Next Steps (for discussion)

- Confirm language choice (Rust vs Go) and approve the controller/agent split with outbound-only agents.
- Lock runtime stack for the controller (axum/hyper/rustls) and DB (SQLCipher) for Phase 1.
- Define the v0 plugin ABI (WASM host calls, schemas for inputs/outputs) and produce the first sample plugin spec.
- Draft the gRPC service definitions for enrollment, desired state, and log streaming.

---

Appendix A — Why not rely on Fluent Bit?
- Fluent Bit is excellent but introduces an extra component and identity layer. We want message-layer identity and integrity tied directly to the SWAP agent and controller with a unified mTLS and policy model, plus tighter plugin-driven parsing/diagnostics.

Appendix B — Why not filter by MAC?
- MACs are L2-only and trivial to spoof; not visible across routers. Use mTLS identities pinned to a CA and optionally IP allowlists.

Appendix C — Example Cipher/Protocol Defaults
- TLS 1.3 only; disable 1.0–1.2.
- Ciphers: TLS_AES_256_GCM_SHA384 preferred; TLS_CHACHA20_POLY1305_SHA256 allowed.
- ALPN: h2; optionally http/1.1 if needed.
- OCSP stapling enabled; certificate revocation via CRL/OCSP.
