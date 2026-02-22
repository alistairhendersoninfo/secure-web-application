# Spec Index and Review Map

Use this index to navigate all specs and plans without relying on chat history. Each item links to the local spec file (for LLMs) and notes the associated GitHub PR for review/merge context.

## Roadmap
- docs/ROADMAP.md — Milestones, dependencies, acceptance criteria (start here)

## Core Architecture
- Protocol/ABI/Events: docs/protocol.md (PR #5)
- Controller gRPC + migrations: docs/protocol.md and crates/controller (PR #6)
- Single image deploy: docs/deploy/single_image_spec.md (PR #44)

## Security Policies
- Route obfuscation (static slugs): docs/security/routing_obfuscation_spec.md (PR #20)
- Sessions + CSRF (SSR): docs/security/session_csrf_spec.md (PR #34)
- Error model, rate limiting, DoS: docs/security/error_rate_limit_spec.md (PR #26)
- Default rate limits and backpressure: docs/security/default_rate_limits_spec.md (PR #48)
- PKI & enrollment: docs/security/pki_enrollment_spec.md (PR #21)
- Secrets management (KMS/Vault): docs/security/secrets_management_spec.md (PR #38)
- Honey endpoints (decoy): docs/security/honey_endpoints_alerting_spec.md (PR #31)
- RBAC (roles/actions): docs/security/rbac_matrix_spec.md (PR #42)

## Auth
- Internal encrypted auth store: docs/security/auth_store_spec.md (PR #43)

## UI/UX
- UI shell (SSR-first + obfuscated routes): docs/ui/framework_style_spec.md (PR #11)
- Navigation with slugs: docs/ui/navigation_obfuscation_spec.md (PR #35)
- Node-less build profile (pure SSR): docs/ui/node_less_build_spec.md (PR #49)

## Logging and Observability
- Log streaming/classification: docs/logging/streaming_classification_spec.md (PR #12)
- Live log transport (SSE/WS): docs/logging/live_transport_spec.md (PR #36)
- Log UI (live tail/search/drilldown): docs/logging/ui_spec.md (PR #13)
- Graphing (aggregation): docs/logging/graphing_spec.md (PR #14)
- Diagnostics schemas + hints: docs/logging/diagnostics_hints_spec.md (PR #25)
- Observability (metrics/health/tracing): docs/ops/observability_metrics_spec.md (PR #27)
- Data retention/partitioning/archival: docs/data/retention_partitioning_spec.md (PR #33)

## Plugins
- Packaging/signing/registry trust: docs/plugins/packaging_signing_spec.md (PR #24)
- Schema registry & validation: docs/plugins/schema_registry_spec.md (PR #37)
- Registry/install UI: docs/plugins/plugin_install_registry_ui_spec.md (PR #16)
- UIs: nftables (PR #17), Coraza (PR #18), AppArmor (PR #19), ClamAV (PR #39), Falco (PR #40)

## Agent & Network
- Agent isolation/apply: docs/agent/isolation_apply_spec.md (PR #23)
- Optional site gateway: docs/network/site_gateway_spec.md (PR #30)

## Supply Chain & Ops
- Supply chain & release: docs/ops/supply_chain_release_spec.md (PR #28)
- Upgrades (controller/agent/plugins): docs/ops/upgrades_spec.md (PR #29)
- Operator runbooks (certs/keys/CA): docs/ops/runbooks_rotation_spec.md (PR #32)

## Implementation Plans
- Auth store scaffolding: docs/implementation/auth_store_scaffolding_plan.md (PR #45)
- SSR routes + CSRF/session: docs/implementation/ssr_routes_csrf_plan.md (PR #46)
- Event ingestion + SSE tail: docs/implementation/event_ingestion_sse_plan.md (PR #47)

## Review Order (Recommended)
1. Auth store (PR #43), RBAC (PR #42)
2. Sessions/CSRF (PR #34), UI shell SSR (PR #11)
3. Log pipeline (PR #12), Log UI (PR #13)
4. Implementation plans (PR #45, #46, #47)
5. Rate limits (PR #48), Node-less profile (PR #49)

## Process Guards
- SPEC-first PR enforcement: .github/workflows/pr-spec-check.yml, .github/PULL_REQUEST_TEMPLATE.md
- Contributor guidance: CONTRIBUTING.md, AGENTS.md, docs/SPEC_STANDARDS.md
