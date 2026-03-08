# Project Roadmap

This document sequences implementation work into reviewable milestones with clear acceptance criteria and links to the relevant SPECs and PLANs. Use alongside `docs/INDEX.md`.

## Phase 0 — Baseline Approvals and Repo Hygiene
- SPEC-first PRs reviewed/merged; CI SPEC check enabled; branch protections in place.
- Acceptance: PR template used; SPEC check green by default.

## Phase 1 — Core Security Foundations

1. Auth Store (encrypted, standalone; default admin)
   - SPEC: docs/security/auth_store_spec.md (PR #43)
   - PLAN: docs/implementation/auth_store_scaffolding_plan.md (PR #45)
   - Deliverables:
     - `auth_store` crate (Argon2id + AEAD; KMS/TPM optional)
     - Controller bootstrap: default admin, one-time password, must-change enforcement
     - Lockout + audit of auth events
   - Acceptance: Unit/integration tests pass; one-time password process verified; audit entries created.

2. SSR Routes + Sessions/CSRF (static slugs)
   - SPEC: docs/security/session_csrf_spec.md (PR #34); docs/ui/framework_style_spec.md (PR #11); docs/security/routing_obfuscation_spec.md (PR #20)
   - PLAN: docs/implementation/ssr_routes_csrf_plan.md (PR #46)
   - Deliverables:
     - SSR-only sensitive routes bound to install-time slugs
     - Session + CSRF middleware; health restricted
   - Acceptance: Auth→home flow with CSRF; uniform 404s; tests for session/CSRF.

3. Event Ingestion + Live SSE Tail
   - SPEC: docs/logging/streaming_classification_spec.md (PR #12); docs/logging/live_transport_spec.md (PR #36)
   - PLAN: docs/implementation/event_ingestion_sse_plan.md (PR #47)
   - Deliverables:
     - gRPC log ingestion with normalization and indices
     - SSE live tail endpoint with filters; reconnection + heartbeats
   - Acceptance: Events persisted; SSE filters + last-event-id; rate limits applied.

## Phase 2 — UI/UX and Plugin MVPs

4. Log UI + Graphs (MVP)
   - SPEC: docs/logging/ui_spec.md (PR #13); docs/logging/graphing_spec.md (PR #14)
   - Deliverables: Live tail view, search/filter UI; basic severity/source charts.
   - Acceptance: Bookmarkable filters; chart renders for selected ranges.

5. Plugin Registry + Schema Validation
   - SPEC: docs/plugins/plugin_install_registry_ui_spec.md (PR #16); docs/plugins/schema_registry_spec.md (PR #37)
   - Deliverables: Signed plugin install/update/remove; schema validation on submit/apply.
   - Acceptance: Verified install with capability review; invalid configs rejected with clear errors.

6. Module UIs (first set)
   - SPEC: nftables (PR #17), AppArmor (PR #19), Coraza (PR #18)
   - Deliverables: Config editors with validation/diff/apply; logs/hints views.
   - Acceptance: Safe-apply + rollback; hints visible and actioned.

## Phase 3 — Operations, Hardening, and Defaults

7. Rate Limits and Backpressure Defaults
   - SPEC: docs/security/default_rate_limits_spec.md (PR #48)
   - Deliverables: Configurable defaults; metrics + alerts on hits; backpressure behavior.
   - Acceptance: Limits enforced in dev/stage; dashboard shows rejects.

8. Observability
   - SPEC: docs/ops/observability_metrics_spec.md (PR #27)
   - Deliverables: Prometheus metrics; obfuscated health/readiness; structured logs.
   - Acceptance: Metrics scraped; alerts wired for SLOs.

9. Node-less Build Profile
   - SPEC: docs/ui/node_less_build_spec.md (PR #49)
   - Deliverables: Build flag producing SSR-only assets with vendored CSS/JS.
   - Acceptance: Image builds without Node; SSR pages render; CSP passes.

## Phase 4 — Supply Chain and Upgrades

10. Supply Chain Security
    - SPEC: docs/ops/supply_chain_release_spec.md (PR #28)
    - Deliverables: SBOM, SLSA provenance, cosign signing + verification.
    - Acceptance: Signed artifacts verifiable in CI/deploy.

11. Upgrades (Controller/Agents/Plugins)
    - SPEC: docs/ops/upgrades_spec.md (PR #29)
    - Deliverables: Blue/green controller; agent canary; plugin pins/rollback.
    - Acceptance: Staged rollouts with rollback on failure.

## Phase 5 — K3s Infrastructure Migration

12. Alpine 3.22 Image Build
    - SPEC: docs/deploy/k3s_infrastructure_spec.md
    - Deliverables: Multi-stage Dockerfile (`rust:1-bookworm` builder, `alpine:3.22` runtime); musl static binary; OCI image signing.
    - Acceptance: Image builds in CI; cosign-verified; passes Trivy scan.

13. Core Cluster Helm Chart + K3s Deployment
    - SPEC: docs/deploy/k3s_infrastructure_spec.md
    - Deliverables: `swap-core` Helm chart (Controller, PostgreSQL, Auth Store, Plugin Registry); Headlamp; embedded etcd HA; `--secrets-encryption`.
    - Acceptance: Controller deploys on K3s; gRPC/mTLS functional; agents connect.

14. Multi-Cluster Topology (DMZ, ETL, Monitoring)
    - SPEC: docs/deploy/k3s_infrastructure_spec.md; docs/network/site_gateway_spec.md (DMZ cluster supersedes site gateway for K3s deployments)
    - Deliverables: `swap-dmz` (Traefik + Coraza WAF), `swap-etl` (ingestion workers), `swap-monitoring` (Prometheus federation, Grafana, Alertmanager, Loki) Helm charts.
    - Acceptance: Inter-cluster mTLS validated; NetworkPolicies enforce default-deny; Prometheus federation operational.

15. Rancher Fleet Management
    - Deliverables: Rancher Fleet GitOps config; cluster registration; coordinated rollouts.
    - Acceptance: Fleet syncs all charts from Git; staged rollouts across clusters.

## Cross-Cutting Security
- Route obfuscation (static slugs) — PR #20
- Honey endpoints (decoy) — PR #31
- Secrets management — PR #38
- PKI and enrollment — PR #21
- RBAC matrix — PR #42
- Audit chain — PR #41

## Dependencies
- Phase 1 must be completed before Phase 2.
- Rate limits (Phase 3) depend on Phase 1 SSE endpoint.
- Node-less build can proceed after SSR routes are in place.
- Phase 5 (K3s migration) can proceed in parallel with Phases 2-4; requires Phase 1 controller binary to be functional.
- Alpine image build (Phase 5, item 12) is a prerequisite for all K3s cluster deployments.

## Status Tracking
- Use PR numbers in this roadmap for review ordering.
- Ensure PR bodies follow SPEC template with diagrams and acceptance criteria.

