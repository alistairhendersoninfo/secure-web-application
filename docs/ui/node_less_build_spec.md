# SPEC: Node-less Build Profile (Pure SSR Asset Pipeline)

## Goals
- Provide a build profile that avoids Node tooling for restricted environments.

## Non-Goals
- Feature parity with SPA; focus on SSR-only admin flows.

## Architecture Overview
- SSR templates (e.g., Askama/maud or minimal HTML rendering) + htmx for interactivity.
- CSS: prebuilt minimal CSS (system fonts + small utility classes) vendored; no Tailwind build step.
- JS: minimal inline modules with CSP nonces or external small JS vendored; avoid bundlers.

```mermaid
flowchart LR
  Templates[SSR Templates] --> HTML[Rendered HTML]
  CSS[Prebuilt CSS] --> HTML
  JS[Minimal JS (vendored)] --> HTML
```

## Detailed Design
- Templates: Rust SSR (Askama/Maude-like) with strict escaping; no client-side routing.
- Styles: vendored CSS file (~10–20KB) providing layout/grid/spacing; dark/light via CSS variables.
- Scripts: vendored htmx (optional) with SRI hash and CSP nonce; or no JS where feasible.
- Asset hashing: done by Rust build or simple hash-on-copy script; no Node bundlers.

## Security Posture
- Strict CSP with nonces; no remote scripts; minimal JS surface.

## Operations
- Profile selection via build flag: `SSR_NODELESS=1` produces image with prebuilt CSS/JS.

## Acceptance Criteria
- Documented build steps for Node-less profile; SSR pages render with vendored CSS/JS; no Node required to build.
