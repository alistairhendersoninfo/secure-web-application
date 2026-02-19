# SPEC: UI Framework, Styling, and Shell

## Goals
- Define front-end stack and styling conventions for a secure, operable admin UI.
- Balance developer velocity with supply-chain and runtime risk.
- Enable accessible, consistent components with minimal bespoke CSS.

## Non-Goals
- Implementing the UI; this PR sets standards and system design only.
- Choosing a charting lib (see Graphing SPEC).

## Architecture Overview
- Server-side: Rust (axum) serves API and static assets.
- Front-end options:
  - Option A: SSR-first with minimal JS (htmx + Alpine.js) and Tailwind core.
  - Option B: React + Tailwind + shadcn/ui (Radix primitives) for rich components.
- We harden build pipeline and restrict third-party scripts via strict CSP (nonces, no inline eval).

```mermaid
flowchart LR
  subgraph Browser
    UI[SPA/SSR UI]
  end
  subgraph Controller
    API[gRPC/HTTP API]
    Assets[Static Assets]
  end
  UI -->|HTTPS| API
  UI <-->|HTTPS| Assets
```

## Detailed Design
- Recommended default: Option B (React + Tailwind + shadcn/ui) for component completeness, if the team accepts Node-based build tooling under tightened supply-chain controls.
- Alternate hardened mode: Option A for environments that forbid Node tooling; SSR HTML with htmx for interactivity and Tailwind core for styling.
- Common guidelines:
  - CSP: `default-src 'self'; script-src 'self' 'nonce-<rand>'; object-src 'none'; frame-ancestors 'none'`. No third-party JS/CDN.
  - No client-side secrets; all secrets remain server-side.
  - Components must be keyboard-accessible; color contrast checks.
  - Theming via CSS variables and Tailwind tokens.
  - Avoid client-side routing for critical admin flows; prefer server validation.

## Security Posture
- Strict CSP; no inline scripts without nonces; no remote scripts.
- Integrity-checked assets; asset hashing; subresource integrity when feasible.
- Build pipeline pinned, SBOM generated; dependency review and allowlist.

## Operations
- Build artifacts are baked into the controller image (immutable).
- Feature flags for Option A vs Option B builds.

## Acceptance Criteria
- Decision recorded: default to React + Tailwind + shadcn/ui for admin UX; provide SSR/htmx fallback profile.
- CSP policy defined and enforced across pages.
- Accessibility checklist documented for components.

## Open Questions
- Do we require a no-Node build path for classified environments by default?
- Which UI areas must remain SSR-only (e.g., login, enrollment)?
