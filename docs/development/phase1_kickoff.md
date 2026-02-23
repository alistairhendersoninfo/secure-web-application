# Phase 1 Development Kickoff — Auth Store, SSR Slugs/CSRF, Events SSE

## Goals
- Begin Phase 1 implementation with minimal, reviewable slices.

## Scope (Phase 1)
- Encrypted Auth Store (standalone; default admin bootstrap)
- SSR routes bound to static slugs + Sessions/CSRF middleware
- Event ingestion + SSE live tail endpoint

## Architecture Overview
```mermaid
flowchart LR
  subgraph Controller
    API[gRPC/HTTP]
    AUTH[Auth Store]
    SSE[SSE Tail]
  end
  Agent -->|Logs (gRPC)| API
  Browser -->|SSR (slugs) + CSRF| API
  API --> AUTH
  API --> SSE
```

## Deliverables
- Auth store crate + bootstrap flow (one-time default admin)
- SSR auth/home routes using configured slugs; session+CSRF middleware
- gRPC ingestion path; SSE endpoint with filters + reconnection

## Acceptance Criteria
- Auth: default admin printed once; must-change enforced; audit auth events
- SSR: login->home flow uses slugs; uniform 404s; CSRF validated
- SSE: ingest normalized events; SSE tail streams filtered events; last-event-id supported

## Non-Goals
- UI polishing; production theming; multi-tenant RLS enforcement

## Risks & Mitigations
- Config mistakes on slugs: strict config validation + startup logs
- Backpressure: per-connection rate limits; drop-oldest + metrics
